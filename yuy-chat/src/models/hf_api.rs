use anyhow::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use crate::config::YUUKI_API;

#[derive(Debug, Serialize)]
struct YuukiRequest {
    prompt: String,
    temperature: f32,
    top_p: f32,
    max_tokens: u32,
}

#[derive(Debug, Deserialize)]
struct YuukiResponse {
    response: String,
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
            model: format!("{}/{}", org, model),
        }
    }

    pub async fn generate(&self, prompt: &str, temperature: f32, top_p: f32) -> Result<String> {
        // Use Yuuki API endpoint
        let url = YUUKI_API;

        let request = YuukiRequest {
            prompt: prompt.to_string(),
            temperature,
            top_p,
            max_tokens: 512,
        };

        let mut req = self.client.post(url).json(&request);

        // Add token if available (optional for public API)
        if let Some(token) = &self.token {
            req = req.header("Authorization", format!("Bearer {}", token));
        }

        let response = req.send().await?;

        if !response.status().is_success() {
            anyhow::bail!("Yuuki API error: {}", response.status());
        }

        let yuuki_response: YuukiResponse = response.json().await?;
        Ok(yuuki_response.response)
    }
}
