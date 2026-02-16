use crate::app::App;
use ratatui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};

pub fn render<B: Backend>(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),   // Header
            Constraint::Min(0),       // Messages
            Constraint::Length(5),    // Input
            Constraint::Length(1),    // Help
        ])
        .split(f.area());

    // Header
    render_header(f, app, chunks[0]);

    // Messages
    render_messages(f, app, chunks[1]);

    // Input box
    render_input(f, app, chunks[2]);

    // Help bar
    render_help(f, chunks[3]);
}

fn render_header<B: Backend>(f: &mut Frame, app: &App, area: ratatui::layout::Rect) {
    let model_name = app
        .current_model
        .as_ref()
        .map(|m| m.name.clone())
        .unwrap_or_else(|| "No model".to_string());

    let tokens = app.conversation.messages.iter()
        .map(|m| m.content.len())
        .sum::<usize>();

    let header_text = format!(
        "Model: {} | Preset: {} | Tokens: {}/4096 | Messages: {}",
        model_name,
        app.current_preset.as_str(),
        tokens,
        app.conversation.messages.len()
    );

    let header = Paragraph::new(header_text)
        .style(Style::default().fg(Color::Cyan))
        .alignment(Alignment::Left)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("yuy-chat")
                .style(Style::default().fg(Color::Magenta)),
        );

    f.render_widget(header, area);
}

fn render_messages<B: Backend>(f: &mut Frame, app: &App, area: ratatui::layout::Rect) {
    let mut lines = Vec::new();

    for msg in &app.conversation.messages {
        let (prefix, style) = if msg.role == "user" {
            ("You: ", Style::default().fg(Color::Blue).add_modifier(Modifier::BOLD))
        } else {
            ("Yuuki: ", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD))
        };

        lines.push(Line::from(vec![
            Span::styled(prefix, style),
            Span::raw(&msg.content),
        ]));
        lines.push(Line::from(""));
    }

    // Add streaming indicator
    if app.is_streaming {
        lines.push(Line::from(vec![
            Span::styled("Yuuki: ", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
            Span::styled("●●●", Style::default().fg(Color::Yellow)),
        ]));
    }

    let messages = Paragraph::new(lines)
        .block(Block::default().borders(Borders::ALL))
        .wrap(Wrap { trim: false })
        .scroll((app.scroll_offset as u16, 0));

    f.render_widget(messages, area);
}

fn render_input<B: Backend>(f: &mut Frame, app: &App, area: ratatui::layout::Rect) {
    let input_lines: Vec<Line> = app
        .input
        .split('\n')
        .map(|line| Line::from(line.to_string()))
        .collect();

    let input_widget = Paragraph::new(input_lines)
        .style(Style::default().fg(Color::White))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Message")
                .style(Style::default().fg(Color::Yellow)),
        )
        .wrap(Wrap { trim: false });

    f.render_widget(input_widget, area);

    // Set cursor position
    f.set_cursor_position((
        area.x + 1 + (app.input.len() as u16 % (area.width - 2)),
        area.y + 1 + (app.input.len() as u16 / (area.width - 2)),
    ));
}

fn render_help<B: Backend>(f: &mut Frame, area: ratatui::layout::Rect) {
    let help = Paragraph::new("Enter: Send | Shift+Enter: New line | Ctrl+C: Menu | Ctrl+L: Clear | Ctrl+S: Save")
        .style(Style::default().fg(Color::Gray))
        .alignment(Alignment::Center);

    f.render_widget(help, area);
}
