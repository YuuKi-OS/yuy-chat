mod selector;
mod chat;
mod menu;
mod settings;
mod conversations;

use crate::app::{App, AppState};
use ratatui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    Frame,
};

pub fn render<B: Backend>(f: &mut Frame, app: &App) {
    match app.state {
        AppState::ModelSelector => selector::render(f, app),
        AppState::Chat => chat::render(f, app),
        AppState::Menu => menu::render(f, app),
        AppState::Settings => settings::render(f, app),
        AppState::ConversationList => conversations::render(f, app),
    }
}
