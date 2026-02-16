use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub models_dir: PathBuf,
    pub hf_token: Option<String>,
    pub default_preset: Preset,
    pub save_history: bool,
    pub theme: Theme,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Preset {
    Creative,
    Balanced,
    Precise,
}

impl Preset {
    pub fn temperature(&self) -> f32 {
        match self {
            Preset::Creative => 0.8,
            Preset::Balanced => 0.6,
            Preset::Precise => 0.3,
        }
    }

    pub fn top_p(&self) -> f32 {
        match self {
            Preset::Creative => 0.9,
            Preset::Balanced => 0.7,
            Preset::Precise => 0.5,
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            Preset::Creative => "Creative",
            Preset::Balanced => "Balanced",
            Preset::Precise => "Precise",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Theme {
    Dark,
    Light,
}

impl Default for Config {
    fn default() -> Self {
        let home = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
        Self {
            models_dir: home.join(".yuuki").join("models"),
            hf_token: None,
            default_preset: Preset::Balanced,
            save_history: true,
            theme: Theme::Dark,
        }
    }
}

impl Config {
    pub fn load() -> Result<Self> {
        let config_path = Self::config_path()?;
        
        if config_path.exists() {
            let content = fs::read_to_string(&config_path)?;
            let config: Config = toml::from_str(&content)?;
            Ok(config)
        } else {
            let config = Config::default();
            config.save()?;
            Ok(config)
        }
    }

    pub fn save(&self) -> Result<()> {
        let config_path = Self::config_path()?;
        
        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent)?;
        }
        
        let content = toml::to_string_pretty(self)?;
        fs::write(&config_path, content)?;
        Ok(())
    }

    fn config_path() -> Result<PathBuf> {
        let config_dir = dirs::config_dir()
            .context("Could not find config directory")?
            .join("yuy-chat");
        
        fs::create_dir_all(&config_dir)?;
        Ok(config_dir.join("config.toml"))
    }

    pub fn conversations_dir() -> Result<PathBuf> {
        let config_dir = dirs::config_dir()
            .context("Could not find config directory")?
            .join("yuy-chat")
            .join("conversations");
        
        fs::create_dir_all(&config_dir)?;
        Ok(config_dir)
    }
}

// Yuuki constants
pub const HF_ORG: &str = "OpceanAI";
pub const OLLAMA_ORG: &str = "aguitachan3";
pub const YUUKI_API: &str = "https://huggingface.co/spaces/OpceanAI/Yuuki-api";
pub const AVAILABLE_QUANTS: &[&str] = &["q4_0", "q4_k_m", "q5_k_m", "q8_0", "f32"];
