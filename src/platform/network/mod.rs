use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, SystemTime};

use anyhow::{Context, Result};
use reqwest::{Client, ClientBuilder, Method, StatusCode, Url};
use tokio::sync::RwLock;

/* TLS signeture module */
pub mod sig;


#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct RequestConfig {
    pub timeout_ms: u64,
    pub follow_redirects: bool,
    pub max_redirects: u32,
    pub headers: HashMap<String, String>,
}

// Default Condig
impl Default for RequestConfig {
    fn default() -> Self {
        Self {
            timeout_ms: 30000, // 30 seconds
            follow_redirects: true,
            max_redirects: 10,
            headers: HashMap::new(),
        }
    }
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Response {
    pub status: StatusCode,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>,
    pub url: Url,
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
    client: Client,
    cache: Arc<RwLock<HashMap<String, CacheEntry>>>,
    default_config: RequestConfig,
}

// NetworkCore implementation
#[allow(dead_code)]
impl NetworkCore {
    pub fn new() -> Result<Self> {
        let client = ClientBuilder::new()
            .user_agent(format!(
                "OrinionBrowser/0.1 (+https://github.com/Orinas-github/Orinium-browser)"
            ))
            .gzip(true)
            .brotli(true)
            .deflate(true)
            .build()
            .context("Failed to create HTTP client")?;

        Ok(Self {
            client,
            cache: Arc::new(RwLock::new(HashMap::new())),
            default_config: RequestConfig::default(),
        })
    }

    pub fn set_default_config(&mut self, config: RequestConfig) {
        self.default_config = config;
    }

    pub async fn fetch(&self, url: &str) -> Result<Response> {
        self.fetch_with_config(url, &self.default_config).await
    }

    pub async fn fetch_with_config(&self, url: &str, config: &RequestConfig) -> Result<Response> {
        let url_obj = Url::parse(url).context("Invalid URL")?;
        
        if let Some(cached) = self.get_from_cache(&url_obj).await {
            log::info!("Resource retrieved from cache: {}", url);
            return Ok(cached);
        }

        let mut request_builder = self.client.request(Method::GET, url_obj.clone());
        request_builder = request_builder.timeout(Duration::from_millis(config.timeout_ms));
        for (name, value) in &config.headers {
            request_builder = request_builder.header(name, value);
        }

        log::info!("Starting to fetch resource: {}", url);
        let response = request_builder
            .send()
            .await
            .context(format!("Request failed: {}", url))?;
        
        let status = response.status();
        let mut headers = HashMap::new();
        for (name, value) in response.headers() {
            if let Ok(value_str) = value.to_str() {
                headers.insert(name.as_str().to_string(), value_str.to_string());
            }
        }
        
        // Read the body
        let body = response
            .bytes()
            .await
            .context("Failed to get response body")?
            .to_vec();
        
        let response_obj = Response {
            status,
            headers,
            body,
            url: url_obj.clone(),
        };
        
        if status.is_success() {
            self.add_to_cache(url_obj, response_obj.clone()).await?;
        }
        
        Ok(response_obj)
    }

    pub async fn post(&self, url: &str, body: Vec<u8>, content_type: &str) -> Result<Response> {
        let url_obj = Url::parse(url).context("Invalid URL")?;
        let mut request_builder = self.client
            .request(Method::POST, url_obj.clone())
            .timeout(Duration::from_millis(self.default_config.timeout_ms))
            .body(body);
        request_builder = request_builder.header("Content-Type", content_type);
        for (name, value) in &self.default_config.headers {
            request_builder = request_builder.header(name, value);
        }

        log::info!("Sending POST request: {}", url);
        let response = request_builder
            .send()
            .await
            .context(format!("POST request failed: {}", url))?;
        
        let status = response.status();
        let mut headers = HashMap::new();
        for (name, value) in response.headers() {
            if let Ok(value_str) = value.to_str() {
                headers.insert(name.as_str().to_string(), value_str.to_string());
            }
        }

        let body = response
            .bytes()
            .await
            .context("Failed to get response body")?
            .to_vec();
        
        Ok(Response {
            status,
            headers,
            body,
            url: url_obj,
        })
    }

    async fn get_from_cache(&self, url: &Url) -> Option<Response> {
        let cache = self.cache.read().await;
        let key = url.as_str();
        
        if let Some(entry) = cache.get(key) {
            if let Some(expires_at) = entry.expires_at {
                if SystemTime::now() > expires_at {
                    log::debug!("Cache expired: {}", key);
                    return None;
                }
            }
            log::debug!("Cache hit: {}", key);
            return Some(entry.response.clone());
        }
        log::debug!("Cache miss: {}", key);
        None
    }

    async fn add_to_cache(&self, url: Url, response: Response) -> Result<()> {
        let mut cache = self.cache.write().await;
        let key = url.as_str().to_string();
        let mut expires_at = None;
        if let Some(cache_control) = response.headers.get("cache-control") {
            if let Some(max_age_pos) = cache_control.find("max-age=") {
                let max_age_str = &cache_control[max_age_pos + 8..];
                if let Some(end_pos) = max_age_str.find(|c: char| !c.is_ascii_digit()) {
                    if let Ok(max_age) = max_age_str[..end_pos].parse::<u64>() {
                        expires_at = Some(SystemTime::now() + Duration::from_secs(max_age));
                    }
                } else if let Ok(max_age) = max_age_str.parse::<u64>() {
                    expires_at = Some(SystemTime::now() + Duration::from_secs(max_age));
                }
            }
        }
        if expires_at.is_none() {
            if response.headers.contains_key("expires") {
                expires_at = Some(SystemTime::now() + Duration::from_secs(3600));
            }
        }
        cache.insert(
            key,
            CacheEntry {
                response,
                cached_at: SystemTime::now(),
                expires_at,
            },
        );
        Ok(())
    }

    pub async fn clear_cache(&self) {
        let mut cache = self.cache.write().await;
        cache.clear();
        log::info!("Cache cleared");
    }
}


/*
      ∧,,∧
    (  > ̫ <  ）
     / つ　)
     しー-Ｊ
 */