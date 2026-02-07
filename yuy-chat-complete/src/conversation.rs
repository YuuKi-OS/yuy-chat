use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fs;
use crate::config::Config;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub role: String,
    pub content: String,
    pub timestamp: DateTime<Utc>,
}

impl Message {
    pub fn user(content: String) -> Self {
        Self {
            role: "user".to_string(),
            content,
            timestamp: Utc::now(),
        }
    }

    pub fn assistant(content: String) -> Self {
        Self {
            role: "assistant".to_string(),
            content,
            timestamp: Utc::now(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Conversation {
    pub messages: Vec<Message>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Conversation {
    pub fn new() -> Self {
        let now = Utc::now();
        Self {
            messages: Vec::new(),
            created_at: now,
            updated_at: now,
        }
    }

    pub fn add_message(&mut self, message: Message) {
        self.messages.push(message);
        self.updated_at = Utc::now();
    }

    pub fn save(&self) -> Result<String> {
        let conversations_dir = Config::conversations_dir()?;
        let filename = format!("conversation-{}.json", self.created_at.format("%Y%m%d-%H%M%S"));
        let path = conversations_dir.join(&filename);
        
        let json = serde_json::to_string_pretty(self)?;
        fs::write(&path, json)?;
        
        Ok(filename)
    }

    pub fn load(filename: &str) -> Result<Self> {
        let conversations_dir = Config::conversations_dir()?;
        let path = conversations_dir.join(filename);
        
        let content = fs::read_to_string(&path)?;
        let conversation: Conversation = serde_json::from_str(&content)?;
        
        Ok(conversation)
    }

    pub fn list_saved() -> Result<Vec<String>> {
        let conversations_dir = Config::conversations_dir()?;
        
        let mut conversations = Vec::new();
        
        if conversations_dir.exists() {
            for entry in fs::read_dir(&conversations_dir)? {
                let entry = entry?;
                if entry.path().extension().map_or(false, |e| e == "json") {
                    if let Some(filename) = entry.file_name().to_str() {
                        conversations.push(filename.to_string());
                    }
                }
            }
        }
        
        conversations.sort();
        conversations.reverse(); // Most recent first
        
        Ok(conversations)
    }

    pub fn delete(filename: &str) -> Result<()> {
        let conversations_dir = Config::conversations_dir()?;
        let path = conversations_dir.join(filename);
        
        if path.exists() {
            fs::remove_file(&path)?;
        }
        
        Ok(())
    }

    pub fn get_summary(&self) -> String {
        if let Some(first_msg) = self.messages.first() {
            let preview = first_msg.content.chars().take(50).collect::<String>();
            if first_msg.content.len() > 50 {
                format!("{}...", preview)
            } else {
                preview
            }
        } else {
            "Empty conversation".to_string()
        }
    }
}
