// telegram_client.rs

extern crate reqwest;

use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT};
use reqwest::{Client, Method};
use serde_json::json;

pub struct TelegramBot {
    token: String,
    base_url: String,
}

impl TelegramBot {
    pub fn new(token: &str) -> Self {
        let base_url = format!("https://api.telegram.org/bot{}/sendMessage", token);
        TelegramBot {
            token: token.to_string(),
            base_url,
        }
    }

    pub async fn send_message(&self, chat_id: &str, text: &str) -> Result<(&str), reqwest::Error> {
        let url = &self.base_url;

        let mut headers = HeaderMap::new();
        headers.insert(
            USER_AGENT,
            HeaderValue::from_static(
                "Telegram Bot SDK - (https://github.com/irazasyed/telegram-bot-sdk)",
            ),
        );
        headers.insert(
            reqwest::header::ACCEPT,
            HeaderValue::from_static("application/json"),
        );
        headers.insert(
            reqwest::header::CONTENT_TYPE,
            HeaderValue::from_static("application/json"),
        );

        let client = Client::new();
        let payload = json!({
            "text": text,
            "parse_mode": "Markdown",
            "disable_web_page_preview": false,
            "disable_notification": false,
            "chat_id": chat_id
        });

        println!("{}", payload);

        let response = client
            .post(url)
            .headers(headers)
            .json(&payload)
            .send()
            .await?;

        if response.status() != reqwest::StatusCode::OK {
            let error_body = response
                .text()
                .await
                .unwrap_or_else(|_| "Failed to retrieve error body".to_string());

            println!("error telegram {}", error_body);
        }

        println!("SENT TO TELEGRAM");

        Ok("Success")
    }
}
