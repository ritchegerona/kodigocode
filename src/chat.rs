use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Clear, Paragraph},
    Terminal,
};
use tokio_stream::StreamExt;
use std::io;
#[path = "ai/mod.rs"]
mod ai;

const TEAL: Color = Color::from_u32(0x001a2e3b);
const TEAL_BRIGHT: Color = Color::from_u32(0x0000bfa5);
const TEAL_DIM: Color = Color::from_u32(0x00004d40);
const BG: Color = Color::from_u32(0x00122025);
const FG: Color = Color::from_u32(0x00e0e0e0);
const ACCENT: Color = Color::from_u32(0x0026a69a);
const DIM: Color = Color::from_u32(0x005a7a7a);

pub struct ChatApp {
    pub messages: Vec<ChatMsg>,
    pub input: tui_textarea::TextArea<'static>,
    pub provider: String,
    pub model: String,
    pub providers: Vec<ProviderEntry>,
    pub scroll: usize,
    pub streaming: bool,
    pub palette: bool,
    pub palette_filter: String,
    pub palette_idx: usize,
}

pub struct ChatMsg { pub role: String, pub content: String }
pub struct ProviderEntry { pub name: String, pub models: Vec<String> }

impl ChatApp {
    pub fn new(provider: String, model: String) -> Self {
        let mut ta = tui_textarea::TextArea::default();
        ta.set_placeholder_text("Ask anything...");
        ta.set_style(Style::default().bg(TEAL).fg(FG));

        let providers = vec![
            ProviderEntry { name: "openclaude".into(), models: vec!["claude-sonnet-4-20250514".into(), "claude-opus-4-20250514".into(), "claude-haiku-3-20250218".into()] },
            ProviderEntry { name: "openai".into(),     models: vec!["gpt-4o".into(), "gpt-4-turbo".into()] },
            ProviderEntry { name: "deepseek".into(),   models: vec!["deepseek-v3".into(), "deepseek-r1".into()] },
            ProviderEntry { name: "vertex".into(),     models: vec!["gemini-2.5-pro".into(), "gemini-2.5-flash".into()] },
            ProviderEntry { name: "nvidia".into(),     models: vec!["meta/llama3-70b-instruct".into(), "meta/llama3-8b-instruct".into()] },
        ];

        Self {
            messages: vec![], input: ta, provider, model, providers,
            scroll: 0, streaming: false,
            palette: false, palette_filter: String::new(), palette_idx: 0,
        }
    }

    pub fn select_provider(&mut self, idx: usize) {
        if let Some(entry) = self.providers.get(idx) {
            self.provider = entry.name.clone();
            self.model = entry.models.first().cloned().unwrap_or_default();
        }
    }
}

pub async fn run_chat(provider: String, model: String, _registry: &crate::tool::ToolRegistry) -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = ratatui::backend::CrosstermBackend::new(stdout);
    let mut terminal = Terminal::<ratatui::backend::CrosstermBackend<io::Stdout>>::new(backend)?;
    let mut app = ChatApp::new(provider, model);
    let res = run_app(&mut terminal, &mut app).await;
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    if let Err(err) = res { eprintln!("{}", err); }
    Ok(())
}

async fn run_app(
    terminal: &mut Terminal<ratatui::backend::CrosstermBackend<io::Stdout>>,
    app: &mut ChatApp,
) -> Result<()> {
    loop {
        terminal.draw(|f| ui(f, app))?;

        let ev = event::read()?;
        match ev {
            Event::Key(key) if key.kind == KeyEventKind::Press => match key.code {
                KeyCode::Esc => {
                    if app.palette { app.palette = false; app.palette_filter.clear(); app.palette_idx = 0; }
                    else { return Ok(()); }
                }
                KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => return Ok(()),
                KeyCode::PageUp => app.scroll = app.scroll.saturating_add(5),
                KeyCode::PageDown => app.scroll = app.scroll.saturating_sub(5),

                KeyCode::Char('/') if !app.palette => {
                    app.palette = true; app.palette_filter = "/".into(); app.palette_idx = 0;
                }

                KeyCode::Enter => {
                    if app.palette {
                        let entries = matched_entries(app);
                        if let Some(name) = entries.get(app.palette_idx) {
                            if let Some(idx) = app.providers.iter().position(|p| &p.name == name) {
                                app.select_provider(idx);
                                app.messages.push(ChatMsg { role: "assistant".into(), content: format!("Switched to provider: {}", name) });
                            }
                        }
                        app.palette = false; app.palette_filter.clear(); app.palette_idx = 0;
                    } else {
                        handle_send(terminal, app).await;
                    }
                }

                KeyCode::Backspace if app.palette => {
                    if app.palette_filter.len() > 1 { app.palette_filter.pop(); app.palette_idx = 0; }
                }
                KeyCode::Char(ch) if app.palette => {
                    app.palette_filter.push(ch); app.palette_idx = 0;
                }
                KeyCode::Up | KeyCode::Char('k') if app.palette => {
                    app.palette_idx = app.palette_idx.saturating_sub(1);
                }
                KeyCode::Down | KeyCode::Char('j') if app.palette => {
                    let max = matched_entries(app).len();
                    if max > 0 && app.palette_idx + 1 < max { app.palette_idx += 1; }
                }
                _ => { if !app.palette { app.input.input(ev); } }
            },
            _ => {}
        }
    }
}

fn matched_entries(app: &ChatApp) -> Vec<String> {
    let filter = app.palette_filter.trim_start_matches('/').to_lowercase();
    if filter.is_empty() {
        app.providers.iter().map(|p| p.name.clone()).collect()
    } else {
        app.providers.iter()
            .filter(|p| p.name.to_lowercase().contains(&filter))
            .map(|p| p.name.clone())
            .collect()
    }
}

async fn handle_send(terminal: &mut Terminal<ratatui::backend::CrosstermBackend<io::Stdout>>, app: &mut ChatApp) {
    let content = app.input.lines().join("\n").trim().to_string();
    if content.is_empty() { return; }

    app.scroll = 0;
    app.messages.push(ChatMsg { role: "user".into(), content: content.clone() });

    let api_key = std::env::var("API_KEY")
        .or_else(|_| std::env::var("OPENAI_API_KEY"))
        .or_else(|_| std::env::var("CLAUDE_API_KEY")).unwrap_or_default();

    if api_key.is_empty() {
        app.messages.push(ChatMsg { role: "assistant".into(), content: "No API key set (API_KEY, OPENAI_API_KEY, or CLAUDE_API_KEY)".into() });
    } else {
        match ai::make_provider(&app.provider, &api_key) {
            Ok(provider) => {
                let ai_msgs: Vec<ai::AiMessage> = app.messages.iter()
                    .map(|m| ai::AiMessage { role: m.role.clone(), content: m.content.clone() }).collect();
                match provider.stream_chat(&app.model, &ai_msgs, None, &[]).await {
                    Ok(mut stream) => {
                        app.streaming = true;
                        let mut buf = String::new();
                        app.messages.push(ChatMsg { role: "assistant".into(), content: String::new() });
                        loop {
                            let delta = stream.next().await;
                            let mut done = false;
                            match delta {
                                Some(Ok(ai::StreamDelta::Text(t))) => buf.push_str(&t),
                                Some(Ok(ai::StreamDelta::Stop)) | None => done = true,
                                Some(Ok(_)) => {}
                                Some(Err(e)) => { buf = format!("Stream error: {}", e); done = true; }
                            }
                            if let Some(m) = app.messages.last_mut() {
                                m.content = if buf.is_empty() && !done { "● ...".into() } else { buf.clone() };
                            }
                            let _ = terminal.draw(|f| ui(f, app));
                            if done { break; }
                        }
                        if let Some(m) = app.messages.last_mut() {
                            m.content = if buf.is_empty() { "(no response)".into() } else { buf };
                        }
                        app.streaming = false;
                    }
                    Err(e) => app.messages.push(ChatMsg { role: "assistant".into(), content: format!("Provider error: {}", e) }),
                }
            }
            Err(e) => app.messages.push(ChatMsg { role: "assistant".into(), content: format!("Provider error: {}", e) }),
        }
    }

    let mut ta = tui_textarea::TextArea::default();
    ta.set_placeholder_text("Ask anything...");
    ta.set_style(Style::default().bg(TEAL).fg(FG));
    app.input = ta;
}

fn ui(f: &mut ratatui::Frame, app: &mut ChatApp) {
    let area = f.area();

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(1),
            Constraint::Length(1),
            Constraint::Length(3),
        ])
        .split(area);

    render_messages(f, app, layout[0]);
    render_status(f, app, layout[1]);
    render_input(f, app, layout[2]);

    if app.palette {
        render_palette(f, app, layout[0]);
    }
}

fn render_messages(f: &mut ratatui::Frame, app: &ChatApp, area: Rect) {
    let mut lines: Vec<Line> = vec![];

    if app.messages.is_empty() {
        lines.push(Line::from(""));
        lines.push(Line::from(Span::styled(
            "  Welcome — type / to see commands",
            Style::default().fg(DIM),
        )));
        lines.push(Line::from(""));
        lines.push(Line::from(vec![
            Span::styled("  /", Style::default().fg(DIM)),
            Span::styled("provider", Style::default().fg(TEAL_BRIGHT).add_modifier(Modifier::BOLD)),
            Span::styled("  Switch AI provider", Style::default().fg(DIM)),
        ]));
        lines.push(Line::from(vec![
            Span::styled("  /", Style::default().fg(DIM)),
            Span::styled("model", Style::default().fg(TEAL_BRIGHT).add_modifier(Modifier::BOLD)),
            Span::styled("     Switch model", Style::default().fg(DIM)),
        ]));
        lines.push(Line::from(""));
        lines.push(Line::from(Span::styled(
            "  Enter your message to begin.",
            Style::default().fg(DIM),
        )));
    } else {
        let skip = app.scroll.min(app.messages.len().saturating_sub(1));
        let visible: Vec<&ChatMsg> = app.messages.iter().skip(skip).collect();
        let mut first = true;

        for msg in visible {
            if msg.role == "user" {
                if !first { lines.push(Line::from("")); }
                lines.push(Line::from(vec![
                    Span::styled("▸", Style::default().fg(ACCENT).add_modifier(Modifier::BOLD)),
                    Span::styled(format!(" {}", msg.content), Style::default().fg(FG)),
                ]));
            } else {
                if msg.content.is_empty() || msg.content == "● ..." {
                    lines.push(Line::from(Span::styled(
                        "● ...",
                        Style::default().fg(TEAL_DIM).italic(),
                    )));
                } else {
                    for ln in msg.content.lines() {
                        if ln.is_empty() {
                            lines.push(Line::from(""));
                        } else {
                            lines.push(Line::from(Span::styled(
                                ln.to_string(),
                                Style::default().fg(FG),
                            )));
                        }
                    }
                }
                lines.push(Line::from(""));
            }
            first = false;
        }
    }

    let p = Paragraph::new(Text::from(lines))
        .style(Style::default().bg(BG).fg(FG))
        .scroll((0, 0));
    f.render_widget(p, area);
}

fn render_status(f: &mut ratatui::Frame, app: &ChatApp, area: Rect) {
    let text = format!(
        " {}  │  {} msgs  {}",
        app.provider,
        app.messages.len(),
        if app.streaming { "● streaming" } else { "" }
    );
    let p = Paragraph::new(Line::from(Span::styled(text, Style::default().fg(DIM))))
        .style(Style::default().bg(TEAL));
    f.render_widget(p, area);
}

fn render_input(f: &mut ratatui::Frame, app: &mut ChatApp, area: Rect) {
    let input_block = Block::default()
        .borders(Borders::TOP)
        .border_style(Style::default().fg(TEAL_DIM));
    app.input.set_block(input_block);
    app.input.set_style(Style::default().bg(TEAL).fg(FG));
    app.input.set_cursor_style(Style::default().fg(TEAL_BRIGHT));
    f.render_widget(&app.input, area);
}

fn render_palette(f: &mut ratatui::Frame, app: &ChatApp, parent: Rect) {
    let entries = matched_entries(app);
    if entries.is_empty() { return; }

    let height = (entries.len() as u16).min(10) + 2;
    let width = 36u16;
    let x = 2u16;
    let y = if parent.height > height + 3 { parent.y + 2 } else { parent.bottom().saturating_sub(height) };

    let area = Rect { x, y, width, height };

    let mut lines: Vec<Line> = vec![];
    lines.push(Line::from(Span::styled(
        format!(" {}", app.palette_filter),
        Style::default().fg(TEAL_BRIGHT).add_modifier(Modifier::BOLD),
    )));
    lines.push(Line::from(Span::styled(
        "─".repeat(width as usize - 2),
        Style::default().fg(TEAL_DIM),
    )));

    for (i, name) in entries.iter().enumerate() {
        let selected = i == app.palette_idx;
        let style = if selected {
            Style::default().fg(TEAL_BRIGHT).bg(ACCENT)
        } else {
            Style::default().fg(FG).bg(TEAL)
        };
        let prefix = if selected { "▸ " } else { "  " };
        lines.push(Line::from(Span::styled(
            format!("{}{}", prefix, name),
            style,
        )));
    }

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(TEAL_BRIGHT))
        .style(Style::default().bg(TEAL));

    f.render_widget(Clear, area);
    f.render_widget(Paragraph::new(Text::from(lines)).block(block), area);
}