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
    let title = Paragraph::new("yuy-chat v0.1.0")
        .style(Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(title, chunks[0]);

    // Model list
    let items: Vec<ListItem> = app
        .models
        .iter()
        .enumerate()
        .map(|(i, model)| {
            let content = if i == app.selected_model_idx {
                Line::from(vec![
                    Span::styled("> ", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
                    Span::styled(&model.name, Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
                    Span::raw(format!("  {} ", model.display_name())),
                ])
            } else {
                Line::from(vec![
                    Span::raw("  "),
                    Span::raw(&model.name),
                    Span::styled(format!("  {} ", model.display_name()), Style::default().fg(Color::Gray)),
                ])
            };
            ListItem::new(content)
        })
        .collect();

    let list_widget = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("📋 Select a Model")
                .style(Style::default().fg(Color::Cyan)),
        );

    f.render_widget(list_widget, chunks[1]);

    // Help
    let help = if app.models.is_empty() {
        Paragraph::new("⚠️  No models found | Download with: yuy download Yuuki-best | R: Refresh | Q: Quit")
    } else {
        Paragraph::new("↑/↓: Navigate | Enter: Select | R: Refresh | Q: Quit")
    }
    .style(Style::default().fg(Color::Gray))
    .alignment(Alignment::Center)
    .block(Block::default().borders(Borders::ALL));
    
    f.render_widget(help, chunks[2]);
}
