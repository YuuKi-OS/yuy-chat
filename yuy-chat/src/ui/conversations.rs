use crate::app::App;
use ratatui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};

pub fn render<B: Backend>(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(3),
        ])
        .split(f.area());

    // Title
    let title = Paragraph::new("Saved Conversations")
        .style(Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(title, chunks[0]);

    // Conversation list
    let items: Vec<ListItem> = if app.saved_conversations.is_empty() {
        vec![ListItem::new(Line::from(Span::styled(
            "No saved conversations",
            Style::default().fg(Color::Gray),
        )))]
    } else {
        app.saved_conversations
            .iter()
            .enumerate()
            .map(|(i, filename)| {
                let content = if i == app.selected_conversation_idx {
                    Line::from(vec![
                        Span::styled("> ", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
                        Span::styled(filename, Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
                    ])
                } else {
                    Line::from(vec![
                        Span::raw("  "),
                        Span::raw(filename),
                    ])
                };
                ListItem::new(content)
            })
            .collect()
    };

    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL));

    f.render_widget(list, chunks[1]);

    // Help
    let help = Paragraph::new("↑/↓: Navigate | Enter: Load | D: Delete | Esc: Back")
        .style(Style::default().fg(Color::Gray))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    
    f.render_widget(help, chunks[2]);
}
