use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Clone)]
pub struct LlmClient {
    client: Client,
    url: Arc<str>,
}

#[derive(Serialize)]
struct GenerateRequest<'a> {
    model: &'a str,
    prompt: &'a str,
    stream: bool,
    options: GenerateOptions,
}

#[derive(Serialize)]
struct GenerateOptions {
    temperature: f64,
    num_predict: u32,
}

#[derive(Deserialize)]
struct GenerateResponse {
    response: String,
}

#[derive(Serialize)]
struct EmbeddingRequest<'a> {
    model: &'a str,
    prompt: &'a str,
}

#[derive(Deserialize)]
struct EmbeddingResponse {
    embedding: Vec<f64>,
}


impl LlmClient {
    pub fn new(ollama_url: String) -> Self {
        Self {
            client: Client::new(),
            url: ollama_url.into(),
        }
    }

    pub async fn generate(&self, model: &str, prompt: &str, temperature: f64, max_tokens: u32) -> Result<String, reqwest::Error> {
        let request_url = format!("{}/api/generate", self.url);
        let request_body = GenerateRequest {
            model,
            prompt,
            stream: false,
            options: GenerateOptions {
                temperature,
                num_predict: max_tokens,
            },
        };

        let res = self
            .client
            .post(&request_url)
            .json(&request_body)
            .send()
            .await?;

        let response_body = res.json::<GenerateResponse>().await?;
        Ok(response_body.response)
    }

    pub async fn generate_embeddings(&self, model: &str, prompt: &str) -> Result<Vec<f64>, reqwest::Error> {
        let request_url = format!("{}/api/embeddings", self.url);
        let request_body = EmbeddingRequest {
            model,
            prompt,
        };

        let res = self
            .client
            .post(&request_url)
            .json(&request_body)
            .send()
            .await?;

        let response_body = res.json::<EmbeddingResponse>().await?;
        Ok(response_body.embedding)
    }

    pub async fn check_health(&self) -> Result<bool, reqwest::Error> {
        let request_url = format!("{}/api/tags", self.url);

        match self.client.get(&request_url).send().await {
            Ok(response) => Ok(response.status().is_success()),
            Err(_) => Ok(false),
        }
    }
}
