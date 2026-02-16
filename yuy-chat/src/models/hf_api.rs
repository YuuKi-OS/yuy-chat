use anyhow::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use crate::config::YUUKI_API;

#[derive(Debug, Serialize)]
struct YuukiRequest {
    prompt: String,
    model: String,
    max_new_tokens: u32,
    temperature: f32,
    top_p: f32,
}

#[derive(Debug, Deserialize)]
struct YuukiResponse {
    response: String,
    model: String,
    tokens_generated: u32,
    time_ms: u32,
}

pub struct HuggingFaceAPI {
    client: Client,
    token: Option<String>,
    model: String,
}

impl HuggingFaceAPI {
    pub fn new(token: String, org: String, model: String) -> Self {
        Self {
            client: Client::new(),
            token: Some(token),
            model: model.to_lowercase(), // yuuki-best, yuuki-3.7, etc.
        }
    }

    pub async fn generate(&self, prompt: &str, temperature: f32, top_p: f32) -> Result<String> {
        // Yuuki API endpoint: https://huggingface.co/spaces/OpceanAI/Yuuki-api/generate
        let url = format!("{}/generate", YUUKI_API);

        let request = YuukiRequest {
            prompt: prompt.to_string(),
            model: self.model.clone(),
            max_new_tokens: 120,
            temperature,
            top_p,
        };

        let response = self.client
            .post(&url)
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            anyhow::bail!("Yuuki API error {}: {}", status, error_text);
        }

        let yuuki_response: YuukiResponse = response.json().await?;
        Ok(yuuki_response.response)
    }
}
