use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Paragraph},
    Terminal,
};
use tokio_stream::StreamExt;
use crate::tool;
use std::io;
#[path = "ai/mod.rs"]
mod ai;

const TEAL: Color = Color::from_u32(0x001a2e3b);
const TEAL_BRIGHT: Color = Color::from_u32(0x0000bfa5);
const TEAL_DIM: Color = Color::from_u32(0x00004d40);
const BG: Color = Color::from_u32(0x00122025);
const FG: Color = Color::from_u32(0x00e0e0e0);
const ACCENT: Color = Color::from_u32(0x0026a69a);

pub struct ChatApp {
    pub messages: Vec<ChatMsg>,
    pub input: tui_textarea::TextArea<'static>,
    pub provider: String,
    pub model: String,
    pub show_menu: bool,
    pub providers: Vec<ProviderEntry>,
    pub cursor_idx: usize,
    pub scroll: usize,
    pub streaming: bool,
}

pub struct ChatMsg {
    pub role: String,
    pub content: String,
}

pub struct ProviderEntry {
    pub name: String,
    pub models: Vec<String>,
}

impl ChatApp {
    pub fn new(provider: String, model: String) -> Self {
        let mut ta = tui_textarea::TextArea::default();
        ta.set_placeholder_text("Ask anything... (Esc to quit)");
        ta.set_style(Style::default().bg(TEAL).fg(FG));

        let providers = vec![
            ProviderEntry {
                name: "openclaude".into(),
                models: vec![
                    "claude-sonnet-4-20250514".into(),
                    "claude-opus-4-20250514".into(),
                    "claude-haiku-3-20250218".into(),
                ],
            },
            ProviderEntry {
                name: "openai".into(),
                models: vec!["gpt-4o".into(), "gpt-4-turbo".into()],
            },
            ProviderEntry {
                name: "deepseek".into(),
                models: vec!["deepseek-v3".into(), "deepseek-r1".into()],
            },
            ProviderEntry {
                name: "vertex".into(),
                models: vec!["gemini-2.5-pro".into(), "gemini-2.5-flash".into()],
            },
            ProviderEntry {
                name: "nvidia".into(),
                models: vec!["meta/llama3-70b-instruct".into(), "meta/llama3-8b-instruct".into()],
            },
        ];

        Self {
            messages: vec![],
            input: ta,
            provider,
            model,
            show_menu: false,
            providers,
            cursor_idx: 0,
            scroll: 0,
            streaming: false,
        }
    }

    pub fn status_text(&self) -> String {
        format!(
            " {} :: {}  |  {} messages ",
            self.provider,
            self.model,
            self.messages.len()
        )
    }

    pub fn select_provider(&mut self, idx: usize) {
        if let Some(entry) = self.providers.get(idx) {
            self.provider = entry.name.clone();
            self.model = entry.models.first().cloned().unwrap_or_default();
        }
    }
}

pub async fn run_chat(provider: String, model: String, _registry: &tool::ToolRegistry) -> Result<()> {
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

    if let Err(err) = res {
        eprintln!("{}", err);
    }
    Ok(())
}

async fn run_app(
    terminal: &mut Terminal<ratatui::backend::CrosstermBackend<io::Stdout>>,
    app: &mut ChatApp,
) -> Result<()> {
    loop {
        terminal.draw(|f| ui(f, app))?;

        if app.show_menu {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Esc => {
                            app.show_menu = false;
                            app.cursor_idx = 0;
                        }
                        KeyCode::Up | KeyCode::Char('k') => {
                            app.cursor_idx = app.cursor_idx.saturating_sub(1);
                        }
                        KeyCode::Down | KeyCode::Char('j') => {
                            if app.cursor_idx + 1 < app.providers.len() {
                                app.cursor_idx += 1;
                            }
                        }
                        KeyCode::Enter => {
                            app.select_provider(app.cursor_idx);
                            app.show_menu = false;
                        }
                        _ => {}
                    }
                }
            }
            continue;
        }

        let ev = event::read()?;
        match ev {
            Event::Key(key) if key.kind == KeyEventKind::Press => {
                match key.code {
                    KeyCode::Esc => return Ok(()),
                    KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => return Ok(()),
                    KeyCode::PageUp => app.scroll = app.scroll.saturating_add(5),
                    KeyCode::PageDown => app.scroll = app.scroll.saturating_sub(5),
                    KeyCode::Enter => {
                        let content: String = app.input.lines().iter()
                            .map(|s| s.trim())
                            .collect::<Vec<_>>().join("\n");
                        let content = content.trim().to_string();
                        if !content.is_empty() {
                            app.scroll = 0;

                            if content.starts_with('/') {
                                let parts: Vec<&str> = content.split_whitespace().collect();
                                match parts.get(0).map(|s| *s) {
                                    Some("/provider") => {
                                        if parts.len() == 1 {
                                            let list: Vec<String> = app.providers.iter().map(|p| p.name.clone()).collect();
                                            app.messages.push(ChatMsg { role: "assistant".into(), content: format!("Available providers: {}", list.join(", ")) });
                                        } else if let Some(name) = parts.get(1) {
                                            if let Some(idx) = app.providers.iter().position(|p| p.name == *name) {
                                                app.select_provider(idx);
                                                app.messages.push(ChatMsg { role: "assistant".into(), content: format!("Switched to {}", name) });
                                            } else {
                                                app.messages.push(ChatMsg { role: "assistant".into(), content: format!("Unknown provider '{}'", name) });
                                            }
                                        }
                                    }
                                    Some("/model") => {
                                        if parts.len() == 1 {
                                            if let Some(entry) = app.providers.iter().find(|p| p.name == app.provider) {
                                                app.messages.push(ChatMsg { role: "assistant".into(), content: format!("Models for {}: {}", app.provider, entry.models.join(", ")) });
                                            }
                                        } else if let Some(mdl) = parts.get(1) {
                                            if let Some(entry) = app.providers.iter().find(|p| p.name == app.provider) {
                                                if entry.models.iter().any(|m| m == mdl) {
                                                    app.model = (*mdl).to_string();
                                                    app.messages.push(ChatMsg { role: "assistant".into(), content: format!("Switched to {}", mdl) });
                                                } else {
                                                    app.messages.push(ChatMsg { role: "assistant".into(), content: format!("Model '{}' not available for {}", mdl, app.provider) });
                                                }
                                            }
                                        }
                                    }
                                    _ => {
                                        app.messages.push(ChatMsg { role: "user".into(), content: content.clone() });
                                    }
                                }
                            } else {
                                app.messages.push(ChatMsg { role: "user".into(), content: content.clone() });
                            }

                            if let Some(last) = app.messages.last() {
                                if last.role == "user" {
                                    let api_key = std::env::var("API_KEY")
                                        .or_else(|_| std::env::var("OPENAI_API_KEY"))
                                        .or_else(|_| std::env::var("CLAUDE_API_KEY"))
                                        .unwrap_or_default();
                                    if api_key.is_empty() {
                                        app.messages.push(ChatMsg { role: "assistant".into(), content: "No API key set (API_KEY, OPENAI_API_KEY, or CLAUDE_API_KEY)".into() });
                                    } else {
                                        match ai::make_provider(&app.provider, &api_key) {
                                            Ok(provider) => {
                                                let ai_messages: Vec<ai::AiMessage> = app.messages.iter()
                                                    .map(|m| ai::AiMessage { role: m.role.clone(), content: m.content.clone() })
                                                    .collect();
                                                match provider.stream_chat(&app.model, &ai_messages, None, &[]).await {
                                                    Ok(mut stream) => {
                                                        app.streaming = true;
                                                        let mut buf = String::new();
                                                        app.messages.push(ChatMsg { role: "assistant".into(), content: String::new() });
                                                        loop {
                                                            let delta = stream.next().await;
                                                            let mut done = false;
                                                            match delta {
                                                                Some(Ok(ai::StreamDelta::Text(t))) => { buf.push_str(&t); },
                                                                Some(Ok(ai::StreamDelta::Stop)) | None => { done = true; },
                                                                Some(Ok(_)) => {},
                                                                Some(Err(e)) => {
                                                                    buf = format!("Stream error: {}", e);
                                                                    done = true;
                                                                }
                                                            }
                                                            if let Some(m) = app.messages.last_mut() {
                                                                m.content = if buf.is_empty() && !done { "● ...".into() } else { buf.clone() };
                                                            }
                                                            terminal.draw(|f| ui(f, app))?;
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
                                }
                            }
                        }
                        let mut ta = tui_textarea::TextArea::default();
                        ta.set_placeholder_text("Ask anything...");
                        ta.set_style(Style::default().bg(TEAL).fg(FG));
                        app.input = ta;
                    }
                    _ => {
                        app.input.input(ev);
                    }
                }
            }
            _ => {}
        }
    }
}

fn ui(f: &mut ratatui::Frame, app: &mut ChatApp) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Min(0),
            Constraint::Length(3),
        ])
        .split(f.area());

    let status_text = format!(
        " {} :: {}  {} {}",
        app.provider, app.model,
        if app.streaming { "● ..." } else { "" },
        if !app.streaming { format!("│ {} msgs", app.messages.len()) } else { String::new() }
    );
    let status = Span::styled(status_text, Style::default().fg(TEAL_BRIGHT).bg(TEAL));
    f.render_widget(Paragraph::new(Line::from(status)).style(Style::default().bg(TEAL)), chunks[0]);

    let messages_lines: Vec<Line> = if app.show_menu {
        provider_menu_lines(app)
    } else {
        build_message_lines(app)
    };

    let msg_widget = Paragraph::new(Text::from(messages_lines))
        .style(Style::default().bg(BG).fg(FG));
    f.render_widget(msg_widget, chunks[1]);

    let input_block = Block::default()
        .borders(Borders::TOP)
        .border_style(Style::default().fg(TEAL_DIM));
    app.input.set_block(input_block);
    app.input.set_style(Style::default().bg(TEAL).fg(FG));
    app.input.set_cursor_style(Style::default().fg(TEAL_BRIGHT));
    f.render_widget(&app.input, chunks[2]);
}

fn build_message_lines(app: &ChatApp) -> Vec<Line> {
    let mut lines: Vec<Line> = Vec::new();

    if app.messages.is_empty() {
        lines.push(Line::from(""));
        lines.push(Line::from(Span::styled(
            "  Welcome to kodigocode",
            Style::default().fg(TEAL_BRIGHT).bold(),
        )));
        lines.push(Line::from(""));
        lines.push(Line::from(Span::styled(
            "  Commands:",
            Style::default().fg(Color::Gray),
        )));
        lines.push(Line::from(Span::styled(
            "    /provider          List available AI providers",
            Style::default().fg(Color::Gray),
        )));
        lines.push(Line::from(Span::styled(
            "    /provider <name>   Switch to a specific provider",
            Style::default().fg(Color::Gray),
        )));
        lines.push(Line::from(Span::styled(
            "    /model             List models for current provider",
            Style::default().fg(Color::Gray),
        )));
        lines.push(Line::from(Span::styled(
            "    /model <name>      Switch to a specific model",
            Style::default().fg(Color::Gray),
        )));
        lines.push(Line::from(""));
        lines.push(Line::from(Span::styled(
            "  Type your message and press Enter to start coding.",
            Style::default().fg(TEAL_BRIGHT),
        )));
        return lines;
    }

    let skip = app.scroll.min(app.messages.len().saturating_sub(1));
    let visible: Vec<&ChatMsg> = app.messages.iter().skip(skip).collect();

    for msg in visible {
        let (prefix, role_color) = match msg.role.as_str() {
            "user" => ("▸", ACCENT),
            _ => ("■", TEAL_BRIGHT),
        };

        lines.push(Line::from(vec![
            Span::styled(format!("{}", prefix), Style::default().fg(role_color).bold()),
            Span::styled(format!(" {}", msg.role), Style::default().fg(role_color)),
        ]));

        for cl in msg.content.lines() {
            if cl.is_empty() {
                lines.push(Line::from(""));
            } else {
                lines.push(Line::from(vec![
                    Span::styled("  ", Style::default().fg(role_color)),
                    Span::raw(cl),
                ]));
            }
        }

        if msg.role == "assistant" && !msg.content.is_empty() && !msg.content.contains("\n") {
            // no gap needed for short messages
        } else {
            lines.push(Line::from(""));
        }
    }
    lines
}

fn provider_menu_lines(app: &ChatApp) -> Vec<Line> {
    let mut lines: Vec<Line> = vec![
        Line::from(Span::styled(
            "  Select Provider / Model",
            Style::default().fg(TEAL_BRIGHT).bold(),
        )),
        Line::from(""),
        Line::from(Span::styled(
            "  j/k or Up/Down: navigate  ·  Enter: select  ·  Esc: cancel",
            Style::default().fg(Color::Gray),
        )),
        Line::from(""),
    ];
    for (i, p) in app.providers.iter().enumerate() {
        let selected = i == app.cursor_idx;
        let prefix = if selected { " ▸ " } else { "   " };
        let style = if selected {
            Style::default().fg(TEAL_BRIGHT).bold()
        } else {
            Style::default().fg(FG)
        };
        lines.push(Line::from(Span::styled(
            format!("{}{}", prefix, p.name),
            style,
        )));
        if selected {
            for mdl in &p.models {
                lines.push(Line::from(Span::styled(
                    format!("       · {}", mdl),
                    Style::default().fg(Color::Gray),
                )));
            }
        }
    }
    lines
}