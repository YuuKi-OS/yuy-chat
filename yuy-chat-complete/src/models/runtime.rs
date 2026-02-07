use super::{Model, ModelFormat, ModelSource};
use crate::config::Preset;
use anyhow::{Context, Result};
use std::process::Stdio;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::{Child, Command};
use tokio::sync::mpsc;

pub struct ModelRuntime {
    model: Model,
    preset: Preset,
    process: Option<Child>,
    response_rx: Option<mpsc::Receiver<String>>,
}

impl ModelRuntime {
    pub async fn new(model: Model, preset: Preset) -> Result<Self> {
        Ok(Self {
            model,
            preset,
            process: None,
            response_rx: None,
        })
    }

    pub async fn generate(&mut self, prompt: &str) -> Result<()> {
        match &self.model.source {
            ModelSource::Local(_) => self.generate_local(prompt).await,
            ModelSource::HuggingFace { .. } => self.generate_hf(prompt).await,
        }
    }

    async fn generate_local(&mut self, prompt: &str) -> Result<()> {
        let command = match self.model.format {
            ModelFormat::GGUF => self.build_llama_cpp_command(prompt)?,
            ModelFormat::Llamafile => self.build_llamafile_command(prompt)?,
        };

        let (tx, rx) = mpsc::channel(100);
        self.response_rx = Some(rx);

        let prompt_owned = prompt.to_string();
        
        tokio::spawn(async move {
            if let Ok(mut child) = command.spawn() {
                if let Some(stdout) = child.stdout.take() {
                    let reader = BufReader::new(stdout);
                    let mut lines = reader.lines();

                    while let Ok(Some(line)) = lines.next_line().await {
                        if tx.send(line).await.is_err() {
                            break;
                        }
                    }
                }

                let _ = child.wait().await;
                let _ = tx.send("[DONE]".to_string()).await;
            }
        });

        Ok(())
    }

    fn build_llama_cpp_command(&self, prompt: &str) -> Result<Command> {
        let llama_cmd = self.find_llama_binary()?;
        
        let mut cmd = Command::new(llama_cmd);
        cmd.arg("-m")
            .arg(&self.model.path)
            .arg("--temp")
            .arg(self.preset.temperature().to_string())
            .arg("--top-p")
            .arg(self.preset.top_p().to_string())
            .arg("-c")
            .arg("4096")
            .arg("-p")
            .arg(prompt)
            .stdout(Stdio::piped())
            .stderr(Stdio::null());

        Ok(cmd)
    }

    fn build_llamafile_command(&self, prompt: &str) -> Result<Command> {
        let mut cmd = Command::new(&self.model.path);
        cmd.arg("--temp")
            .arg(self.preset.temperature().to_string())
            .arg("--top-p")
            .arg(self.preset.top_p().to_string())
            .arg("-c")
            .arg("4096")
            .arg("-p")
            .arg(prompt)
            .stdout(Stdio::piped())
            .stderr(Stdio::null());

        Ok(cmd)
    }

    fn find_llama_binary(&self) -> Result<String> {
        for binary in &["llama-cli", "llama", "main"] {
            if which::which(binary).is_ok() {
                return Ok(binary.to_string());
            }
        }

        anyhow::bail!("llama.cpp binary not found. Install with: yuy runtime install llama-cpp")
    }

    async fn generate_hf(&mut self, prompt: &str) -> Result<()> {
        // Placeholder for HuggingFace API call
        let (tx, rx) = mpsc::channel(100);
        self.response_rx = Some(rx);

        let prompt_owned = prompt.to_string();
        
        tokio::spawn(async move {
            // Simulated streaming response
            let response = format!("Response to: {}", prompt_owned);
            for word in response.split_whitespace() {
                let _ = tx.send(format!("{} ", word)).await;
                tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
            }
            let _ = tx.send("[DONE]".to_string()).await;
        });

        Ok(())
    }

    pub async fn poll_chunk(&mut self) -> Result<Option<String>> {
        if let Some(rx) = &mut self.response_rx {
            Ok(rx.recv().await)
        } else {
            Ok(None)
        }
    }
}

impl Drop for ModelRuntime {
    fn drop(&mut self) {
        if let Some(mut process) = self.process.take() {
            let _ = process.start_kill();
        }
    }
}
