use axum::{
    Json, Router,
    extract::{Path, Query, State},
    http::StatusCode,
    routing::{delete, get, patch, post},
};
use rusqlite::Connection;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;
use tower_http::cors::CorsLayer;
use tower_http::services::ServeDir;

type Db = Arc<Mutex<Connection>>;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let db = Arc::new(Mutex::new(init_db()));
    seed_db(&db).await;

    let app = Router::new()
        // Content
        .route("/api/v1/content", get(list_content).post(create_content))
        .route("/api/v1/content/{id}", get(get_content).patch(update_content))
        .route("/api/v1/content/{id}/publish", post(publish_content))
        .route("/api/v1/content/{id}/unpublish", post(unpublish_content))
        .route("/api/v1/content/{id}/review", post(request_review))
        .route("/api/v1/content/{id}/seo", get(get_seo).patch(update_seo))
        .route("/api/v1/content/{id}/status", get(get_status))
        // Social
        .route("/api/v1/social/accounts", get(list_social_accounts))
        .route("/api/v1/social/posts", post(schedule_post))
        .route("/api/v1/social/posts/scheduled", get(list_scheduled_posts))
        .route("/api/v1/social/posts/{id}", delete(delete_post))
        .route("/api/v1/social/metrics", get(get_social_metrics))
        // Video
        .route("/api/v1/videos", get(list_videos).post(upload_video))
        .route("/api/v1/videos/{id}", get(get_video).patch(update_video))
        .route("/api/v1/videos/{id}/analytics", get(get_video_analytics))
        // Media
        .route("/api/v1/media", get(list_media).post(upload_media))
        .route("/api/v1/media/{id}", get(get_media_item))
        // Taxonomy
        .route("/api/v1/categories", get(list_categories).post(create_category))
        // Calendar
        .route("/api/v1/calendar", get(get_calendar))
        // Frontend
        .fallback_service(ServeDir::new("frontend/dist"))
        .layer(CorsLayer::permissive())
        .with_state(db);

    tracing::info!("CMS Backend running on http://localhost:7799");
    tracing::info!("Dashboard: http://localhost:7799");
    tracing::info!("API: http://localhost:7799/api/v1");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:7799").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

fn init_db() -> Connection {
    let conn = Connection::open("cms.db").unwrap();
    conn.execute_batch("
        CREATE TABLE IF NOT EXISTS content (
            id TEXT PRIMARY KEY, title TEXT, body TEXT, status TEXT DEFAULT 'draft',
            content_type TEXT DEFAULT 'article', tags TEXT DEFAULT '[]',
            author TEXT, seo_title TEXT, seo_description TEXT, seo_keywords TEXT DEFAULT '[]',
            created_at TEXT, updated_at TEXT
        );
        CREATE TABLE IF NOT EXISTS social_posts (
            id TEXT PRIMARY KEY, platforms TEXT, content TEXT, media_url TEXT,
            scheduled_at TEXT, status TEXT DEFAULT 'scheduled'
        );
        CREATE TABLE IF NOT EXISTS videos (
            id TEXT PRIMARY KEY, title TEXT, description TEXT, tags TEXT DEFAULT '[]',
            privacy TEXT DEFAULT 'private', status TEXT DEFAULT 'processing',
            views INTEGER DEFAULT 0, watch_time_hours REAL DEFAULT 0.0
        );
        CREATE TABLE IF NOT EXISTS media (
            id TEXT PRIMARY KEY, url TEXT, alt_text TEXT, filename TEXT,
            media_type TEXT DEFAULT 'image', created_at TEXT
        );
        CREATE TABLE IF NOT EXISTS categories (
            id TEXT PRIMARY KEY, name TEXT, parent_id TEXT
        );
    ").unwrap();
    conn
}

async fn seed_db(db: &Db) {
    let conn = db.lock().await;
    let count: i64 = conn.query_row("SELECT COUNT(*) FROM content", [], |r| r.get(0)).unwrap();
    if count > 0 { return; }

    conn.execute_batch("
        INSERT INTO content VALUES ('post-1','Getting Started with AI Agents','AI agents are transforming how we build software. In this guide, we cover the fundamentals of agent architecture, tool use, and orchestration patterns.','published','article','[\"ai\",\"tutorial\",\"engineering\"]','James Karanja','AI Agents Guide | Zavora','Learn how to build AI agents with MCP','[\"ai\",\"agents\",\"mcp\"]','2026-05-20T10:00:00Z','2026-05-20T10:00:00Z');
        INSERT INTO content VALUES ('post-2','Product Update: Dark Mode is Here','We listened to your feedback! Dark mode is now available across all platforms. Here is how to enable it.','draft','article','[\"product\",\"update\",\"dark-mode\"]','Lisa Chen',NULL,NULL,'[]','2026-05-24T14:00:00Z','2026-05-24T14:00:00Z');
        INSERT INTO content VALUES ('post-3','Enterprise Security Best Practices','A comprehensive guide to securing your enterprise deployment including SSO, RBAC, and audit logging.','published','guide','[\"security\",\"enterprise\",\"sso\"]','Alice Nguyen','Enterprise Security | Zavora','Security best practices for enterprise deployments','[\"security\",\"enterprise\"]','2026-05-18T09:00:00Z','2026-05-22T11:00:00Z');
        INSERT INTO content VALUES ('page-1','About Us','Zavora AI builds enterprise-grade AI infrastructure. Founded in 2024, we help teams ship AI-powered products faster.','published','page','[]','James Karanja','About Zavora AI','Enterprise AI infrastructure company','[\"about\",\"company\"]','2026-01-01T00:00:00Z','2026-05-01T00:00:00Z');

        INSERT INTO social_posts VALUES ('sp-1','[\"twitter\",\"linkedin\"]','🚀 Just published: Getting Started with AI Agents. A complete guide to building with MCP. Link in bio!',NULL,'2026-05-26T10:00:00Z','scheduled');
        INSERT INTO social_posts VALUES ('sp-2','[\"twitter\"]','Dark mode is coming! 🌙 Stay tuned for the announcement.',NULL,'2026-05-27T15:00:00Z','scheduled');

        INSERT INTO videos VALUES ('vid-1','MCP Server Architecture Deep Dive','A 20-minute walkthrough of how MCP servers work, including tool routing, transport layers, and multi-backend patterns.','[\"mcp\",\"architecture\",\"tutorial\"]','public','published',12400,890.5);
        INSERT INTO videos VALUES ('vid-2','Product Demo: AI Agent Workflows','See how AI agents orchestrate multi-step workflows with approvals, notifications, and error handling.','[\"demo\",\"workflows\",\"ai\"]','public','published',5200,320.0);

        INSERT INTO media VALUES ('img-1','https://cdn.zavora.ai/blog/ai-agents-hero.png','AI Agents architecture diagram','ai-agents-hero.png','image','2026-05-20T10:00:00Z');
        INSERT INTO media VALUES ('img-2','https://cdn.zavora.ai/blog/dark-mode-preview.png','Dark mode preview screenshot','dark-mode-preview.png','image','2026-05-24T14:00:00Z');
        INSERT INTO media VALUES ('vid-thumb-1','https://cdn.zavora.ai/video/mcp-thumb.jpg','MCP Deep Dive thumbnail','mcp-thumb.jpg','image','2026-05-15T08:00:00Z');

        INSERT INTO categories VALUES ('cat-1','Engineering',NULL);
        INSERT INTO categories VALUES ('cat-2','Product',NULL);
        INSERT INTO categories VALUES ('cat-3','Tutorial',NULL);
        INSERT INTO categories VALUES ('cat-4','Security','cat-1');
    ").unwrap();
    tracing::info!("Database seeded with sample content");
}

// --- Handlers ---

#[derive(Deserialize)]
struct ContentQuery { status: Option<String>, r#type: Option<String> }

#[derive(Serialize)]
struct ContentRow { id: String, title: String, body: String, status: String, content_type: String, tags: serde_json::Value, author: Option<String>, created_at: String, updated_at: String }

async fn list_content(State(db): State<Db>, Query(q): Query<ContentQuery>) -> Json<Vec<ContentRow>> {
    let conn = db.lock().await;
    let mut sql = "SELECT id,title,body,status,content_type,tags,author,created_at,updated_at FROM content WHERE 1=1".to_string();
    if let Some(ref s) = q.status { sql.push_str(&format!(" AND status='{}'", s)); }
    if let Some(ref t) = q.r#type { sql.push_str(&format!(" AND content_type='{}'", t)); }
    sql.push_str(" ORDER BY created_at DESC");
    let mut stmt = conn.prepare(&sql).unwrap();
    let rows = stmt.query_map([], |r| {
        Ok(ContentRow { id: r.get(0)?, title: r.get(1)?, body: r.get(2)?, status: r.get(3)?, content_type: r.get(4)?, tags: serde_json::from_str(&r.get::<_,String>(5)?).unwrap_or_default(), author: r.get(6)?, created_at: r.get(7)?, updated_at: r.get(8)? })
    }).unwrap().filter_map(|r| r.ok()).collect();
    Json(rows)
}

async fn get_content(State(db): State<Db>, Path(id): Path<String>) -> Json<serde_json::Value> {
    let conn = db.lock().await;
    let row = conn.query_row("SELECT id,title,body,status,content_type,tags,author,seo_title,seo_description,created_at,updated_at FROM content WHERE id=?", [&id], |r| {
        Ok(serde_json::json!({"id":r.get::<_,String>(0)?,"title":r.get::<_,String>(1)?,"body":r.get::<_,String>(2)?,"status":r.get::<_,String>(3)?,"content_type":r.get::<_,String>(4)?,"tags":serde_json::from_str::<serde_json::Value>(&r.get::<_,String>(5)?).unwrap_or_default(),"author":r.get::<_,Option<String>>(6)?,"seo_title":r.get::<_,Option<String>>(7)?,"seo_description":r.get::<_,Option<String>>(8)?,"created_at":r.get::<_,String>(9)?,"updated_at":r.get::<_,String>(10)?}))
    }).unwrap_or(serde_json::json!({"error":"not found"}));
    Json(row)
}

#[derive(Deserialize)]
struct CreateContent { title: String, body: String, content_type: Option<String>, tags: Option<Vec<String>>, author: Option<String> }

async fn create_content(State(db): State<Db>, Json(input): Json<CreateContent>) -> (StatusCode, Json<serde_json::Value>) {
    let id = format!("post-{}", &uuid::Uuid::new_v4().to_string()[..8]);
    let now = chrono::Utc::now().to_rfc3339();
    let tags = serde_json::to_string(&input.tags.unwrap_or_default()).unwrap();
    let conn = db.lock().await;
    conn.execute("INSERT INTO content (id,title,body,status,content_type,tags,author,created_at,updated_at) VALUES (?1,?2,?3,'draft',?4,?5,?6,?7,?7)",
        rusqlite::params![id, input.title, input.body, input.content_type.unwrap_or("article".into()), tags, input.author, now]).unwrap();
    (StatusCode::CREATED, Json(serde_json::json!({"id": id, "status": "draft"})))
}

#[derive(Deserialize)]
struct UpdateContent { title: Option<String>, body: Option<String>, tags: Option<Vec<String>> }

async fn update_content(State(db): State<Db>, Path(id): Path<String>, Json(input): Json<UpdateContent>) -> Json<serde_json::Value> {
    let conn = db.lock().await;
    if let Some(t) = input.title { conn.execute("UPDATE content SET title=?,updated_at=? WHERE id=?", rusqlite::params![t, chrono::Utc::now().to_rfc3339(), id]).unwrap(); }
    if let Some(b) = input.body { conn.execute("UPDATE content SET body=?,updated_at=? WHERE id=?", rusqlite::params![b, chrono::Utc::now().to_rfc3339(), id]).unwrap(); }
    if let Some(t) = input.tags { conn.execute("UPDATE content SET tags=?,updated_at=? WHERE id=?", rusqlite::params![serde_json::to_string(&t).unwrap(), chrono::Utc::now().to_rfc3339(), id]).unwrap(); }
    Json(serde_json::json!({"updated": true, "id": id}))
}

async fn publish_content(State(db): State<Db>, Path(id): Path<String>) -> Json<serde_json::Value> {
    let conn = db.lock().await;
    conn.execute("UPDATE content SET status='published',updated_at=? WHERE id=?", rusqlite::params![chrono::Utc::now().to_rfc3339(), id]).unwrap();
    Json(serde_json::json!({"status": "published", "id": id}))
}

async fn unpublish_content(State(db): State<Db>, Path(id): Path<String>) -> Json<serde_json::Value> {
    let conn = db.lock().await;
    conn.execute("UPDATE content SET status='draft',updated_at=? WHERE id=?", rusqlite::params![chrono::Utc::now().to_rfc3339(), id]).unwrap();
    Json(serde_json::json!({"status": "draft", "id": id}))
}

async fn request_review(State(db): State<Db>, Path(id): Path<String>) -> Json<serde_json::Value> {
    let conn = db.lock().await;
    conn.execute("UPDATE content SET status='in_review',updated_at=? WHERE id=?", rusqlite::params![chrono::Utc::now().to_rfc3339(), id]).unwrap();
    Json(serde_json::json!({"status": "in_review", "id": id}))
}

async fn get_seo(State(db): State<Db>, Path(id): Path<String>) -> Json<serde_json::Value> {
    let conn = db.lock().await;
    let row = conn.query_row("SELECT seo_title,seo_description,seo_keywords FROM content WHERE id=?", [&id], |r| {
        Ok(serde_json::json!({"seo_title":r.get::<_,Option<String>>(0)?,"seo_description":r.get::<_,Option<String>>(1)?,"keywords":serde_json::from_str::<serde_json::Value>(&r.get::<_,String>(2).unwrap_or("[]".into())).unwrap_or_default()}))
    }).unwrap_or(serde_json::json!({"error":"not found"}));
    Json(row)
}

#[derive(Deserialize)]
struct UpdateSeo { seo_title: Option<String>, seo_description: Option<String>, keywords: Option<Vec<String>> }

async fn update_seo(State(db): State<Db>, Path(id): Path<String>, Json(input): Json<UpdateSeo>) -> Json<serde_json::Value> {
    let conn = db.lock().await;
    if let Some(t) = input.seo_title { conn.execute("UPDATE content SET seo_title=? WHERE id=?", rusqlite::params![t, id]).unwrap(); }
    if let Some(d) = input.seo_description { conn.execute("UPDATE content SET seo_description=? WHERE id=?", rusqlite::params![d, id]).unwrap(); }
    if let Some(k) = input.keywords { conn.execute("UPDATE content SET seo_keywords=? WHERE id=?", rusqlite::params![serde_json::to_string(&k).unwrap(), id]).unwrap(); }
    Json(serde_json::json!({"updated": true}))
}

async fn get_status(State(db): State<Db>, Path(id): Path<String>) -> Json<serde_json::Value> {
    let conn = db.lock().await;
    let status: String = conn.query_row("SELECT status FROM content WHERE id=?", [&id], |r| r.get(0)).unwrap_or("unknown".into());
    Json(serde_json::json!({"id": id, "status": status}))
}

// Social
async fn list_social_accounts() -> Json<serde_json::Value> {
    Json(serde_json::json!([
        {"id":"tw-1","platform":"twitter","handle":"@zavora_ai","followers":2400,"status":"connected"},
        {"id":"li-1","platform":"linkedin","handle":"Zavora AI","followers":8900,"status":"connected"},
        {"id":"ig-1","platform":"instagram","handle":"@zavora.ai","followers":1200,"status":"connected"}
    ]))
}

#[derive(Deserialize)]
struct SchedulePost { platforms: Vec<String>, content: String, media_url: Option<String>, scheduled_at: String }

async fn schedule_post(State(db): State<Db>, Json(input): Json<SchedulePost>) -> (StatusCode, Json<serde_json::Value>) {
    let id = format!("sp-{}", &uuid::Uuid::new_v4().to_string()[..8]);
    let conn = db.lock().await;
    conn.execute("INSERT INTO social_posts (id,platforms,content,media_url,scheduled_at) VALUES (?1,?2,?3,?4,?5)",
        rusqlite::params![id, serde_json::to_string(&input.platforms).unwrap(), input.content, input.media_url, input.scheduled_at]).unwrap();
    (StatusCode::CREATED, Json(serde_json::json!({"id": id, "status": "scheduled"})))
}

async fn list_scheduled_posts(State(db): State<Db>) -> Json<Vec<serde_json::Value>> {
    let conn = db.lock().await;
    let mut stmt = conn.prepare("SELECT id,platforms,content,scheduled_at,status FROM social_posts WHERE status='scheduled' ORDER BY scheduled_at").unwrap();
    let rows = stmt.query_map([], |r| Ok(serde_json::json!({"id":r.get::<_,String>(0)?,"platforms":serde_json::from_str::<serde_json::Value>(&r.get::<_,String>(1)?).unwrap_or_default(),"content":r.get::<_,String>(2)?,"scheduled_at":r.get::<_,String>(3)?,"status":r.get::<_,String>(4)?}))).unwrap().filter_map(|r| r.ok()).collect();
    Json(rows)
}

async fn delete_post(State(db): State<Db>, Path(id): Path<String>) -> Json<serde_json::Value> {
    let conn = db.lock().await;
    conn.execute("DELETE FROM social_posts WHERE id=?", [&id]).unwrap();
    Json(serde_json::json!({"deleted": true, "id": id}))
}

#[derive(Deserialize)]
struct MetricsQuery { post_id: Option<String> }

async fn get_social_metrics(Query(_q): Query<MetricsQuery>) -> Json<serde_json::Value> {
    Json(serde_json::json!({"likes":142,"shares":38,"comments":12,"reach":8400,"impressions":12600,"engagement_rate":3.2}))
}

// Video
async fn list_videos(State(db): State<Db>) -> Json<Vec<serde_json::Value>> {
    let conn = db.lock().await;
    let mut stmt = conn.prepare("SELECT id,title,description,tags,privacy,status,views,watch_time_hours FROM videos").unwrap();
    let rows = stmt.query_map([], |r| Ok(serde_json::json!({"id":r.get::<_,String>(0)?,"title":r.get::<_,String>(1)?,"description":r.get::<_,String>(2)?,"tags":serde_json::from_str::<serde_json::Value>(&r.get::<_,String>(3)?).unwrap_or_default(),"privacy":r.get::<_,String>(4)?,"status":r.get::<_,String>(5)?,"views":r.get::<_,i64>(6)?,"watch_time_hours":r.get::<_,f64>(7)?}))).unwrap().filter_map(|r| r.ok()).collect();
    Json(rows)
}

async fn get_video(State(db): State<Db>, Path(id): Path<String>) -> Json<serde_json::Value> {
    let conn = db.lock().await;
    let row = conn.query_row("SELECT id,title,description,tags,privacy,status,views,watch_time_hours FROM videos WHERE id=?", [&id], |r| Ok(serde_json::json!({"id":r.get::<_,String>(0)?,"title":r.get::<_,String>(1)?,"description":r.get::<_,String>(2)?,"tags":serde_json::from_str::<serde_json::Value>(&r.get::<_,String>(3)?).unwrap_or_default(),"privacy":r.get::<_,String>(4)?,"status":r.get::<_,String>(5)?,"views":r.get::<_,i64>(6)?,"watch_time_hours":r.get::<_,f64>(7)?}))).unwrap_or(serde_json::json!({"error":"not found"}));
    Json(row)
}

#[derive(Deserialize)]
struct UploadVideo { title: String, description: String, tags: Option<Vec<String>>, privacy: Option<String> }

async fn upload_video(State(db): State<Db>, Json(input): Json<UploadVideo>) -> (StatusCode, Json<serde_json::Value>) {
    let id = format!("vid-{}", &uuid::Uuid::new_v4().to_string()[..8]);
    let conn = db.lock().await;
    conn.execute("INSERT INTO videos (id,title,description,tags,privacy,status) VALUES (?1,?2,?3,?4,?5,'processing')",
        rusqlite::params![id, input.title, input.description, serde_json::to_string(&input.tags.unwrap_or_default()).unwrap(), input.privacy.unwrap_or("private".into())]).unwrap();
    (StatusCode::CREATED, Json(serde_json::json!({"id": id, "status": "processing"})))
}

#[derive(Deserialize)]
struct UpdateVideo { title: Option<String>, description: Option<String>, tags: Option<Vec<String>> }

async fn update_video(State(db): State<Db>, Path(id): Path<String>, Json(input): Json<UpdateVideo>) -> Json<serde_json::Value> {
    let conn = db.lock().await;
    if let Some(t) = input.title { conn.execute("UPDATE videos SET title=? WHERE id=?", rusqlite::params![t, id]).unwrap(); }
    if let Some(d) = input.description { conn.execute("UPDATE videos SET description=? WHERE id=?", rusqlite::params![d, id]).unwrap(); }
    if let Some(t) = input.tags { conn.execute("UPDATE videos SET tags=? WHERE id=?", rusqlite::params![serde_json::to_string(&t).unwrap(), id]).unwrap(); }
    Json(serde_json::json!({"updated": true, "id": id}))
}

async fn get_video_analytics(Path(id): Path<String>) -> Json<serde_json::Value> {
    Json(serde_json::json!({"video_id": id, "views":12400,"watch_time_hours":890.5,"avg_retention_pct":62.3,"ctr_pct":4.8,"likes":340,"comments":28}))
}

// Media
async fn list_media(State(db): State<Db>) -> Json<Vec<serde_json::Value>> {
    let conn = db.lock().await;
    let mut stmt = conn.prepare("SELECT id,url,alt_text,filename,media_type,created_at FROM media").unwrap();
    let rows = stmt.query_map([], |r| Ok(serde_json::json!({"id":r.get::<_,String>(0)?,"url":r.get::<_,String>(1)?,"alt_text":r.get::<_,Option<String>>(2)?,"filename":r.get::<_,Option<String>>(3)?,"media_type":r.get::<_,String>(4)?,"created_at":r.get::<_,String>(5)?}))).unwrap().filter_map(|r| r.ok()).collect();
    Json(rows)
}

#[derive(Deserialize)]
struct UploadMedia { url: String, alt_text: Option<String>, filename: Option<String> }

async fn upload_media(State(db): State<Db>, Json(input): Json<UploadMedia>) -> (StatusCode, Json<serde_json::Value>) {
    let id = format!("img-{}", &uuid::Uuid::new_v4().to_string()[..8]);
    let conn = db.lock().await;
    conn.execute("INSERT INTO media (id,url,alt_text,filename,media_type,created_at) VALUES (?1,?2,?3,?4,'image',?5)",
        rusqlite::params![id, input.url, input.alt_text, input.filename, chrono::Utc::now().to_rfc3339()]).unwrap();
    (StatusCode::CREATED, Json(serde_json::json!({"id": id, "status": "uploaded"})))
}

async fn get_media_item(State(db): State<Db>, Path(id): Path<String>) -> Json<serde_json::Value> {
    let conn = db.lock().await;
    let row = conn.query_row("SELECT id,url,alt_text,filename,media_type,created_at FROM media WHERE id=?", [&id], |r| Ok(serde_json::json!({"id":r.get::<_,String>(0)?,"url":r.get::<_,String>(1)?,"alt_text":r.get::<_,Option<String>>(2)?,"filename":r.get::<_,Option<String>>(3)?,"media_type":r.get::<_,String>(4)?,"created_at":r.get::<_,String>(5)?}))).unwrap_or(serde_json::json!({"error":"not found"}));
    Json(row)
}

// Categories
async fn list_categories(State(db): State<Db>) -> Json<Vec<serde_json::Value>> {
    let conn = db.lock().await;
    let mut stmt = conn.prepare("SELECT id,name,parent_id FROM categories").unwrap();
    let rows = stmt.query_map([], |r| Ok(serde_json::json!({"id":r.get::<_,String>(0)?,"name":r.get::<_,String>(1)?,"parent_id":r.get::<_,Option<String>>(2)?}))).unwrap().filter_map(|r| r.ok()).collect();
    Json(rows)
}

#[derive(Deserialize)]
struct CreateCategory { name: String, parent_id: Option<String> }

async fn create_category(State(db): State<Db>, Json(input): Json<CreateCategory>) -> (StatusCode, Json<serde_json::Value>) {
    let id = format!("cat-{}", &uuid::Uuid::new_v4().to_string()[..8]);
    let conn = db.lock().await;
    conn.execute("INSERT INTO categories (id,name,parent_id) VALUES (?1,?2,?3)", rusqlite::params![id, input.name, input.parent_id]).unwrap();
    (StatusCode::CREATED, Json(serde_json::json!({"id": id, "name": input.name})))
}

// Calendar
async fn get_calendar(State(db): State<Db>) -> Json<Vec<serde_json::Value>> {
    let conn = db.lock().await;
    let mut items: Vec<serde_json::Value> = Vec::new();
    // Scheduled social posts
    let mut stmt = conn.prepare("SELECT id,content,scheduled_at FROM social_posts WHERE status='scheduled'").unwrap();
    let posts: Vec<_> = stmt.query_map([], |r| Ok(serde_json::json!({"id":r.get::<_,String>(0)?,"title":r.get::<_,String>(1)?,"scheduled_at":r.get::<_,String>(2)?,"channel":"social"}))).unwrap().filter_map(|r| r.ok()).collect();
    items.extend(posts);
    // Draft content (upcoming)
    let mut stmt2 = conn.prepare("SELECT id,title,updated_at FROM content WHERE status IN ('draft','in_review')").unwrap();
    let drafts: Vec<_> = stmt2.query_map([], |r| Ok(serde_json::json!({"id":r.get::<_,String>(0)?,"title":r.get::<_,String>(1)?,"scheduled_at":r.get::<_,String>(2)?,"channel":"blog"}))).unwrap().filter_map(|r| r.ok()).collect();
    items.extend(drafts);
    Json(items)
}
