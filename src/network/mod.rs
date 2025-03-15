use reqwest::{Client, header::{USER_AGENT, CONTENT_TYPE}};
use serde_json::Value;
use std::error::Error;

pub struct Net {
    client: Client,
}

impl Net {
    pub fn new() -> Self {
        Net {
            client: Client::new(),
        }
    }

    // 1. シンプルな GET リクエスト
    pub async fn get(&self, url: &str) -> Result<String, reqwest::Error> {
        let response = self.client.get(url).send().await?;
        let body = response.text().await?;
        Ok(body)
    }

    // 2. ヘッダー付き GET リクエスト
    pub async fn get_with_headers(&self, url: &str, user_agent: &str) -> Result<String, reqwest::Error> {
        let response = self.client
            .get(url)
            .header(USER_AGENT, user_agent)
            .send()
            .await?;
        let body = response.text().await?;
        Ok(body)
    }

    // 3. POST リクエスト (JSON ボディ)
    pub async fn post_json(&self, url: &str, json_data: &Value) -> Result<String, reqwest::Error> {
        let response = self.client
            .post(url)
            .header(CONTENT_TYPE, "application/json")
            .json(json_data)
            .send()
            .await?;
        let body = response.text().await?;
        Ok(body)
    }

    // 4. エラーハンドリング強化
    pub async fn fetch_with_error_handling(&self, url: &str) -> Result<String, Box<dyn Error>> {
        let response = self.client.get(url).send().await?;

        if !response.status().is_success() {
            return Err(format!("HTTP error: {}", response.status()).into());
        }

        let body = response.text().await?;
        Ok(body)
    }
}