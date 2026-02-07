mod app;
mod config;
mod conversation;
mod models;
mod ui;

use anyhow::Result;
use app::{App, AppState};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    Terminal,
};
use std::io;
use tracing_subscriber;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app
    let mut app = App::new().await?;

    // Run app
    let res = run_app(&mut terminal, &mut app).await;

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        eprintln!("Error: {:?}", err);
    }

    Ok(())
}

async fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
) -> Result<()> {
    loop {
        terminal.draw(|f| ui::render(f, app))?;

        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                match app.state {
                    AppState::ModelSelector => {
                        match key.code {
                            KeyCode::Char('q') => return Ok(()),
                            KeyCode::Up | KeyCode::Char('k') => app.previous_model(),
                            KeyCode::Down | KeyCode::Char('j') => app.next_model(),
                            KeyCode::Enter => {
                                app.load_selected_model().await?;
                            }
                            KeyCode::Char('r') => {
                                app.refresh_models().await?;
                            }
                            _ => {}
                        }
                    }
                    AppState::Chat => {
                        match (key.modifiers, key.code) {
                            // Ctrl+C: Open menu
                            (KeyModifiers::CONTROL, KeyCode::Char('c')) => {
                                app.state = AppState::Menu;
                            }
                            // Ctrl+L: Clear chat
                            (KeyModifiers::CONTROL, KeyCode::Char('l')) => {
                                app.clear_chat();
                            }
                            // Ctrl+S: Save conversation
                            (KeyModifiers::CONTROL, KeyCode::Char('s')) => {
                                app.save_conversation()?;
                            }
                            // Enter: Send message
                            (_, KeyCode::Enter) if !key.modifiers.contains(KeyModifiers::SHIFT) => {
                                app.send_message().await?;
                            }
                            // Shift+Enter: New line
                            (KeyModifiers::SHIFT, KeyCode::Enter) => {
                                app.input.push('\n');
                            }
                            // Ctrl+Enter: Send (always)
                            (KeyModifiers::CONTROL, KeyCode::Enter) => {
                                app.send_message().await?;
                            }
                            // Backspace
                            (_, KeyCode::Backspace) => {
                                app.input.pop();
                            }
                            // Character input
                            (_, KeyCode::Char(c)) => {
                                app.input.push(c);
                            }
                            // Up arrow: Scroll chat up
                            (_, KeyCode::Up) if app.input.is_empty() => {
                                app.scroll_up();
                            }
                            // Down arrow: Scroll chat down
                            (_, KeyCode::Down) if app.input.is_empty() => {
                                app.scroll_down();
                            }
                            _ => {}
                        }
                    }
                    AppState::Menu => {
                        match key.code {
                            KeyCode::Char('q') | KeyCode::Esc => {
                                app.state = AppState::Chat;
                            }
                            KeyCode::Char('1') => {
                                app.state = AppState::ModelSelector;
                            }
                            KeyCode::Char('2') => {
                                app.cycle_preset();
                            }
                            KeyCode::Char('3') => {
                                app.save_conversation()?;
                                app.state = AppState::Chat;
                            }
                            KeyCode::Char('4') => {
                                app.state = AppState::ConversationList;
                            }
                            KeyCode::Char('5') => {
                                app.clear_chat();
                                app.state = AppState::Chat;
                            }
                            KeyCode::Char('6') => {
                                app.state = AppState::Settings;
                            }
                            _ => {}
                        }
                    }
                    AppState::Settings => {
                        match key.code {
                            KeyCode::Esc | KeyCode::Char('q') => {
                                app.state = AppState::Menu;
                            }
                            KeyCode::Up => app.previous_setting(),
                            KeyCode::Down => app.next_setting(),
                            KeyCode::Enter => app.edit_setting(),
                            _ => {}
                        }
                    }
                    AppState::ConversationList => {
                        match key.code {
                            KeyCode::Esc | KeyCode::Char('q') => {
                                app.state = AppState::Menu;
                            }
                            KeyCode::Up => app.previous_conversation(),
                            KeyCode::Down => app.next_conversation(),
                            KeyCode::Enter => {
                                app.load_selected_conversation()?;
                                app.state = AppState::Chat;
                            }
                            KeyCode::Char('d') => {
                                app.delete_selected_conversation()?;
                            }
                            _ => {}
                        }
                    }
                }
            }
        }

        // Handle streaming responses
        if app.is_streaming {
            if let Some(response) = app.poll_response().await? {
                app.handle_response_chunk(response);
            }
        }
    }
}
