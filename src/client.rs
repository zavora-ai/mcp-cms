use anyhow::{Result, bail};
use reqwest::Client;
use serde_json::Value;

#[derive(Clone)]
pub struct ApiClient {
    pub http: Client,
    pub base_url: String,
    pub auth_header: Option<String>,
}

impl ApiClient {
    pub fn new(base_url: &str, auth: Option<String>) -> Self {
        Self { http: Client::new(), base_url: base_url.trim_end_matches('/').to_string(), auth_header: auth }
    }

    pub async fn get(&self, path: &str) -> Result<Value> {
        let mut req = self.http.get(format!("{}{}", self.base_url, path));
        if let Some(ref a) = self.auth_header { req = req.header("Authorization", a); }
        let resp = req.send().await?;
        if !resp.status().is_success() { bail!("API {}: {}", resp.status(), resp.text().await?); }
        Ok(resp.json().await?)
    }

    pub async fn post(&self, path: &str, body: &Value) -> Result<Value> {
        let mut req = self.http.post(format!("{}{}", self.base_url, path)).json(body);
        if let Some(ref a) = self.auth_header { req = req.header("Authorization", a); }
        let resp = req.send().await?;
        if !resp.status().is_success() { bail!("API {}: {}", resp.status(), resp.text().await?); }
        Ok(resp.json().await?)
    }

    pub async fn patch(&self, path: &str, body: &Value) -> Result<Value> {
        let mut req = self.http.patch(format!("{}{}", self.base_url, path)).json(body);
        if let Some(ref a) = self.auth_header { req = req.header("Authorization", a); }
        let resp = req.send().await?;
        if !resp.status().is_success() { bail!("API {}: {}", resp.status(), resp.text().await?); }
        Ok(resp.json().await?)
    }

    pub async fn delete(&self, path: &str) -> Result<Value> {
        let mut req = self.http.delete(format!("{}{}", self.base_url, path));
        if let Some(ref a) = self.auth_header { req = req.header("Authorization", a); }
        let resp = req.send().await?;
        if !resp.status().is_success() { bail!("API {}: {}", resp.status(), resp.text().await?); }
        Ok(resp.json().await?)
    }
}

/// Multi-backend: content (WordPress/Contentful/Custom), social (Twitter/LinkedIn/Meta), video (YouTube)
#[derive(Clone)]
pub struct CmsBackend {
    pub content: ApiClient,
    pub social: Option<ApiClient>,
    pub video: Option<ApiClient>,
}

impl CmsBackend {
    pub fn from_env() -> Result<Self> {
        // Content backend
        let content = if let Ok(url) = std::env::var("WORDPRESS_URL") {
            let user = std::env::var("WORDPRESS_USERNAME").unwrap_or_default();
            let pass = std::env::var("WORDPRESS_APP_PASSWORD").unwrap_or_default();
            let auth = base64::engine::general_purpose::STANDARD.encode(format!("{}:{}", user, pass));
            tracing::info!("Content backend: WordPress");
            ApiClient::new(&format!("{}/wp-json/wp/v2", url), Some(format!("Basic {}", auth)))
        } else if let (Ok(space), Ok(token)) = (std::env::var("CONTENTFUL_SPACE_ID"), std::env::var("CONTENTFUL_MANAGEMENT_TOKEN")) {
            tracing::info!("Content backend: Contentful");
            ApiClient::new(&format!("https://api.contentful.com/spaces/{}", space), Some(format!("Bearer {}", token)))
        } else if let Ok(url) = std::env::var("CMS_API_URL") {
            let auth = std::env::var("CMS_API_KEY").ok().map(|k| format!("Bearer {}", k));
            tracing::info!("Content backend: Custom API");
            ApiClient::new(&url, auth)
        } else {
            bail!("No CMS backend. Set WORDPRESS_URL, CONTENTFUL_SPACE_ID, or CMS_API_URL")
        };

        // Social backend (optional)
        let social = if let Ok(token) = std::env::var("TWITTER_BEARER_TOKEN") {
            tracing::info!("Social backend: Twitter");
            Some(ApiClient::new("https://api.twitter.com/2", Some(format!("Bearer {}", token))))
        } else if let Ok(token) = std::env::var("LINKEDIN_ACCESS_TOKEN") {
            tracing::info!("Social backend: LinkedIn");
            Some(ApiClient::new("https://api.linkedin.com/v2", Some(format!("Bearer {}", token))))
        } else if let Ok(token) = std::env::var("META_ACCESS_TOKEN") {
            tracing::info!("Social backend: Meta");
            Some(ApiClient::new("https://graph.facebook.com/v18.0", Some(format!("Bearer {}", token))))
        } else {
            // Fall back to CMS API for social endpoints
            std::env::var("CMS_API_URL").ok().map(|url| {
                let auth = std::env::var("CMS_API_KEY").ok().map(|k| format!("Bearer {}", k));
                ApiClient::new(&url, auth)
            })
        };

        // Video backend (optional)
        let video = if let Ok(token) = std::env::var("YOUTUBE_OAUTH_TOKEN") {
            tracing::info!("Video backend: YouTube");
            Some(ApiClient::new("https://www.googleapis.com/youtube/v3", Some(format!("Bearer {}", token))))
        } else {
            std::env::var("CMS_API_URL").ok().map(|url| {
                let auth = std::env::var("CMS_API_KEY").ok().map(|k| format!("Bearer {}", k));
                ApiClient::new(&url, auth)
            })
        };

        Ok(Self { content, social, video })
    }
}

use base64::Engine as _;
