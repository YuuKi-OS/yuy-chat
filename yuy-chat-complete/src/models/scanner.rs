use crate::config::Config;
use anyhow::Result;
use std::path::PathBuf;
use walkdir::WalkDir;

#[derive(Debug, Clone)]
pub enum ModelSource {
    Local(PathBuf),
    HuggingFace { org: String, model: String },
}

#[derive(Debug, Clone)]
pub struct Model {
    pub name: String,
    pub path: PathBuf,
    pub source: ModelSource,
    pub format: ModelFormat,
    pub size: u64,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ModelFormat {
    GGUF,
    Llamafile,
}

impl Model {
    pub fn display_name(&self) -> String {
        format!("{} ({}) [{}]", 
            self.name,
            format_size(self.size),
            match &self.source {
                ModelSource::Local(_) => "Local",
                ModelSource::HuggingFace { .. } => "HuggingFace API",
            }
        )
    }

    pub fn format_name(&self) -> &str {
        match self.format {
            ModelFormat::GGUF => "GGUF",
            ModelFormat::Llamafile => "Llamafile",
        }
    }
}

pub struct ModelScanner;

impl ModelScanner {
    pub fn new() -> Self {
        Self
    }

    pub async fn scan_all(&self, config: &Config) -> Result<Vec<Model>> {
        let mut models = Vec::new();

        // Scan local models
        models.extend(self.scan_local(&config.models_dir)?);

        // Scan HuggingFace if token is available
        if let Some(token) = &config.hf_token {
            if let Ok(hf_models) = self.scan_huggingface(token).await {
                models.extend(hf_models);
            }
        }

        Ok(models)
    }

    fn scan_local(&self, models_dir: &PathBuf) -> Result<Vec<Model>> {
        let mut models = Vec::new();

        if !models_dir.exists() {
            return Ok(models);
        }

        for entry in WalkDir::new(models_dir)
            .max_depth(3)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();
            
            if !path.is_file() {
                continue;
            }

            let extension = path.extension().and_then(|s| s.to_str());
            
            let format = match extension {
                Some("gguf") => ModelFormat::GGUF,
                Some("llamafile") => ModelFormat::Llamafile,
                _ => continue,
            };

            let name = path
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("Unknown")
                .to_string();

            let size = path.metadata().map(|m| m.len()).unwrap_or(0);

            models.push(Model {
                name,
                path: path.to_path_buf(),
                source: ModelSource::Local(path.to_path_buf()),
                format,
                size,
            });
        }

        Ok(models)
    }

    async fn scan_huggingface(&self, _token: &str) -> Result<Vec<Model>> {
        // Placeholder for HuggingFace API integration
        // Would query API for available Yuuki models
        
        let hf_models = vec![
            Model {
                name: "Yuuki-best (HF API)".to_string(),
                path: PathBuf::from(""),
                source: ModelSource::HuggingFace {
                    org: "OpceanAI".to_string(),
                    model: "Yuuki-best".to_string(),
                },
                format: ModelFormat::GGUF,
                size: 0,
            },
        ];

        Ok(hf_models)
    }
}

fn format_size(bytes: u64) -> String {
    const GB: u64 = 1024 * 1024 * 1024;
    const MB: u64 = 1024 * 1024;

    if bytes >= GB {
        format!("{:.2} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.2} MB", bytes as f64 / MB as f64)
    } else {
        format!("{} B", bytes)
    }
}
