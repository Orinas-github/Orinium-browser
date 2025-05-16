use std::{error::Error, fs, path::Path};

use reqwest::{
    header::{CONTENT_TYPE, USER_AGENT},
    Client,
};
use serde_json::Value;

/// `Net` 構造体は、HTTP リクエストを簡単に扱うためのユーティリティを提供します。
/// 内部的に `reqwest::Client` を使用しており、GET や POST リクエスト、
/// ファイル操作などの機能をサポートします。
#[allow(unused)]
pub struct Net {
    client: Client,
}

/// `Net` 構造体の実装。以下の機能を提供します:
///
/// - `new`: 新しい `Net` インスタンスを作成します。
/// - `get`: シンプルな GET リクエストを送信します。
/// - `get_with_headers`: ヘッダー付きの GET リクエストを送信します。
/// - `post_json`: JSON ボディを含む POST リクエストを送信します。
/// - `fetch_with_error_handling`: エラーハンドリングを強化した GET リクエストを送信します。
/// - `file_get_relative_path`: 相対パスを指定してファイルの内容を取得します。
#[allow(unused)]
impl Net {
    /// 新しい `Net` インスタンスを作成します。
    ///
    /// # 戻り値
    /// 新しい `Net` 構造体のインスタンス。
    pub fn new() -> Self {
        Net {
            client: Client::new(),
        }
    }

    /// シンプルな GET リクエストを送信します。
    ///
    /// # 引数
    /// - `url`: リクエストを送信する URL。
    ///
    /// # 戻り値
    /// 成功した場合はレスポンスボディの文字列を返します。
    /// エラーが発生した場合は `reqwest::Error` を返します。
    pub async fn get(&self, url: &str) -> Result<String, reqwest::Error> {
        let response = self.client.get(url).send().await?;
        let body = response.text().await?;
        Ok(body)
    }

    /// ヘッダー付きの GET リクエストを送信します。
    ///
    /// # 引数
    /// - `url`: リクエストを送信する URL。
    /// - `user_agent`: `User-Agent` ヘッダーに設定する文字列。
    ///
    /// # 戻り値
    /// 成功した場合はレスポンスボディの文字列を返します。
    /// エラーが発生した場合は `reqwest::Error` を返します。
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

    /// JSON ボディを含む POST リクエストを送信します。
    ///
    /// # 引数
    /// - `url`: リクエストを送信する URL。
    /// - `json_data`: リクエストボディとして送信する JSON データ。
    ///
    /// # 戻り値
    /// 成功した場合はレスポンスボディの文字列を返します。
    /// エラーが発生した場合は `reqwest::Error` を返します。
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

    /// エラーハンドリングを強化した GET リクエストを送信します。
    ///
    /// # 引数
    /// - `url`: リクエストを送信する URL。
    ///
    /// # 戻り値
    /// 成功した場合はレスポンスボディの文字列を返します。
    /// エラーが発生した場合はエラー内容を含む `Box<dyn Error>` を返します。
    pub async fn fetch_with_error_handling(&self, url: &str) -> Result<String, Box<dyn Error>> {
        let response = self.client.get(url).send().await?;

        if !response.status().is_success() {
            return Err(format!("HTTP error: {}", response.status()).into());
        }

        let body = response.text().await?;
        Ok(body)
    }

    /// 相対パスを指定してファイルの内容を取得します。
    ///
    /// # 引数
    /// - `relative_path`: 読み込むファイルの相対パス。
    ///
    /// # 戻り値
    /// ファイルの内容を文字列として返します。
    /// ファイルが存在しない場合は空文字列を返します。
    /// エラーが発生した場合はエラー内容を含む `Box<dyn Error>` を返します。
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
