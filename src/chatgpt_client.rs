use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::error::Error;

#[derive(Serialize)]
pub struct CompletionRequest {
    pub model: String,
    pub messages: Vec<Message>,
    pub temperature: f32,
    pub max_tokens: usize,
    pub top_p: f32,
    pub frequency_penalty: f32,
    pub presence_penalty: f32,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CompletionResponse {
    pub choices: Vec<Choice>,
    pub created: i64,
    pub id: String,
    pub model: String,
    pub object: String,
    pub usage: Usage,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Choice {
    pub finish_reason: String,
    pub index: i64,
    pub message: Message,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Message {
    pub content: String,
    pub role: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Usage {
    pub completion_tokens: i64,
    pub prompt_tokens: i64,
    pub total_tokens: i64,
}

pub struct OpenAI {
    client: Client,
    api_key: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EmbeddingsResponse {
    pub data: Vec<Daum>,
    pub model: String,
    pub object: String,
    pub usage: EmbeddingsUsage,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Daum {
    pub embedding: Vec<f64>,
    pub index: i64,
    pub object: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EmbeddingsUsage {
    pub prompt_tokens: i64,
    pub total_tokens: i64,
}

impl OpenAI {
    pub fn new(api_key: &str) -> Self {
        OpenAI {
            client: Client::new(),
            api_key: api_key.to_string(),
        }
    }

    pub async fn get_completion(
        &self,
        req: &CompletionRequest,
    ) -> Result<CompletionResponse, Box<dyn Error>> {
        let url = "https://api.openai.com/v1/chat/completions";
        let response = self
            .client
            .post(url)
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", &self.api_key))
            .json(req)
            .send()
            .await?;
        let status = response.status(); // Clone the status here

        println!("url {:?} status  {:?}", url.to_string(), response.status());

        if response.status() != reqwest::StatusCode::OK {
            let error_body = response
                .text()
                .await
                .unwrap_or_else(|_| "Failed to retrieve error body".to_string());

            return Err(format!(
                "Failed with HTTP Status: {}. Response body: {}",
                status, error_body
            )
            .into());
        }

        Ok(response.json::<CompletionResponse>().await?)
    }

    pub async fn get_embedding(&self, text: &str) -> Result<EmbeddingsResponse, Box<dyn Error>> {
        let url = "https://api.openai.com/v1/embeddings";
        let response = self
            .client
            .post(url)
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", &self.api_key))
            .json(&json!({
                "input": format!("{:?}", text),
                "model": "text-embedding-ada-002"
            }))
            .send()
            .await?;

        Ok(response.json::<EmbeddingsResponse>().await?)
    }
}
