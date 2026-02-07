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
    let title = Paragraph::new("Settings")
        .style(Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(title, chunks[0]);

    // Settings items
    let items: Vec<ListItem> = vec![
        create_setting_item(0, app, "Models Directory", &app.config.models_dir.display().to_string()),
        create_setting_item(1, app, "HuggingFace Token", 
            if app.config.hf_token.is_some() { "hf_****..." } else { "Not set" }),
        create_setting_item(2, app, "Default Preset", app.config.default_preset.as_str()),
        create_setting_item(3, app, "Save History", 
            if app.config.save_history { "Enabled" } else { "Disabled" }),
        create_setting_item(4, app, "Theme", 
            match app.config.theme {
                crate::config::Theme::Dark => "Dark",
                crate::config::Theme::Light => "Light",
            }),
    ];

    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL));

    f.render_widget(list, chunks[1]);

    // Help
    let help = Paragraph::new("↑/↓: Navigate | Enter: Edit | Esc: Back")
        .style(Style::default().fg(Color::Gray))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    
    f.render_widget(help, chunks[2]);
}

fn create_setting_item(idx: usize, app: &App, label: &str, value: &str) -> ListItem {
    let is_selected = idx == app.selected_setting_idx;
    
    let line = if is_selected {
        Line::from(vec![
            Span::styled("> ", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
            Span::styled(label, Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::raw(": "),
            Span::styled(value, Style::default().fg(Color::Cyan)),
        ])
    } else {
        Line::from(vec![
            Span::raw("  "),
            Span::raw(label),
            Span::raw(": "),
            Span::styled(value, Style::default().fg(Color::Gray)),
        ])
    };
    
    ListItem::new(line)
}
