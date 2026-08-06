use anyhow::Result;
use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEventKind, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::Terminal;
use ratatui::widgets::Clear;
use crate::config;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Paragraph},
};
use tokio_stream::StreamExt;
use std::io;
#[path = "ai/mod.rs"]
mod ai;
#[path = "providers.rs"]
mod providers;

use crate::palette::{BG, FG, ACCENT, DIM, PANEL};

// Convert markdown text (including fenced code blocks) into styled lines for rendering.
fn markdown_to_lines(content: &str) -> Vec<Line<'static>> {
    use pulldown_cmark::{Parser, Event as MdEvent, Tag, CodeBlockKind};
    use syntect::{parsing::SyntaxSet, highlighting::{ThemeSet}};
    use syntect::easy::HighlightLines;
    let mut lines = Vec::new();
    let parser = Parser::new(content);
    let syntax_set = SyntaxSet::load_defaults_newlines();
    let theme_set = ThemeSet::load_defaults();
    let theme = theme_set.themes.get("base16-ocean.dark").unwrap_or_else(|| theme_set.themes.values().next().unwrap());
    let mut in_code = false;
    let mut code_lang: Option<String> = None;
    let mut highlighter: Option<HighlightLines> = None;
    for event in parser {
        match event {
            MdEvent::Start(Tag::CodeBlock(kind)) => {
                in_code = true;
                code_lang = match kind {
                    CodeBlockKind::Fenced(info) => Some(info.to_string()),
                    CodeBlockKind::Indented => None,
                };
                let syntax = code_lang
                    .as_ref()
                    .and_then(|lang| syntax_set.find_syntax_by_token(lang))
                    .unwrap_or_else(|| syntax_set.find_syntax_plain_text());
                highlighter = Some(HighlightLines::new(syntax, theme));
            }
            MdEvent::End(_) => {
                in_code = false;
                code_lang = None;
                highlighter = None;
            }
            MdEvent::Text(text) => {
                if in_code {
                    if let Some(ref mut hl) = highlighter {
                        for line in text.lines() {
                            let ranges = hl.highlight_line(line, &syntax_set).unwrap_or_default();
                            let mut spans = Vec::new();
                            for (style, part) in ranges {
                                let col = Color::Rgb(style.foreground.r, style.foreground.g, style.foreground.b);
                                spans.push(Span::styled(part.to_string(), Style::default().fg(col)));
                            }
                            lines.push(Line::from(spans));
                        }
                    }
                } else {
                    for line in text.lines() {
                        lines.push(Line::from(line.to_string()));
                    }
                }
            }
            MdEvent::SoftBreak | MdEvent::HardBreak => {
                lines.push(Line::from(String::new()));
            }
            _ => {}
        }
    }
    lines
}


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
    // Simple undo history: each entry is a snapshot of messages before the last user turn
    pub history: Vec<Vec<ChatMsg>>,
}

#[derive(Clone)]
pub struct ChatMsg { pub role: String, pub content: String }
pub struct ProviderEntry {
    pub name: String,
    pub description: String,
    pub models: Vec<String>,
    pub api_key_env: String,
    pub base_url: String,
}

impl ChatApp {
    pub fn new(provider: String, model: String) -> Self {
        let mut ta = tui_textarea::TextArea::default();
        ta.set_placeholder_text("Ask anything...");
        ta.set_style(Style::default().bg(PANEL).fg(FG));

        let providers: Vec<ProviderEntry> = providers::all_providers()
            .into_iter()
            .map(|p| ProviderEntry {
                name: p.name,
                description: p.description,
                models: p.models.into_iter().map(|m| m.name).collect(),
                api_key_env: p.api_key_env,
                base_url: p.default_base_url,
            })
            .collect();

        Self {
            messages: vec![], input: ta, provider, model, providers,
            scroll: 0, streaming: false,
            palette: false, palette_filter: String::new(), palette_idx: 0,
            history: Vec::new(),
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
    execute!(stdout, EnterAlternateScreen, cursor::SetCursorStyle::BlinkingBlock)?;
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
                        if let Some((name, _)) = entries.get(app.palette_idx) {
                            if app.palette_filter.starts_with("/model") {
                                // Select model
                                if app.providers.iter().any(|p| p.name == app.provider && p.models.iter().any(|m| m == name)) {
                                    app.model = name.clone();
                                    app.messages.push(ChatMsg { role: "assistant".into(), content: format!("Switched to model: {}", name) });
                                    persist_config(app);
                                }
                            } else {
                                // Select provider
                                if let Some(idx) = app.providers.iter().position(|p| &p.name == name) {
                                    app.select_provider(idx);
                                    app.messages.push(ChatMsg { role: "assistant".into(), content: format!("Switched to provider: {} — {}", name, app.providers[idx].description) });
                                    persist_config(app);
                                }
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

fn matched_entries(app: &ChatApp) -> Vec<(String, String)> {
    let filter = app.palette_filter.trim_start_matches('/').to_lowercase();

    if app.palette_filter.starts_with("/model") {
        let model_filter = app.palette_filter.strip_prefix("/model").unwrap_or("").trim().to_lowercase();
        if let Some(entry) = app.providers.iter().find(|p| p.name == app.provider) {
            entry.models.iter()
                .filter(|m| model_filter.is_empty() || m.to_lowercase().contains(&model_filter))
                .map(|m| (m.clone(), format!("Model for {}", app.provider)))
                .collect()
        } else {
            vec![]
        }
    } else {
        app.providers.iter()
            .filter(|p| filter.is_empty() || p.name.to_lowercase().contains(&filter) || p.description.to_lowercase().contains(&filter))
            .map(|p| (p.name.clone(), format!("{} — {}", p.name, p.description)))
            .collect()
    }
}

async fn handle_send(terminal: &mut Terminal<ratatui::backend::CrosstermBackend<io::Stdout>>, app: &mut ChatApp) {
    let content = app.input.lines().join("\n").trim().to_string();
    if content.is_empty() { return; }

    app.scroll = 0;

    // Handle special slash commands (/clear, /help)
    if content == "/clear" {
        app.messages.clear();
        app.messages.push(ChatMsg { role: "assistant".into(), content: "Chat cleared.".into() });
        return;
    }
    if content == "/help" {
        let help_text = "Available commands:\n  /setup key <provider> <api_key>\n  /setup url <provider> <base_url>\n  /model [model] – list or switch model\n  /clear – clear chat history\n  /help – show this help";
        app.messages.push(ChatMsg { role: "assistant".into(), content: help_text.into() });
        return;
    }
    // Handle undo command
    if content == "/undo" {
        if let Some(prev) = app.history.pop() {
            app.messages = prev;
            app.messages.push(ChatMsg { role: "assistant".into(), content: "Undid last turn.".into() });
        } else {
            app.messages.push(ChatMsg { role: "assistant".into(), content: "No history to undo.".into() });
        }
        return;
    }
    // Handle /setup command
    if content.starts_with("/setup") {
        let parts: Vec<&str> = content.split_whitespace().collect();
        if parts.len() >= 3 && parts[1] == "key" {
            // /setup key <provider> <api_key>
            let prov = parts[2];
            let key = parts.get(3).unwrap_or(&"");
            std::env::set_var(
                providers::find_provider(prov).map(|p| p.api_key_env).unwrap_or_else(|| "API_KEY".into()),
                key,
            );
            app.messages.push(ChatMsg { role: "assistant".into(), content: format!("API key set for {}", prov) });
        } else if parts.len() >= 3 && parts[1] == "url" {
            // /setup url <provider> <base_url>
            let prov = parts[2];
            let url = parts.get(3).unwrap_or(&"");
            if let Some(idx) = app.providers.iter().position(|p| p.name == prov) {
                app.providers[idx].base_url = url.to_string();
                app.messages.push(ChatMsg { role: "assistant".into(), content: format!("Base URL set for {}: {}", prov, url) });
            } else {
                app.messages.push(ChatMsg { role: "assistant".into(), content: format!("Unknown provider '{}'", prov) });
            }
        } else {
            app.messages.push(ChatMsg { role: "assistant".into(), content: "Usage:\n  /setup key <provider> <api_key>\n  /setup url <provider> <base_url>".into() });
        }
        return;
    }

    if content.starts_with('/') {
        let parts: Vec<&str> = content.split_whitespace().collect();
        match parts.get(0).map(|s| *s) {
            Some("/model") => {
                if parts.len() == 1 {
                    if let Some(entry) = app.providers.iter().find(|p| p.name == app.provider) {
                        app.messages.push(ChatMsg { role: "assistant".into(), content: format!("Models for {}: {}", app.provider, entry.models.join(", ")) });
                    }
                } else if let Some(mdl) = parts.get(1) {
                    if let Some(entry) = app.providers.iter().find(|p| p.name == app.provider) {
                        if entry.models.iter().any(|m| m == mdl) {
                            app.model = (*mdl).to_string();
                            app.messages.push(ChatMsg { role: "assistant".into(), content: format!("Switched to model: {}", mdl) });
                            persist_config(app);
                        } else {
                            app.messages.push(ChatMsg { role: "assistant".into(), content: format!("Model '{}' not available for {}", mdl, app.provider) });
                        }
                    }
                }
                return;
            }
            _ => {}
        }
    }

    // Save snapshot for undo before adding new user message
    app.history.push(app.messages.clone());
    app.messages.push(ChatMsg { role: "user".into(), content: content.clone() });

    // Get API key for current provider
    let api_key = providers::get_api_key(&app.provider).unwrap_or_default();

    if api_key.is_empty() {
        let env_var = app.providers.iter()
            .find(|p| p.name == app.provider)
            .map(|p| p.api_key_env.clone())
            .unwrap_or_else(|| "API_KEY".into());
        app.messages.push(ChatMsg {
            role: "assistant".into(),
            content: format!("No API key set for {}. Set it with:\n  /setup key {} <your_key>\nor export {} in your shell.", app.provider, app.provider, env_var),
        });
    } else {
        // Get optional base_url override
        let base_url = app.providers.iter()
            .find(|p| p.name == app.provider)
            .map(|p| p.base_url.as_str());

        match ai::make_provider_with_url(&app.provider, &api_key, base_url) {
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
                                Some(Ok(ai::StreamDelta::ToolUseStart { name, .. })) => {
                                    // Insert a placeholder for a tool call
                                    app.messages.push(ChatMsg { role: "assistant".into(), content: format!("[Tool: {} started]", name) });
                                }
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
    ta.set_style(Style::default().bg(PANEL).fg(FG));
    app.input = ta;
}

fn persist_config(app: &ChatApp) {
    if let Ok(mut cfg) = config::load() {
        cfg.provider.provider = app.provider.clone();
        cfg.provider.model = app.model.clone();
        let _ = config::save(&cfg);
    }
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
            Span::styled("provider", Style::default().fg(ACCENT).add_modifier(Modifier::BOLD)),
            Span::styled("  Switch AI provider", Style::default().fg(DIM)),
        ]));
        lines.push(Line::from(vec![
            Span::styled("  /", Style::default().fg(DIM)),
            Span::styled("model", Style::default().fg(ACCENT).add_modifier(Modifier::BOLD)),
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
                    // Assistant message: render markdown with optional syntax highlighting
                    if msg.content.is_empty() || msg.content == "● ..." {
                        lines.push(Line::from(Span::styled(
                            "● ...",
                            Style::default().fg(DIM).italic(),
                        )));
                    } else {
                        // Convert markdown to styled lines
                        let md_lines = markdown_to_lines(&msg.content);
                        for l in md_lines {
                            lines.push(l);
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
        .style(Style::default().bg(PANEL));
    f.render_widget(p, area);
}

fn render_input(f: &mut ratatui::Frame, app: &mut ChatApp, area: Rect) {
    let input_block = Block::default()
        .borders(Borders::TOP)
        .border_style(Style::default().fg(DIM));
    app.input.set_block(input_block);
    app.input.set_style(Style::default().bg(PANEL).fg(FG));
    app.input.set_cursor_style(Style::default().fg(ACCENT));
    f.render_widget(&app.input, area);
}

fn render_palette(f: &mut ratatui::Frame, app: &ChatApp, parent: Rect) {
    let entries = matched_entries(app);
    if entries.is_empty() { return; }

    let is_model_mode = app.palette_filter.starts_with("/model");
    let height = (entries.len() as u16).min(12) + 2;
    let width = if is_model_mode { 44u16 } else { 48u16 };
    let x = 2u16;
    let y = if parent.height > height + 3 { parent.y + 2 } else { parent.bottom().saturating_sub(height) };

    let area = Rect { x, y, width, height };

    let mut lines: Vec<Line> = vec![];
    lines.push(Line::from(Span::styled(
        format!(" {}", app.palette_filter),
        Style::default().fg(ACCENT).add_modifier(Modifier::BOLD),
    )));
    if is_model_mode {
        lines.push(Line::from(Span::styled("─".repeat(width as usize - 2), Style::default().fg(DIM))));
    } else {
        lines.push(Line::from(Span::styled(" Provider │ press /model for models, type to filter", Style::default().fg(DIM))));
        lines.push(Line::from(Span::styled("─".repeat(width as usize - 2), Style::default().fg(DIM))));
    }

    for (i, (name, desc)) in entries.iter().enumerate() {
        let selected = i == app.palette_idx;
        let style = if selected {
            Style::default().fg(ACCENT).bg(ACCENT)
        } else {
            Style::default().fg(FG).bg(PANEL)
        };
        let prefix = if selected { "▸ " } else { "  " };
        if is_model_mode {
            lines.push(Line::from(Span::styled(format!("{}{}", prefix, name), style)));
        } else {
            lines.push(Line::from(Span::styled(format!("{}{}", prefix, desc), style)));
        }
    }

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(ACCENT))
        .style(Style::default().bg(PANEL));

    f.render_widget(Clear, area);
    f.render_widget(Paragraph::new(Text::from(lines)).block(block), area);
}