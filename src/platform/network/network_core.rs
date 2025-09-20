use std::sync::Arc;
use std::time::{SystemTime};
use tokio::sync::RwLock;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use url::Url;
use anyhow::{Result};

#[allow(unused_imports)]
use crate::platform::network::{
    config::NetworkConfig,
    connection_pool::{ConnectionPool, Connection, HostKey},
    tcp::TcpConnection,
//     tls::TlsConnection,
    cookie_store::CookieStore,
    cache::{Cache, CachedResponse},
};

///   - `http_version` : HTTP バージョン文字列（例: "HTTP/1.1"）
///   - `status_code` : HTTP ステータスコード（例: 200, 404）
///   - `reason_phrase` : ステータス理由句（例: "OK", "Not Found"）
///   - `headers` : ヘッダのリスト (`Vec<(String, String)>`)  
///   - `body` : レスポンス本文 (`Vec<u8>`)
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

/// Cookie 動作について、
/// - 対象 URL のドメインに保存済みの Cookie を自動付与
/// - レスポンスに `Set-Cookie` があれば CookieStore に保存
/// 
/// 接続プールについて、
/// - 同一ホスト・ポート・スキームの接続がプールに存在すれば再利用
/// - 新規接続が必要な場合は TCP または TLS 接続を作成
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

    /// 指定した URL から HTTP GET リクエストを送信し、レスポンスを取得
    ///
    /// # 概要
    /// - 内部でキャッシュを確認し、有効なキャッシュがあればそれを返
    /// - ConnectionPoolから既存の TCP/TLS 接続を再利用
    /// - CookieStore を利用して対象 URL に対応する Cookie ヘッダを自動付与
    /// - レスポンス取得後、Set-Cookie ヘッダがあれば CookieStore に保存
    /// - レスポンスを Cache に保存し、次回の GET で再利用可能に
    ///
    /// # パラメータ
    /// - `url`: 取得対象の URL 文字列。HTTP または HTTPS スキームをサポート
    ///
    /// # 戻り値
    /// - 成功時: `Response` 構造体
    /// - 失敗時: `anyhow::Result`
    ///
    /// # キャッシュ動作
    /// - キャッシュが存在し、有効期限内の場合はキャッシュから直接レスポンスを返
    /// - レスポンスの `Cache-Control: max-age` または `Expires` ヘッダを元に TTL を計算しキャッシュ
    pub async fn fetch(&self, url: &str) -> Result<Response> {
        let url = Url::parse(url)?;
        if let Some(cached) = self.cache.get(&url).await {
            return Ok(Response {
                http_version: "HTTP/1.1".to_string(),
                status_code: 200,
                reason_phrase: "OK (cache)".to_string(),
                headers: cached.headers,
                body: cached.body,
            });
        }

        let host = url.host_str().ok_or_else(|| anyhow::anyhow!("Invalid host"))?.to_string();
        let port = url.port_or_known_default().unwrap_or(80);
        let key = HostKey { scheme: url.scheme().to_string(), host: host.clone(), port };

        // Connection取得 or 新規作成
        let mut conn = match self.connection_pool.get_connection(&key).await {
            Some(c) => c,
            None => {
                log::debug!("Creating new connection to {}:{}", host, port);
                let cfg = self.config.read().await;
                if url.scheme() == "https" {
                    log::warn!("TLS not supported yet");
                    anyhow::bail!("TLS not supported yet");
                    /*
                    let tcp = TcpConnection::connect(&host, port, cfg.connect_timeout).await?;
                    Connection::Tls(TlsConnection::connect(tcp, &host, cfg.verify_tls).await?)
                    */
                } else {
                    Connection::Tcp(TcpConnection::connect(&host, port, cfg.connect_timeout).await?)
                }
            }
        };

        // Cookie ヘッダ
        let cookie_header = self.cookie_store.get_cookie_header(&url).await;
        let mut request = format!(
            "GET {} HTTP/1.1\r\nHost: {}\r\nConnection: keep-alive\r\n",
            url.path(),
            host
        );
        if let Some(cookie) = cookie_header {
            request.push_str(&format!("Cookie: {}\r\n", cookie));
        }
        request.push_str("\r\n");

        let mut buf = Vec::new();
        // 実際にリクエスト送信・レスポンス受信
        match &mut conn {
            Connection::Tcp(c) => {
                log::debug!("Request:\n{}", request);
                c.stream.write_all(request.as_bytes()).await?;
                log::debug!("Request sent, waiting for response...");
                loop {
                    let mut tmp = [0u8; 1024];
                    let n = c.stream.read(&mut tmp).await?;
                    if n == 0 {
                        break;
                    }
                    buf.extend_from_slice(&tmp[..n]);
                    if let Some(_) = twoway::find_bytes(&buf, b"\r\n\r\n") {
                        break;
                    }
                }
                log::debug!("Response received, {} bytes", buf.len());
            }
            /*
            Connection::Tls(c) => {
                c.stream.write_all(request.as_bytes()).await?;
                c.stream.read_to_end(&mut buf).await?;
            }
            */
        }

        // ヘッダとボディ分離
        let text = String::from_utf8_lossy(&buf);
        let mut lines = text.lines();
        let status_line = lines.next().unwrap_or("");
        let mut headers = Vec::new();
        let mut body_start = 0;

        // ステータス行のパース
        let mut http_version = "HTTP/1.1".to_string();
        let mut status_code = 0;
        let mut reason_phrase = String::new();
        if let Some((version, rest)) = status_line.split_once(' ') {
            http_version = version.to_string();
            if let Some((code_str, reason)) = rest.split_once(' ') {
                status_code = code_str.parse::<u16>().unwrap_or(0);
                reason_phrase = reason.to_string();
            }
        }

        for (i, line) in text.lines().enumerate() {
            if line.is_empty() {
                body_start = i + 1;
                break;
            }
            if let Some((k, v)) = line.split_once(':') {
                headers.push((k.trim().to_string(), v.trim().to_string()));
            }
        }

        let body = lines.skip(body_start).collect::<Vec<&str>>().join("\n").into_bytes();

        // Cookie 保存
        let set_cookie_headers = headers.iter()
            .filter(|(k, _)| k.to_lowercase() == "set-cookie")
            .map(|(_, v)| v.clone())
            .collect::<Vec<_>>();
        self.cookie_store.set_cookies(&url, &set_cookie_headers).await;

        // Cache 保存
        self.cache.set(&url, body.clone(), headers.clone()).await;

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

    /// 指定した URL に対して HTTP POST リクエストを送信
    ///
    /// # 概要
    /// - ConnectionPoolから TCP/TLS 接続を再利用
    /// - CookieStore を利用して対象 URL に対応する Cookie ヘッダを自動付与
    /// - レスポンス取得後、Set-Cookie ヘッダがあれば CookieStore に保存
    /// - レスポンスは Cache に保存されない（GET リクエストのみキャッシュ対象）
    ///
    /// # パラメータ
    /// - `url`: 送信先の URL 文字列。HTTP または HTTPS スキームをサポート
    /// - `body`: POST ボディのバイト列
    /// - `content_type`: Content-Type ヘッダに設定する文字列（例: "application/json"）
    ///
    /// # 戻り値
    /// - 成功時: `Response` 構造体
    /// - 失敗時: `anyhow::Result`
    pub async fn post(&self, url: &str, body: Vec<u8>, content_type: &str) -> Result<Response> {
        let url = Url::parse(url)?;
        let host = url.host_str().ok_or_else(|| anyhow::anyhow!("Invalid host"))?.to_string();
        let port = url.port_or_known_default().unwrap_or(80);
        let key = HostKey { scheme: url.scheme().to_string(), host: host.clone(), port };

        // Connection取得 or 新規作成
        let mut conn = match self.connection_pool.get_connection(&key).await {
            Some(c) => c,
            None => {
                let cfg = self.config.read().await;
                if url.scheme() == "https" {
                    log::warn!("TLS not supported yet");
                    anyhow::bail!("TLS not supported yet");
                    /*
                    let tcp = TcpConnection::connect(&host, port, cfg.connect_timeout).await?;
                    Connection::Tls(TlsConnection::connect(tcp, &host, cfg.verify_tls).await?)
                    */
                } else {
                    Connection::Tcp(TcpConnection::connect(&host, port, cfg.connect_timeout).await?)
                }
            }
        };

        // Cookie ヘッダ
        let cookie_header = self.cookie_store.get_cookie_header(&url).await;
        let mut request = format!(
            "POST {} HTTP/1.1\r\nHost: {}\r\nConnection: keep-alive\r\nContent-Type: {}\r\nContent-Length: {}\r\n",
            url.path(),
            host,
            content_type,
            body.len()
        );
        if let Some(cookie) = cookie_header {
            request.push_str(&format!("Cookie: {}\r\n", cookie));
        }
        request.push_str("\r\n");

        let mut buf = Vec::new();
        match &mut conn {
            Connection::Tcp(c) => {
                c.stream.write_all(request.as_bytes()).await?;
                c.stream.write_all(&body).await?;
                loop {
                    let mut tmp = [0u8; 1024];
                    let n = c.stream.read(&mut tmp).await?;
                    if n == 0 {
                        break;
                    }
                    buf.extend_from_slice(&tmp[..n]);
                    if let Some(_) = twoway::find_bytes(&buf, b"\r\n\r\n") {
                        break;
                    }
                }
            }
            /*
            Connection::Tls(c) => {
                c.stream.write_all(request.as_bytes()).await?;
                c.stream.write_all(&body).await?;
                c.stream.read_to_end(&mut buf).await?;
            }
            */
        }

        // ヘッダとボディ分離
        let text = String::from_utf8_lossy(&buf);
        let mut lines = text.lines();
        let status_line = lines.next().unwrap_or("");
        let mut headers = Vec::new();
        let mut body_start = 0;

        // ステータス行のパース
        let mut http_version = "HTTP/1.1".to_string();
        let mut status_code = 0;
        let mut reason_phrase = String::new();
        if let Some((version, rest)) = status_line.split_once(' ') {
            http_version = version.to_string();
            if let Some((code_str, reason)) = rest.split_once(' ') {
                status_code = code_str.parse::<u16>().unwrap_or(0);
                reason_phrase = reason.to_string();
            }
        }

        for (i, line) in text.lines().enumerate() {
            if line.is_empty() {
                body_start = i + 1;
                break;
            }
            if let Some((k, v)) = line.split_once(':') {
                headers.push((k.trim().to_string(), v.trim().to_string()));
            }
        }

        let body_bytes = lines.skip(body_start).collect::<Vec<&str>>().join("\n").into_bytes();

        // Cookie 保存
        let set_cookie_headers = headers.iter()
            .filter(|(k, _)| k.to_lowercase() == "set-cookie")
            .map(|(_, v)| v.clone())
            .collect::<Vec<_>>();
        self.cookie_store.set_cookies(&url, &set_cookie_headers).await;

        // Connection プールに戻す
        self.connection_pool.add_connection(key, conn).await;

        Ok(Response {
            http_version,
            status_code,
            reason_phrase,
            headers,
            body: body_bytes,
        })
    }
}
