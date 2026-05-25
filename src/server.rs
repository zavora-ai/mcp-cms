use crate::client::CmsBackend;
use rmcp::{handler::server::wrapper::Parameters, schemars, tool, tool_router};
use serde_json::json;

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct EmptyInput {}
#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct IdInput { pub id: String }
#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct ContentFilterInput { pub status: Option<String>, pub content_type: Option<String>, pub channel: Option<String> }
#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct CreateContentInput { pub title: String, pub body: String, pub content_type: Option<String>, pub tags: Option<Vec<String>>, pub author: Option<String> }
#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct UpdateContentInput { pub id: String, pub title: Option<String>, pub body: Option<String>, pub tags: Option<Vec<String>> }
#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct SchedulePostInput { pub platforms: Vec<String>, pub content: String, pub media_url: Option<String>, pub scheduled_at: String }
#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct VideoUploadInput { pub title: String, pub description: String, pub tags: Option<Vec<String>>, pub privacy: Option<String> }
#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct UpdateVideoInput { pub id: String, pub title: Option<String>, pub description: Option<String>, pub tags: Option<Vec<String>> }
#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct MediaFilterInput { pub media_type: Option<String> }
#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct UploadMediaInput { pub url: String, pub alt_text: Option<String>, pub filename: Option<String> }
#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct CreateCategoryInput { pub name: String, pub parent_id: Option<String> }
#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct AssignCategoryInput { pub content_id: String, pub category_ids: Vec<String> }
#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct UpdateSeoInput { pub content_id: String, pub title: Option<String>, pub description: Option<String>, pub keywords: Option<Vec<String>> }
#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct CalendarInput { pub start_date: Option<String>, pub end_date: Option<String> }
#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct ReviewInput { pub content_id: String, pub note: Option<String> }

#[derive(Clone)]
pub struct CmsServer { pub backend: CmsBackend }

fn r(result: Result<serde_json::Value, anyhow::Error>) -> String {
    match result { Ok(v) => serde_json::to_string_pretty(&v).unwrap(), Err(e) => format!("Error: {}", e) }
}

#[tool_router(server_handler)]
impl CmsServer {
    // === Content (6) ===
    #[tool(description = "List content (articles, pages, posts) filtered by status, type, or channel")]
    async fn list_content(&self, Parameters(input): Parameters<ContentFilterInput>) -> String {
        let mut path = "/content?".to_string();
        if let Some(s) = &input.status { path.push_str(&format!("status={}&", s)); }
        if let Some(t) = &input.content_type { path.push_str(&format!("type={}&", t)); }
        if let Some(c) = &input.channel { path.push_str(&format!("channel={}&", c)); }
        r(self.backend.content.get(&path).await)
    }

    #[tool(description = "Get full content with metadata, SEO, and media")]
    async fn get_content(&self, Parameters(input): Parameters<IdInput>) -> String {
        r(self.backend.content.get(&format!("/content/{}", input.id)).await)
    }

    #[tool(description = "Create content (article, page, post) in draft state")]
    async fn create_content(&self, Parameters(input): Parameters<CreateContentInput>) -> String {
        r(self.backend.content.post("/content", &json!({
            "title": input.title, "body": input.body, "status": "draft",
            "content_type": input.content_type.unwrap_or("article".into()),
            "tags": input.tags.unwrap_or_default(), "author": input.author
        })).await)
    }

    #[tool(description = "Update content body, title, or metadata")]
    async fn update_content(&self, Parameters(input): Parameters<UpdateContentInput>) -> String {
        let mut body = json!({});
        if let Some(t) = input.title { body["title"] = json!(t); }
        if let Some(b) = input.body { body["body"] = json!(b); }
        if let Some(t) = input.tags { body["tags"] = json!(t); }
        r(self.backend.content.patch(&format!("/content/{}", input.id), &body).await)
    }

    #[tool(description = "Publish content — makes it live on website/blog")]
    async fn publish_content(&self, Parameters(input): Parameters<IdInput>) -> String {
        r(self.backend.content.post(&format!("/content/{}/publish", input.id), &json!({})).await)
    }

    #[tool(description = "Unpublish content — takes it offline")]
    async fn unpublish_content(&self, Parameters(input): Parameters<IdInput>) -> String {
        r(self.backend.content.post(&format!("/content/{}/unpublish", input.id), &json!({})).await)
    }

    // === Social Media (5) ===
    #[tool(description = "List connected social media accounts (Twitter, LinkedIn, Instagram, Facebook)")]
    async fn list_social_accounts(&self, Parameters(_): Parameters<EmptyInput>) -> String {
        match &self.backend.social {
            Some(api) => r(api.get("/social/accounts").await),
            None => "Error: No social backend configured".into(),
        }
    }

    #[tool(description = "Schedule a post to one or more social platforms")]
    async fn schedule_social_post(&self, Parameters(input): Parameters<SchedulePostInput>) -> String {
        match &self.backend.social {
            Some(api) => r(api.post("/social/posts", &json!({
                "platforms": input.platforms, "content": input.content,
                "media_url": input.media_url, "scheduled_at": input.scheduled_at
            })).await),
            None => "Error: No social backend configured".into(),
        }
    }

    #[tool(description = "Get social media engagement metrics (likes, shares, comments, reach)")]
    async fn get_social_metrics(&self, Parameters(input): Parameters<IdInput>) -> String {
        match &self.backend.social {
            Some(api) => r(api.get(&format!("/social/metrics?post_id={}", input.id)).await),
            None => "Error: No social backend configured".into(),
        }
    }

    #[tool(description = "List upcoming scheduled social posts")]
    async fn list_scheduled_posts(&self, Parameters(_): Parameters<EmptyInput>) -> String {
        match &self.backend.social {
            Some(api) => r(api.get("/social/posts/scheduled").await),
            None => "Error: No social backend configured".into(),
        }
    }

    #[tool(description = "Cancel a scheduled social post")]
    async fn delete_scheduled_post(&self, Parameters(input): Parameters<IdInput>) -> String {
        match &self.backend.social {
            Some(api) => r(api.delete(&format!("/social/posts/{}", input.id)).await),
            None => "Error: No social backend configured".into(),
        }
    }

    // === Video / YouTube (5) ===
    #[tool(description = "List videos (YouTube, Vimeo, or custom platform)")]
    async fn list_videos(&self, Parameters(_): Parameters<EmptyInput>) -> String {
        match &self.backend.video {
            Some(api) => r(api.get("/videos").await),
            None => "Error: No video backend configured".into(),
        }
    }

    #[tool(description = "Get video details, stats, and transcript")]
    async fn get_video(&self, Parameters(input): Parameters<IdInput>) -> String {
        match &self.backend.video {
            Some(api) => r(api.get(&format!("/videos/{}", input.id)).await),
            None => "Error: No video backend configured".into(),
        }
    }

    #[tool(description = "Upload video with title, description, and tags")]
    async fn upload_video(&self, Parameters(input): Parameters<VideoUploadInput>) -> String {
        match &self.backend.video {
            Some(api) => r(api.post("/videos", &json!({
                "title": input.title, "description": input.description,
                "tags": input.tags.unwrap_or_default(), "privacy": input.privacy.unwrap_or("private".into())
            })).await),
            None => "Error: No video backend configured".into(),
        }
    }

    #[tool(description = "Update video title, description, tags, or thumbnail")]
    async fn update_video(&self, Parameters(input): Parameters<UpdateVideoInput>) -> String {
        match &self.backend.video {
            Some(api) => {
                let mut body = json!({});
                if let Some(t) = input.title { body["title"] = json!(t); }
                if let Some(d) = input.description { body["description"] = json!(d); }
                if let Some(t) = input.tags { body["tags"] = json!(t); }
                r(api.patch(&format!("/videos/{}", input.id), &body).await)
            }
            None => "Error: No video backend configured".into(),
        }
    }

    #[tool(description = "Get video analytics: views, watch time, retention, CTR")]
    async fn get_video_analytics(&self, Parameters(input): Parameters<IdInput>) -> String {
        match &self.backend.video {
            Some(api) => r(api.get(&format!("/videos/{}/analytics", input.id)).await),
            None => "Error: No video backend configured".into(),
        }
    }

    // === Media Library (3) ===
    #[tool(description = "List media assets (images, videos, files)")]
    async fn list_media(&self, Parameters(input): Parameters<MediaFilterInput>) -> String {
        let mut path = "/media?".to_string();
        if let Some(t) = &input.media_type { path.push_str(&format!("type={}&", t)); }
        r(self.backend.content.get(&path).await)
    }

    #[tool(description = "Upload a media asset to the library")]
    async fn upload_media(&self, Parameters(input): Parameters<UploadMediaInput>) -> String {
        r(self.backend.content.post("/media", &json!({"url": input.url, "alt_text": input.alt_text, "filename": input.filename})).await)
    }

    #[tool(description = "Get media asset details and URL")]
    async fn get_media(&self, Parameters(input): Parameters<IdInput>) -> String {
        r(self.backend.content.get(&format!("/media/{}", input.id)).await)
    }

    // === Taxonomy & SEO (4) ===
    #[tool(description = "List categories and tags")]
    async fn list_categories(&self, Parameters(_): Parameters<EmptyInput>) -> String {
        r(self.backend.content.get("/categories").await)
    }

    #[tool(description = "Create a category or tag")]
    async fn create_category(&self, Parameters(input): Parameters<CreateCategoryInput>) -> String {
        r(self.backend.content.post("/categories", &json!({"name": input.name, "parent_id": input.parent_id})).await)
    }

    #[tool(description = "Get SEO metadata for content (title, description, OG tags)")]
    async fn get_seo_metadata(&self, Parameters(input): Parameters<IdInput>) -> String {
        r(self.backend.content.get(&format!("/content/{}/seo", input.id)).await)
    }

    #[tool(description = "Update SEO metadata (title, description, keywords)")]
    async fn update_seo(&self, Parameters(input): Parameters<UpdateSeoInput>) -> String {
        let mut body = json!({});
        if let Some(t) = input.title { body["seo_title"] = json!(t); }
        if let Some(d) = input.description { body["seo_description"] = json!(d); }
        if let Some(k) = input.keywords { body["keywords"] = json!(k); }
        r(self.backend.content.patch(&format!("/content/{}/seo", input.content_id), &body).await)
    }

    // === Publishing Workflow (3) ===
    #[tool(description = "View content calendar — all scheduled content across channels")]
    async fn get_content_calendar(&self, Parameters(input): Parameters<CalendarInput>) -> String {
        let mut path = "/calendar?".to_string();
        if let Some(s) = &input.start_date { path.push_str(&format!("start={}&", s)); }
        if let Some(e) = &input.end_date { path.push_str(&format!("end={}&", e)); }
        r(self.backend.content.get(&path).await)
    }

    #[tool(description = "Submit content for editorial review")]
    async fn request_review(&self, Parameters(input): Parameters<ReviewInput>) -> String {
        r(self.backend.content.post(&format!("/content/{}/review", input.content_id), &json!({"note": input.note})).await)
    }

    #[tool(description = "Get content workflow state (draft, review, scheduled, published)")]
    async fn get_publish_status(&self, Parameters(input): Parameters<IdInput>) -> String {
        r(self.backend.content.get(&format!("/content/{}/status", input.id)).await)
    }
}
