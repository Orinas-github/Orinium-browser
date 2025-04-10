use reqwest::{
    header::{CONTENT_TYPE, USER_AGENT},
    Client,
};
use serde_json::Value;
use std::error::Error;
use std::fs;
use std::path::Path;

#[allow(unused)]
pub struct Net {
    client: Client,
}


// 今後使いそうだからunused allowしとく
#[allow(unused)]
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
    pub async fn get_with_headers(
        &self,
        url: &str,
        user_agent: &str,
    ) -> Result<String, reqwest::Error> {
        let response = self
            .client
            .get(url)
            .header(USER_AGENT, user_agent)
            .send()
            .await?;
        let body = response.text().await?;
        Ok(body)
    }

    // 3. POST リクエスト (JSON ボディ)
    pub async fn post_json(&self, url: &str, json_data: &Value) -> Result<String, reqwest::Error> {
        let response = self
            .client
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

    // 5. 相対ファイル取得
    pub fn file_get_relative_path(&self, relative_path: &str) -> Result<String, Box<dyn Error>> {
        let file_path = Path::new(relative_path);

        // ファイルが存在するか確認
        if !file_path.exists() {
            return Ok("".to_string()); // 存在しない
        }

        // ファイルの内容を読み込む
        let contents = fs::read_to_string(file_path)?;

        Ok(contents)
    }
}
