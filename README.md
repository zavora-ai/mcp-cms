# Content Management MCP Server

[![Crates.io](https://img.shields.io/crates/v/mcp-cms.svg)](https://crates.io/crates/mcp-cms)
[![License](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](LICENSE)
[![ADK-Rust Enterprise](https://img.shields.io/badge/ADK--Rust-Enterprise-purple.svg)](https://enterprise.adk-rust.com)
[![Registry Ready](https://img.shields.io/badge/ADK_Registry-Ready-green.svg)](https://www.zavora.ai)

Unified content platform for AI agents — manage articles, social media, YouTube videos, media library, SEO, and publishing workflows from a single MCP server. 26 tools across content, social, and video.

## Architecture

<p align="center">
  <img src="https://raw.githubusercontent.com/zavora-ai/mcp-cms/main/docs/assets/architecture.svg" alt="MCP CMS Architecture" width="850"/>
</p>

## Example Backend (included)

A full reference implementation is included in `example-backend/` — **Rust (Axum) + SQLite + React** dashboard.

<p align="center">
  <img src="https://raw.githubusercontent.com/zavora-ai/mcp-cms/main/docs/assets/dashboard-screenshot.jpg" alt="CMS Dashboard" width="800" style="border-radius: 12px;"/>
</p>

### Quick Start

```bash
# Terminal 1: Start the backend
cd example-backend
cargo run
# → Dashboard: http://localhost:7799
# → API: http://localhost:7799/api/v1

# Terminal 2: Connect mcp-cms
CMS_API_URL="http://localhost:7799/api/v1" mcp-cms
```

### What's Included

- **Axum web server** — all 26 API endpoints implemented
- **SQLite database** — auto-created with schema + seeded demo data
- **React dashboard** — dark theme, content/social/video/media tabs, click-to-open detail views, video player, image previews
- **Zero config** — just `cargo run`, no env vars needed

### Building Your Own Backend

The example backend implements the [Custom API Spec](#custom-api-spec) below. Fork it as a starting point, or build your own in any language (Express, FastAPI, Go, Rails) — as long as it implements the same endpoints, mcp-cms will work with it.

## Tools (26)

### Content Management (6)

| Tool | Purpose | Risk |
|------|---------|------|
| `list_content` | List articles/pages by status, type, channel | read_only |
| `get_content` | Full content with metadata + SEO | read_only |
| `create_content` | Create article/page (draft) | internal_write |
| `update_content` | Update body, title, tags | internal_write |
| `publish_content` | Publish to website (goes live) | external_write |
| `unpublish_content` | Take offline | external_write |

### Social Media (5)

| Tool | Purpose | Risk |
|------|---------|------|
| `list_social_accounts` | Connected accounts (Twitter, LinkedIn, IG, FB) | read_only |
| `schedule_social_post` | Schedule post to platforms | external_write |
| `get_social_metrics` | Engagement: likes, shares, reach | read_only |
| `list_scheduled_posts` | Upcoming scheduled posts | read_only |
| `delete_scheduled_post` | Cancel a scheduled post | internal_write |

### Video / YouTube (5)

| Tool | Purpose | Risk |
|------|---------|------|
| `list_videos` | List videos | read_only |
| `get_video` | Video details, stats, transcript | read_only |
| `upload_video` | Upload with title, description, tags | external_write |
| `update_video` | Update metadata | internal_write |
| `get_video_analytics` | Views, watch time, retention, CTR | read_only |

### Media Library (3)

| Tool | Purpose | Risk |
|------|---------|------|
| `list_media` | List images, videos, files | read_only |
| `upload_media` | Upload asset | internal_write |
| `get_media` | Get asset URL + metadata | read_only |

### Taxonomy & SEO (4)

| Tool | Purpose | Risk |
|------|---------|------|
| `list_categories` | List categories/tags | read_only |
| `create_category` | Create category or tag | internal_write |
| `get_seo_metadata` | SEO title, description, OG tags | read_only |
| `update_seo` | Update SEO metadata | internal_write |

### Publishing Workflow (3)

| Tool | Purpose | Risk |
|------|---------|------|
| `get_content_calendar` | All scheduled content across channels | read_only |
| `request_review` | Submit for editorial review | internal_write |
| `get_publish_status` | Workflow state (draft → review → published) | read_only |

## Installation

```bash
cargo install mcp-cms
```

## Configuration

Multiple backends can be active simultaneously:

### Content Backend (required — pick one)

| Backend | Env Vars |
|---------|----------|
| **WordPress** | `WORDPRESS_URL` + `WORDPRESS_USERNAME` + `WORDPRESS_APP_PASSWORD` |
| **Contentful** | `CONTENTFUL_SPACE_ID` + `CONTENTFUL_MANAGEMENT_TOKEN` |
| **Custom API** | `CMS_API_URL` + `CMS_API_KEY` |

### Social Backend (optional)

| Backend | Env Vars |
|---------|----------|
| **Twitter/X** | `TWITTER_BEARER_TOKEN` |
| **LinkedIn** | `LINKEDIN_ACCESS_TOKEN` |
| **Meta (FB/IG)** | `META_ACCESS_TOKEN` + `META_PAGE_ID` |

### Video Backend (optional)

| Backend | Env Vars |
|---------|----------|
| **YouTube** | `YOUTUBE_OAUTH_TOKEN` |

If using Custom API (`CMS_API_URL`), it handles all endpoints (content + social + video).

## Client Configuration

```json
{
  "mcpServers": {
    "cms": {
      "command": "mcp-cms",
      "args": [],
      "env": {
        "CMS_API_URL": "http://localhost:8080/api/v1",
        "CMS_API_KEY": "your-key"
      }
    }
  }
}
```

## Custom API Spec

If building your own backend, implement these endpoints:

```
# Content
GET    /content?status=&type=&channel=
GET    /content/:id
POST   /content
PATCH  /content/:id
POST   /content/:id/publish
POST   /content/:id/unpublish
POST   /content/:id/review
GET    /content/:id/seo
PATCH  /content/:id/seo
GET    /content/:id/status

# Social
GET    /social/accounts
POST   /social/posts
GET    /social/posts/scheduled
DELETE /social/posts/:id
GET    /social/metrics?post_id=

# Video
GET    /videos
GET    /videos/:id
POST   /videos
PATCH  /videos/:id
GET    /videos/:id/analytics

# Media
GET    /media?type=
POST   /media
GET    /media/:id

# Taxonomy
GET    /categories
POST   /categories

# Calendar
GET    /calendar?start=&end=
```

## Usage Examples

### Publish a blog post
```
"Write and publish a blog post about our new feature"
→ create_content(title="New Feature Launch", body="...", content_type="article")
→ update_seo(content_id="...", title="New Feature | Company", description="...")
→ request_review(content_id="...", note="Ready for review")
→ publish_content(id="...")
```

### Cross-post to social
```
"Share the blog post on Twitter and LinkedIn"
→ schedule_social_post(platforms=["twitter","linkedin"], content="We just launched...", scheduled_at="now")
```

### Upload a YouTube video
```
"Upload the product demo video"
→ upload_video(title="Product Demo 2026", description="...", tags=["demo","product"])
→ get_video_analytics(id="...") — track performance
```

## License

Apache-2.0

---

Part of the [ADK-Rust Enterprise](https://enterprise.adk-rust.com) MCP server ecosystem.

Built with ❤️ by [Zavora AI](https://zavora.ai)
