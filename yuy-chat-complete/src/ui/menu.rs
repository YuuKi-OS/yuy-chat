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
    let title = Paragraph::new("Menu")
        .style(Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(title, chunks[0]);

    // Menu items
    let items = vec![
        ListItem::new(Line::from(vec![
            Span::styled("1. ", Style::default().fg(Color::Yellow)),
            Span::raw("Change Model"),
        ])),
        ListItem::new(Line::from(vec![
            Span::styled("2. ", Style::default().fg(Color::Yellow)),
            Span::raw(format!("Change Preset (Current: {})", app.current_preset.as_str())),
        ])),
        ListItem::new(Line::from(vec![
            Span::styled("3. ", Style::default().fg(Color::Yellow)),
            Span::raw("Save Conversation"),
        ])),
        ListItem::new(Line::from(vec![
            Span::styled("4. ", Style::default().fg(Color::Yellow)),
            Span::raw("Load Conversation"),
        ])),
        ListItem::new(Line::from(vec![
            Span::styled("5. ", Style::default().fg(Color::Yellow)),
            Span::raw("Clear Chat"),
        ])),
        ListItem::new(Line::from(vec![
            Span::styled("6. ", Style::default().fg(Color::Yellow)),
            Span::raw("Settings"),
        ])),
        ListItem::new(""),
        ListItem::new(Line::from(vec![
            Span::styled("Q. ", Style::default().fg(Color::Red)),
            Span::raw("Back to Chat"),
        ])),
    ];

    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title("Options"));

    f.render_widget(list, chunks[1]);

    // Help
    let help = Paragraph::new("Press number key or Q to go back")
        .style(Style::default().fg(Color::Gray))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    
    f.render_widget(help, chunks[2]);
}
