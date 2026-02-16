use anyhow::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
struct HFRequest {
    inputs: String,
    parameters: HFParameters,
}

#[derive(Debug, Serialize)]
struct HFParameters {
    temperature: f32,
    top_p: f32,
    max_new_tokens: u32,
}

#[derive(Debug, Deserialize)]
struct HFResponse {
    generated_text: String,
}

pub struct HuggingFaceAPI {
    client: Client,
    token: String,
    model: String,
}

impl HuggingFaceAPI {
    pub fn new(token: String, org: String, model: String) -> Self {
        Self {
            client: Client::new(),
            token,
            model: format!("{}/{}", org, model),
        }
    }

    pub async fn generate(&self, prompt: &str, temperature: f32, top_p: f32) -> Result<String> {
        let url = format!("https://api-inference.huggingface.co/models/{}", self.model);

        let request = HFRequest {
            inputs: prompt.to_string(),
            parameters: HFParameters {
                temperature,
                top_p,
                max_new_tokens: 512,
            },
        };

        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.token))
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            anyhow::bail!("HuggingFace API error: {}", response.status());
        }

        let hf_response: Vec<HFResponse> = response.json().await?;

        if let Some(first) = hf_response.first() {
            Ok(first.generated_text.clone())
        } else {
            Ok(String::new())
        }
    }
}
