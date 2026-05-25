# Backends Guide

mcp-cms supports multiple backends simultaneously. You can run WordPress for blog content, YouTube for video, and Twitter for social — all at the same time.

## Backend Detection

On startup, the server detects backends from environment variables in this order:

### Content Backend (required — pick one)

| Backend | Detection | API Base |
|---------|-----------|----------|
| WordPress | `WORDPRESS_URL` is set | `{WORDPRESS_URL}/wp-json/wp/v2` |
| Contentful | `CONTENTFUL_SPACE_ID` is set | `https://api.contentful.com/spaces/{id}` |
| Custom API | `CMS_API_URL` is set | `{CMS_API_URL}` |

### Social Backend (optional)

| Backend | Detection | API Base |
|---------|-----------|----------|
| Twitter/X | `TWITTER_BEARER_TOKEN` is set | `https://api.twitter.com/2` |
| LinkedIn | `LINKEDIN_ACCESS_TOKEN` is set | `https://api.linkedin.com/v2` |
| Meta (FB/IG) | `META_ACCESS_TOKEN` is set | `https://graph.facebook.com/v18.0` |
| Custom API | Falls back to `CMS_API_URL` | `{CMS_API_URL}/social/*` |

### Video Backend (optional)

| Backend | Detection | API Base |
|---------|-----------|----------|
| YouTube | `YOUTUBE_OAUTH_TOKEN` is set | `https://www.googleapis.com/youtube/v3` |
| Custom API | Falls back to `CMS_API_URL` | `{CMS_API_URL}/videos/*` |

## WordPress Setup

### Prerequisites
- WordPress 5.6+ with REST API enabled (default)
- Application Password (Settings → Users → Application Passwords)

### Environment Variables

```bash
export WORDPRESS_URL="https://yourblog.com"
export WORDPRESS_USERNAME="admin"
export WORDPRESS_APP_PASSWORD="xxxx xxxx xxxx xxxx xxxx xxxx"
```

### How it maps

| MCP Tool | WordPress API |
|----------|---------------|
| `list_content` | `GET /wp-json/wp/v2/posts` + `/pages` |
| `get_content` | `GET /wp-json/wp/v2/posts/{id}` |
| `create_content` | `POST /wp-json/wp/v2/posts` |
| `update_content` | `PUT /wp-json/wp/v2/posts/{id}` |
| `publish_content` | `PUT /wp-json/wp/v2/posts/{id}` with `status: publish` |
| `list_media` | `GET /wp-json/wp/v2/media` |
| `upload_media` | `POST /wp-json/wp/v2/media` |
| `list_categories` | `GET /wp-json/wp/v2/categories` |

### Limitations
- Social and video tools require a separate backend (WordPress doesn't handle these)
- SEO requires Yoast SEO or RankMath plugin with REST API support

---

## Contentful Setup

### Prerequisites
- Contentful space with content models defined
- Management API token (Settings → API Keys → Content Management tokens)

### Environment Variables

```bash
export CONTENTFUL_SPACE_ID="your-space-id"
export CONTENTFUL_MANAGEMENT_TOKEN="CFPAT-xxxxxxxx"
```

### How it maps

| MCP Tool | Contentful API |
|----------|----------------|
| `list_content` | `GET /entries?content_type={type}` |
| `get_content` | `GET /entries/{id}` |
| `create_content` | `PUT /entries/{id}` |
| `publish_content` | `PUT /entries/{id}/published` |
| `unpublish_content` | `DELETE /entries/{id}/published` |
| `list_media` | `GET /assets` |
| `upload_media` | `POST /uploads` → `POST /assets` |

### Notes
- Contentful uses "entries" not "posts" — the MCP normalizes this
- Content types must be pre-defined in Contentful (the MCP doesn't create models)
- Rich text fields are returned as Contentful's structured JSON

---

## YouTube Setup

### Prerequisites
- Google Cloud project with YouTube Data API v3 enabled
- OAuth 2.0 token with `youtube.upload` and `youtube.readonly` scopes

### Environment Variables

```bash
export YOUTUBE_OAUTH_TOKEN="ya29.xxxxxxxx"
# Optional: for API key-only access (read-only)
export YOUTUBE_API_KEY="AIzaxxxxxxxx"
```

### How it maps

| MCP Tool | YouTube API |
|----------|-------------|
| `list_videos` | `GET /videos?mine=true&part=snippet,statistics` |
| `get_video` | `GET /videos?id={id}&part=snippet,statistics,contentDetails` |
| `upload_video` | `POST /videos?part=snippet,status` (resumable upload) |
| `update_video` | `PUT /videos?part=snippet` |
| `get_video_analytics` | `GET /reports` (YouTube Analytics API) |

### Getting an OAuth Token

1. Create OAuth 2.0 credentials in Google Cloud Console
2. Use the OAuth playground or your app to get a refresh token
3. Exchange for access token:
```bash
curl -X POST https://oauth2.googleapis.com/token \
  -d "client_id=YOUR_CLIENT_ID" \
  -d "client_secret=YOUR_SECRET" \
  -d "refresh_token=YOUR_REFRESH_TOKEN" \
  -d "grant_type=refresh_token"
```

---

## Twitter/X Setup

### Prerequisites
- Twitter Developer account with API v2 access
- Bearer token or OAuth 2.0 user token (for posting)

### Environment Variables

```bash
export TWITTER_BEARER_TOKEN="AAAAAAAAAAAAAAAAAAAAAxxxxxxxx"
```

### How it maps

| MCP Tool | Twitter API |
|----------|-------------|
| `list_social_accounts` | Returns configured Twitter account info |
| `schedule_social_post` | `POST /tweets` (immediate) or queue for scheduling |
| `get_social_metrics` | `GET /tweets/{id}?tweet.fields=public_metrics` |
| `list_scheduled_posts` | Local queue (Twitter doesn't have native scheduling API) |

### Notes
- Twitter API v2 free tier allows 1,500 tweets/month
- For scheduling, posts are queued locally and sent at the scheduled time
- Read access (metrics) works with Bearer token; posting requires OAuth 2.0 User Context

---

## LinkedIn Setup

### Prerequisites
- LinkedIn app with `w_member_social` and `r_liteprofile` permissions
- OAuth 2.0 access token

### Environment Variables

```bash
export LINKEDIN_ACCESS_TOKEN="AQVxxxxxxxx"
```

### How it maps

| MCP Tool | LinkedIn API |
|----------|--------------|
| `schedule_social_post` | `POST /ugcPosts` |
| `get_social_metrics` | `GET /organizationalEntityShareStatistics` |

---

## Meta (Facebook/Instagram) Setup

### Prerequisites
- Facebook Page with connected Instagram account
- Page Access Token with `pages_manage_posts` permission

### Environment Variables

```bash
export META_ACCESS_TOKEN="EAAxxxxxxxx"
export META_PAGE_ID="123456789"
```

### How it maps

| MCP Tool | Meta API |
|----------|----------|
| `schedule_social_post` | `POST /{page_id}/feed` (FB) or `POST /{ig_user_id}/media` (IG) |
| `get_social_metrics` | `GET /{post_id}?fields=insights` |

---

## Custom API Backend

The most flexible option. Build your own backend in any language that implements the API spec.

### Environment Variables

```bash
export CMS_API_URL="http://localhost:7799/api/v1"
export CMS_API_KEY="your-secret-key"  # optional, sent as Bearer token
```

### When to use Custom API

- You want full control over all features (content + social + video)
- You're integrating with an internal CMS
- You want to aggregate multiple sources behind one API
- You're building a SaaS product

### Reference Implementation

See `example-backend/` in this repo — a complete Rust + SQLite + React implementation.

---

## Multi-Backend Configuration

You can combine backends. Example:

```bash
# WordPress for blog content
export WORDPRESS_URL="https://blog.company.com"
export WORDPRESS_USERNAME="editor"
export WORDPRESS_APP_PASSWORD="xxxx xxxx xxxx"

# YouTube for video
export YOUTUBE_OAUTH_TOKEN="ya29.xxxxx"

# Twitter + LinkedIn for social
export TWITTER_BEARER_TOKEN="AAAAAxxxxx"
export LINKEDIN_ACCESS_TOKEN="AQVxxxxx"
```

With this config:
- Content tools → WordPress
- Video tools → YouTube
- Social tools → Twitter (first detected wins for the social client)

### Priority Order

If multiple social backends are configured, the first one detected is used:
1. Twitter (if `TWITTER_BEARER_TOKEN` set)
2. LinkedIn (if `LINKEDIN_ACCESS_TOKEN` set)
3. Meta (if `META_ACCESS_TOKEN` set)
4. Custom API (fallback to `CMS_API_URL`)

---

## Authentication

All backends use the standard auth mechanism for their platform:

| Backend | Auth Method |
|---------|-------------|
| WordPress | Basic Auth (username:app_password, base64 encoded) |
| Contentful | Bearer token (Management API token) |
| YouTube | Bearer token (OAuth 2.0 access token) |
| Twitter | Bearer token |
| LinkedIn | Bearer token (OAuth 2.0) |
| Meta | Bearer token (Page Access Token) |
| Custom API | Bearer token (optional `CMS_API_KEY`) |

The MCP server never stores tokens — they're read from environment variables at startup.
