use anyhow::Result;
use std::sync::Arc;
use std::time::SystemTime;
use tokio::io::{self, AsyncBufReadExt, AsyncRead, AsyncReadExt, AsyncWriteExt, BufReader};
use tokio::sync::RwLock;
use url::Url;

use crate::platform::network::{
    cache::Cache,
    config::NetworkConfig,
    connection_pool::{Connection, ConnectionPool, HostKey},
    cookie_store::CookieStore,
    tcp::TcpConnection,
};

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Response {
    pub http_version: String,
    pub status_code: u16,
    pub reason_phrase: String,
    pub headers: Vec<(String, String)>,
    pub body: Vec<u8>,
}

#[derive(Debug)]
#[allow(dead_code)]
struct CacheEntry {
    response: Response,
    cached_at: SystemTime,
    expires_at: Option<SystemTime>,
}

#[derive(Debug)]
pub struct NetworkCore {
    pub config: Arc<RwLock<NetworkConfig>>,
    pub connection_pool: ConnectionPool,
    pub cookie_store: CookieStore,
    pub cache: Cache,
}

#[allow(dead_code)]
impl NetworkCore {
    pub fn new() -> Result<Self> {
        let config = NetworkConfig::default();
        Ok(Self {
            config: Arc::new(RwLock::new(config)),
            connection_pool: ConnectionPool::new(),
            cookie_store: CookieStore::new(),
            cache: Cache::new(),
        })
    }

    /// 汎用 HTTP リクエスト関数
    async fn send_request(
        &self,
        method: &str,
        url: &Url,
        extra_headers: Vec<(String, String)>,
        body: Option<Vec<u8>>,
        use_cache: bool,
    ) -> Result<Response> {
        let host = url
            .host_str()
            .ok_or_else(|| anyhow::anyhow!("Invalid host"))?
            .to_string();
        let port = url.port_or_known_default().unwrap_or(80);
        let key = HostKey {
            scheme: url.scheme().to_string(),
            host: host.clone(),
            port,
        };

        // GET キャッシュチェック
        if use_cache {
            if let Some(cached) = self.cache.get(url).await {
                return Ok(Response {
                    http_version: "HTTP/1.1".to_string(),
                    status_code: 200,
                    reason_phrase: "OK (cache)".to_string(),
                    headers: cached.headers,
                    body: cached.body,
                });
            }
        }

        // Connection取得
        let mut conn = match self.connection_pool.get_connection(&key).await {
            Some(c) => c,
            None => {
                let cfg = self.config.read().await;
                if url.scheme() == "https" {
                    Connection::Tls(crate::platform::network::tls::TlsConnection::connect(&host, port, cfg.connect_timeout).await?)
                } else {
                    Connection::Tcp(TcpConnection::connect(&host, port, cfg.connect_timeout).await?)
                }
            }
        };

        // Cookie
        let cookie_header = self.cookie_store.get_cookie_header(url).await;

        // ヘッダ作成
        let mut request = format!(
            "{} {} HTTP/1.1\r\nHost: {}\r\nConnection: keep-alive\r\nUser-Agent: {}\r\n",
            method,
            url.path(),
            host,
            self.config.read().await.user_agent
        );

        for (k, v) in extra_headers {
            request.push_str(&format!("{k}: {v}\r\n"));
        }

        if let Some(cookie) = cookie_header {
            request.push_str(&format!("Cookie: {cookie}\r\n"));
        }

        if let Some(ref b) = body {
            request.push_str(&format!("Content-Length: {}\r\n", b.len()));
        }
        request.push_str("\r\n"); // ヘッダ終端

        // 送信 & レスポンス受信
        let (headers, body) = match &mut conn {
            Connection::Tcp(c) => {
                c.stream.write_all(request.as_bytes()).await?;
                if let Some(body_bytes) = body.clone() {
                    c.stream.write_all(&body_bytes).await?;
                }
                let (body_start, headers) = Self::read_headers(&mut c.stream).await?;
                let content_length = headers
                    .iter()
                    .find(|(k, _)| k.to_lowercase() == "content-length")
                    .and_then(|(_, v)| v.parse::<usize>().ok())
                    .unwrap_or(0);
                let body = Self::read_body(&mut c.stream, body_start, content_length).await?;
                (headers, body)
            },
            Connection::Tls(c) => {
                c.stream.write_all(request.as_bytes()).await?;
                if let Some(body_bytes) = body.clone() {
                    c.stream.write_all(&body_bytes).await?;
                }
                let (body_start, headers) = Self::read_headers(&mut c.stream).await?;
                let content_length = headers
                    .iter()
                    .find(|(k, _)| k.to_lowercase() == "content-length")
                    .and_then(|(_, v)| v.parse::<usize>().ok())
                    .unwrap_or(0);
                let body = Self::read_body(&mut c.stream, body_start, content_length).await?;
                (headers, body)
            }
        };

        // ステータス行パース
        let status_line = headers
            .iter()
            .find(|(k, _)| k == "Status-Line")
            .map(|(_, v)| v.as_str())
            .unwrap_or("");
        let http_version = status_line
            .split_whitespace()
            .next()
            .unwrap_or("HTTP/1.1")
            .to_string();
        let status_code = status_line
            .split_whitespace()
            .nth(1)
            .and_then(|s| s.parse::<u16>().ok())
            .unwrap_or(200);
        let reason_phrase = status_line.splitn(3, ' ').nth(2).unwrap_or("").to_string();

        // Cookie 保存
        let set_cookie_headers = headers
            .iter()
            .filter(|(k, _)| k.to_lowercase() == "set-cookie")
            .map(|(_, v)| v.clone())
            .collect::<Vec<_>>();
        self.cookie_store
            .set_cookies(url, &set_cookie_headers)
            .await;

        // Cache 保存（GETのみ）
        if use_cache {
            self.cache.set(url, body.clone(), headers.clone()).await;
        }

        // Connection プールに戻す
        self.connection_pool.add_connection(key, conn).await;

        Ok(Response {
            http_version,
            status_code,
            reason_phrase,
            headers,
            body,
        })
    }

    /// GET（キャッシュあり）
    pub async fn fetch(&self, url: &str) -> Result<Response> {
        let url = Url::parse(url)?;
        self.send_request("GET", &url, vec![], None, true).await
    }

    /// POST（キャッシュなし）
    pub async fn post(&self, url: &str, body: Vec<u8>, content_type: &str) -> Result<Response> {
        let url = Url::parse(url)?;
        let headers = vec![("Content-Type".to_string(), content_type.to_string())];
        self.send_request("POST", &url, headers, Some(body), false)
            .await
    }

    async fn read_headers<R>(stream: &mut R) -> io::Result<(Vec<u8>, Vec<(String, String)>)>
    where
        R: AsyncRead + Unpin,
    {
        let mut reader = BufReader::new(stream);
        let mut headers = Vec::new();
        let mut line = String::new();

        loop {
            line.clear();
            let n = reader.read_line(&mut line).await?;
            if n == 0 || line == "\r\n" {
                break;
            }
            if let Some((k, v)) = line.split_once(':') {
                headers.push((k.trim().to_string(), v.trim().to_string()));
            } else if line.starts_with("HTTP/") {
                headers.push(("Status-Line".to_string(), line.trim().to_string()));
            }
        }

        let body_start = reader.buffer().to_vec();
        Ok((body_start, headers))
    }

    async fn read_body<R>(
        stream: &mut R,
        body_start: Vec<u8>,
        content_length: usize,
    ) -> anyhow::Result<Vec<u8>>
    where
        R: AsyncRead + Unpin,
    {
        let mut body = Vec::with_capacity(content_length);
        body.extend_from_slice(&body_start);
        while body.len() < content_length {
            let mut tmp = [0u8; 1024];
            let n = stream.read(&mut tmp).await?;
            if n == 0 {
                break;
            }
            body.extend_from_slice(&tmp[..n]);
        }
        Ok(body)
    }
}
