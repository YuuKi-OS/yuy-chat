use crate::config::{Config, Preset};
use crate::conversation::{Conversation, Message};
use crate::models::{Model, ModelScanner, ModelRuntime};
use anyhow::Result;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug, Clone, PartialEq)]
pub enum AppState {
    ModelSelector,
    Chat,
    Menu,
    Settings,
    ConversationList,
}

pub struct App {
    pub state: AppState,
    pub config: Config,
    
    // Models
    pub models: Vec<Model>,
    pub selected_model_idx: usize,
    pub current_model: Option<Model>,
    pub runtime: Option<Arc<Mutex<ModelRuntime>>>,
    
    // Chat
    pub conversation: Conversation,
    pub input: String,
    pub scroll_offset: usize,
    pub is_streaming: bool,
    
    // Conversations history
    pub saved_conversations: Vec<String>,
    pub selected_conversation_idx: usize,
    
    // Settings
    pub selected_setting_idx: usize,
    
    // Preset
    pub current_preset: Preset,
}

impl App {
    pub async fn new() -> Result<Self> {
        let config = Config::load()?;
        let scanner = ModelScanner::new();
        let models = scanner.scan_all(&config).await?;

        let saved_conversations = Conversation::list_saved()?;

        Ok(Self {
            state: if models.is_empty() {
                AppState::ModelSelector
            } else {
                AppState::ModelSelector
            },
            config,
            models,
            selected_model_idx: 0,
            current_model: None,
            runtime: None,
            conversation: Conversation::new(),
            input: String::new(),
            scroll_offset: 0,
            is_streaming: false,
            saved_conversations,
            selected_conversation_idx: 0,
            selected_setting_idx: 0,
            current_preset: Preset::Balanced,
        })
    }

    pub fn previous_model(&mut self) {
        if self.selected_model_idx > 0 {
            self.selected_model_idx -= 1;
        }
    }

    pub fn next_model(&mut self) {
        if self.selected_model_idx < self.models.len().saturating_sub(1) {
            self.selected_model_idx += 1;
        }
    }

    pub async fn refresh_models(&mut self) -> Result<()> {
        let scanner = ModelScanner::new();
        self.models = scanner.scan_all(&self.config).await?;
        self.selected_model_idx = 0;
        Ok(())
    }

    pub async fn load_selected_model(&mut self) -> Result<()> {
        if let Some(model) = self.models.get(self.selected_model_idx).cloned() {
            let runtime = ModelRuntime::new(model.clone(), self.current_preset.clone()).await?;
            self.current_model = Some(model);
            self.runtime = Some(Arc::new(Mutex::new(runtime)));
            self.state = AppState::Chat;
        }
        Ok(())
    }

    pub async fn send_message(&mut self) -> Result<()> {
        if self.input.trim().is_empty() {
            return Ok(());
        }

        let user_message = self.input.clone();
        self.conversation.add_message(Message::user(user_message.clone()));
        self.input.clear();

        if let Some(runtime) = &self.runtime {
            self.is_streaming = true;
            let runtime = runtime.clone();
            let user_msg = user_message.clone();
            
            tokio::spawn(async move {
                let mut rt = runtime.lock().await;
                if let Err(e) = rt.generate(&user_msg).await {
                    tracing::error!("Error generating response: {:?}", e);
                }
            });
        }

        Ok(())
    }

    pub async fn poll_response(&mut self) -> Result<Option<String>> {
        if let Some(runtime) = &self.runtime {
            let mut rt = runtime.lock().await;
            Ok(rt.poll_chunk().await?)
        } else {
            Ok(None)
        }
    }

    pub fn handle_response_chunk(&mut self, chunk: String) {
        if chunk == "[DONE]" {
            self.is_streaming = false;
            return;
        }

        if let Some(last_msg) = self.conversation.messages.last_mut() {
            if last_msg.role == "assistant" {
                last_msg.content.push_str(&chunk);
            } else {
                self.conversation.add_message(Message::assistant(chunk));
            }
        } else {
            self.conversation.add_message(Message::assistant(chunk));
        }
    }

    pub fn clear_chat(&mut self) {
        self.conversation = Conversation::new();
        self.scroll_offset = 0;
    }

    pub fn scroll_up(&mut self) {
        self.scroll_offset = self.scroll_offset.saturating_sub(1);
    }

    pub fn scroll_down(&mut self) {
        self.scroll_offset = self.scroll_offset.saturating_add(1);
    }

    pub fn cycle_preset(&mut self) {
        self.current_preset = match self.current_preset {
            Preset::Creative => Preset::Balanced,
            Preset::Balanced => Preset::Precise,
            Preset::Precise => Preset::Creative,
        };
    }

    pub fn save_conversation(&mut self) -> Result<()> {
        let filename = self.conversation.save()?;
        self.saved_conversations.push(filename);
        Ok(())
    }

    pub fn previous_conversation(&mut self) {
        if self.selected_conversation_idx > 0 {
            self.selected_conversation_idx -= 1;
        }
    }

    pub fn next_conversation(&mut self) {
        if self.selected_conversation_idx < self.saved_conversations.len().saturating_sub(1) {
            self.selected_conversation_idx += 1;
        }
    }

    pub fn load_selected_conversation(&mut self) -> Result<()> {
        if let Some(filename) = self.saved_conversations.get(self.selected_conversation_idx) {
            self.conversation = Conversation::load(filename)?;
        }
        Ok(())
    }

    pub fn delete_selected_conversation(&mut self) -> Result<()> {
        if let Some(filename) = self.saved_conversations.get(self.selected_conversation_idx) {
            Conversation::delete(filename)?;
            self.saved_conversations.remove(self.selected_conversation_idx);
            if self.selected_conversation_idx > 0 {
                self.selected_conversation_idx -= 1;
            }
        }
        Ok(())
    }

    pub fn previous_setting(&mut self) {
        if self.selected_setting_idx > 0 {
            self.selected_setting_idx -= 1;
        }
    }

    pub fn next_setting(&mut self) {
        if self.selected_setting_idx < 5 {
            self.selected_setting_idx += 1;
        }
    }

    pub fn edit_setting(&mut self) {
        // Placeholder for setting editing
        // Would open input dialog
    }
}
