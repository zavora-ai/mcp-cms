# Custom API Specification

Build your own CMS backend by implementing these REST endpoints. The mcp-cms server calls these when `CMS_API_URL` is configured.

## Authentication

If `CMS_API_KEY` is set, it's sent as:
```
Authorization: Bearer {CMS_API_KEY}
```

## Content Endpoints

### `GET /content`

List content items.

**Query Parameters:**
- `status` — filter by `draft`, `in_review`, `published`
- `type` — filter by `article`, `page`, `guide`
- `channel` — filter by distribution channel

**Response:** `200 OK`
```json
[
  {
    "id": "post-1",
    "title": "Getting Started",
    "body": "Full content body...",
    "status": "published",
    "content_type": "article",
    "tags": ["tutorial"],
    "author": "James",
    "created_at": "2026-05-20T10:00:00Z",
    "updated_at": "2026-05-20T10:00:00Z"
  }
]
```

### `GET /content/:id`

Get single content item with all fields.

### `POST /content`

Create content.

**Request:**
```json
{
  "title": "New Post",
  "body": "Content here...",
  "content_type": "article",
  "tags": ["news"],
  "author": "Alice"
}
```

**Response:** `201 Created`
```json
{"id": "post-abc123", "status": "draft"}
```

### `PATCH /content/:id`

Update content fields. All fields optional.

```json
{"title": "Updated Title", "body": "New body", "tags": ["updated"]}
```

### `POST /content/:id/publish`

Publish content. Response should include new status.

### `POST /content/:id/unpublish`

Unpublish content. Response should include new status.

### `POST /content/:id/review`

Submit for review.

```json
{"note": "Please review before Friday"}
```

### `GET /content/:id/seo`

Get SEO metadata.

**Response:**
```json
{"seo_title": "Page Title | Site", "seo_description": "Meta description", "keywords": ["key1", "key2"]}
```

### `PATCH /content/:id/seo`

Update SEO metadata.

```json
{"seo_title": "New Title", "seo_description": "New desc", "keywords": ["new"]}
```

### `GET /content/:id/status`

Get workflow status.

**Response:**
```json
{"id": "post-1", "status": "published"}
```

---

## Social Endpoints

### `GET /social/accounts`

List connected social accounts.

**Response:**
```json
[
  {"id": "tw-1", "platform": "twitter", "handle": "@company", "followers": 5000, "status": "connected"}
]
```

### `POST /social/posts`

Schedule a social post.

**Request:**
```json
{
  "platforms": ["twitter", "linkedin"],
  "content": "Exciting news! 🚀",
  "media_url": "https://cdn.example.com/image.png",
  "scheduled_at": "2026-05-27T10:00:00Z"
}
```

**Response:** `201 Created`
```json
{"id": "sp-abc", "status": "scheduled"}
```

### `GET /social/posts/scheduled`

List scheduled posts.

### `DELETE /social/posts/:id`

Delete/cancel a scheduled post.

### `GET /social/metrics`

Get engagement metrics.

**Query:** `?post_id=sp-1`

**Response:**
```json
{"likes": 142, "shares": 38, "comments": 12, "reach": 8400, "impressions": 12600, "engagement_rate": 3.2}
```

---

## Video Endpoints

### `GET /videos`

List all videos.

**Response:**
```json
[
  {"id": "vid-1", "title": "Demo", "description": "...", "tags": ["demo"], "privacy": "public", "status": "published", "views": 12400, "watch_time_hours": 890.5}
]
```

### `GET /videos/:id`

Get single video with full details.

### `POST /videos`

Upload/create a video.

**Request:**
```json
{"title": "New Video", "description": "About...", "tags": ["tutorial"], "privacy": "private"}
```

**Response:** `201 Created`
```json
{"id": "vid-abc", "status": "processing"}
```

### `PATCH /videos/:id`

Update video metadata.

### `GET /videos/:id/analytics`

Get video performance.

**Response:**
```json
{"video_id": "vid-1", "views": 12400, "watch_time_hours": 890.5, "avg_retention_pct": 62.3, "ctr_pct": 4.8, "likes": 340, "comments": 28}
```

---

## Media Endpoints

### `GET /media`

List media assets. Optional `?type=image` filter.

**Response:**
```json
[
  {"id": "img-1", "url": "https://cdn.example.com/hero.png", "alt_text": "Hero image", "filename": "hero.png", "media_type": "image", "created_at": "2026-05-20T10:00:00Z"}
]
```

### `POST /media`

Upload media.

**Request:**
```json
{"url": "https://cdn.example.com/new.png", "alt_text": "Description", "filename": "new.png"}
```

### `GET /media/:id`

Get single media item.

---

## Taxonomy Endpoints

### `GET /categories`

List categories/tags.

**Response:**
```json
[
  {"id": "cat-1", "name": "Engineering", "parent_id": null},
  {"id": "cat-2", "name": "Security", "parent_id": "cat-1"}
]
```

### `POST /categories`

Create category.

```json
{"name": "News", "parent_id": null}
```

---

## Calendar Endpoint

### `GET /calendar`

Get all scheduled content across channels.

**Query:** `?start=2026-05-01&end=2026-05-31`

**Response:**
```json
[
  {"id": "post-2", "title": "Dark Mode", "scheduled_at": "2026-05-26", "channel": "blog"},
  {"id": "sp-1", "title": "Tweet about launch", "scheduled_at": "2026-05-26T10:00:00Z", "channel": "social"}
]
```

---

## Error Responses

All errors return appropriate HTTP status codes:

```json
{"error": "Content not found", "status": 404}
```

| Status | Meaning |
|--------|---------|
| 400 | Bad request (missing required fields) |
| 401 | Unauthorized (invalid API key) |
| 404 | Resource not found |
| 409 | Conflict (e.g., publishing already-published content) |
| 500 | Internal server error |
