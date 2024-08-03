use chrono::Datelike;
use chrono::Local;
use pinecone::{from_bytes, to_slice, to_vec};
use reqwest::header::{HeaderMap, HeaderValue, ACCEPT, CONTENT_TYPE};
use reqwest::Client;
use serde_json::json;
use std::error::Error;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::chatgpt_client::Daum;

use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct QueryResponse {
    pub results: Vec<Value>,
    pub matches: Option<Vec<Match>>,
    pub namespace: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Match {
    pub id: Option<String>,
    pub score: Option<f64>,
    pub values: Option<Vec<Value>>,
    pub metadata: Option<Metadata>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Metadata {
    #[serde(rename = "dayNumber")]
    pub day_number: String,
}

pub struct PineConeClient {
    client: Client,
    headers: HeaderMap,
    base_url: String,
}

impl PineConeClient {
    pub fn new(token: &str, base_url: String) -> Result<Self, Box<dyn std::error::Error>> {
        let client = Client::new();

        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        headers.insert(ACCEPT, HeaderValue::from_static("application/json"));
        headers.insert("Api-Key", HeaderValue::from_str(token)?);

        Ok(Self {
            client,
            headers,
            base_url,
        })
    }

    pub async fn save_vector(&self, embedding: &Vec<f64>) -> Result<(), Box<dyn Error>> {
        let start = SystemTime::now();

        let current_day = Local::now().day();
        let since_the_epoch = start.duration_since(UNIX_EPOCH).unwrap().as_millis();

        let data = json!({
            "vectors": [
                {
                    "values":  embedding,
                    "id": format!("{:?}", since_the_epoch),
                    "metadata": {
                        "dayNumber": format!("{:?}", current_day),
                      }
                },
            ]
        });

        let response = self
            .client
            .post(format!("{}vectors/upsert", &self.base_url))
            .headers(self.headers.clone())
            .json(&data)
            .send()
            .await?;

        println!("save_vector Response: {:?}", response.text().await?);

        Ok(())
    }

    pub async fn query(&self, query_vector: &Vec<f64>) -> Result<QueryResponse, Box<dyn Error>> {
        let current_day = Local::now().day();

        let data = json!({
          "filter": {
            "dayNumber": format!("{}", current_day)
          },
          "includeValues": "false",
          "includeMetadata": true,
          "vector": query_vector,
          "topK": 1
        });

        let response = self
            .client
            .post(format!("{}query", &self.base_url))
            .headers(self.headers.clone())
            .json(&data)
            .send()
            .await?;

        let query_response: QueryResponse = serde_json::from_slice(&response.bytes().await?)?;

        println!("{:?}", query_response);
        Ok(query_response)
    }
}
