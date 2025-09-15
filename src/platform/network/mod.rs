use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, SystemTime};

use anyhow::{Context, Result};
use reqwest::{Client, ClientBuilder, Method, StatusCode, Url};
use tokio::sync::RwLock;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ResourceType {
    Html,
    Css,
    JavaScript,
    Image,
    Font,
    Other,
}

// resource type detection based on content-type
impl ResourceType {
    pub fn from_content_type(content_type: &str) -> Self {
        if content_type.contains("text/html") {
            ResourceType::Html
        } else if content_type.contains("text/css") {
            ResourceType::Css
        } else if content_type.contains("javascript") || content_type.contains("ecmascript") {
            ResourceType::JavaScript
        } else if content_type.contains("image/") {
            ResourceType::Image
        } else if content_type.contains("font/") || content_type.contains("application/font") {
            ResourceType::Font
        } else {
            ResourceType::Other
        }
    }

    pub fn from_extension(extension: &str) -> Self {
        match extension.to_lowercase().as_str() {
            "html" | "htm" => ResourceType::Html,
            "css" => ResourceType::Css,
            "js" => ResourceType::JavaScript,
            "jpg" | "jpeg" | "png" | "gif" | "webp" | "svg" | "ico" => ResourceType::Image,
            "ttf" | "otf" | "woff" | "woff2" => ResourceType::Font,
            _ => ResourceType::Other,
        }
    }
}

#[derive(Debug, Clone)]
pub struct RequestOptions {
    pub timeout_ms: u64,
    pub follow_redirects: bool,
    pub max_redirects: u32,
    pub headers: HashMap<String, String>,
}

// Default options
impl Default for RequestOptions {
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
pub struct Response {
    pub status: StatusCode,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>,
    pub resource_type: ResourceType,
    pub url: Url,
}

#[derive(Debug)]
struct CacheEntry {
    response: Response,
    cached_at: SystemTime,
    expires_at: Option<SystemTime>,
}

#[derive(Debug)]
pub struct NetworkManager {
    client: Client,
    cache: Arc<RwLock<HashMap<String, CacheEntry>>>,
    default_options: RequestOptions,
}

// NetworkManager implementation
impl NetworkManager {
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
            default_options: RequestOptions::default(),
        })
    }

    pub fn set_default_options(&mut self, options: RequestOptions) {
        self.default_options = options;
    }

    pub async fn fetch(&self, url: &str) -> Result<Response> {
        self.fetch_with_options(url, &self.default_options).await
    }

    pub async fn fetch_with_options(&self, url: &str, options: &RequestOptions) -> Result<Response> {
        let url_obj = Url::parse(url).context("Invalid URL")?;
        
        if let Some(cached) = self.get_from_cache(&url_obj).await {
            log::info!("Resource retrieved from cache: {}", url);
            return Ok(cached);
        }

        let mut request_builder = self.client.request(Method::GET, url_obj.clone());
        request_builder = request_builder.timeout(Duration::from_millis(options.timeout_ms));
        for (name, value) in &options.headers {
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
        let resource_type = if let Some(content_type) = headers.get("content-type") {
            ResourceType::from_content_type(content_type)
        } else {
            if let Some(path) = url_obj.path_segments() {
                if let Some(last) = path.last() {
                    if let Some(dot_pos) = last.rfind('.') {
                        let extension = &last[dot_pos + 1..];
                        ResourceType::from_extension(extension)
                    } else {
                        ResourceType::Other
                    }
                } else {
                    ResourceType::Other
                }
            } else {
                ResourceType::Other
            }
        };
        
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
            resource_type,
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
            .timeout(Duration::from_millis(self.default_options.timeout_ms))
            .body(body);
        request_builder = request_builder.header("Content-Type", content_type);
        for (name, value) in &self.default_options.headers {
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
        let resource_type = if let Some(content_type) = headers.get("content-type") {
            ResourceType::from_content_type(content_type)
        } else {
            ResourceType::Other
        };

        let body = response
            .bytes()
            .await
            .context("Failed to get response body")?
            .to_vec();
        
        Ok(Response {
            status,
            headers,
            body,
            resource_type,
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

// load
pub async fn load_html_page(url: &str) -> Result<String> {
    let network = NetworkManager::new()?;
    let response = network.fetch(url).await?;
    if !response.status.is_success() {
        return Err(anyhow::anyhow!(
            "Failed to retrieve page: HTTP {} {}",
            response.status.as_u16(),
            response.status.canonical_reason().unwrap_or("")
        ));
    }
    if response.resource_type != ResourceType::Html {
        log::warn!("Resource specified as HTML is of a different type: {:?}", response.resource_type);
    }
    let html = String::from_utf8(response.body)
        .context("Failed to decode HTML content as UTF-8")?;
    Ok(html)
}

pub async fn load_local_file(path: &str) -> Result<Vec<u8>> {
    use tokio::fs::File;
    use tokio::io::AsyncReadExt;
    let mut file = File::open(path).await.context("Failed to open file")?;
    let mut contents = Vec::new();
    file.read_to_end(&mut contents).await.context("Failed to read file")?;
    Ok(contents)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_resource_type_detection() {
        assert_eq!(ResourceType::from_content_type("text/html; charset=utf-8"), ResourceType::Html);
        assert_eq!(ResourceType::from_content_type("text/css"), ResourceType::Css);
        assert_eq!(ResourceType::from_content_type("application/javascript"), ResourceType::JavaScript);
        assert_eq!(ResourceType::from_content_type("image/png"), ResourceType::Image);
        assert_eq!(ResourceType::from_content_type("font/woff2"), ResourceType::Font);
        assert_eq!(ResourceType::from_content_type("text/plain"), ResourceType::Other);
        
        assert_eq!(ResourceType::from_extension("html"), ResourceType::Html);
        assert_eq!(ResourceType::from_extension("css"), ResourceType::Css);
        assert_eq!(ResourceType::from_extension("js"), ResourceType::JavaScript);
        assert_eq!(ResourceType::from_extension("png"), ResourceType::Image);
        assert_eq!(ResourceType::from_extension("woff2"), ResourceType::Font);
        assert_eq!(ResourceType::from_extension("txt"), ResourceType::Other);
    }
}
/*
      ∧,,∧
    (  > ̫ <  ）
     / つ　)
     しー-Ｊ
 */