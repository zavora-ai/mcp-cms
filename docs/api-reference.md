# API Reference

Complete reference for all 26 tools in mcp-cms.

## Content Management

### list_content

List articles, pages, and posts with optional filters.

**Parameters:**
| Name | Type | Required | Description |
|------|------|:---:|-------------|
| `status` | string | ❌ | Filter: `draft`, `in_review`, `published` |
| `content_type` | string | ❌ | Filter: `article`, `page`, `guide`, `post` |
| `channel` | string | ❌ | Filter by distribution channel |

**Example:**
```json
{"name": "list_content", "arguments": {"status": "published"}}
```

---

### get_content

Get full content with body, metadata, SEO, and media.

**Parameters:**
| Name | Type | Required | Description |
|------|------|:---:|-------------|
| `id` | string | ✅ | Content ID |

---

### create_content

Create new content in draft state.

**Parameters:**
| Name | Type | Required | Description |
|------|------|:---:|-------------|
| `title` | string | ✅ | Content title |
| `body` | string | ✅ | Content body (plain text or markdown) |
| `content_type` | string | ❌ | Type: `article`, `page`, `guide` (default: `article`) |
| `tags` | string[] | ❌ | Tags for categorization |
| `author` | string | ❌ | Author name |

**Example:**
```json
{"name": "create_content", "arguments": {"title": "Getting Started", "body": "Welcome to...", "tags": ["tutorial", "beginner"]}}
```

---

### update_content

Update existing content fields.

**Parameters:**
| Name | Type | Required | Description |
|------|------|:---:|-------------|
| `id` | string | ✅ | Content ID |
| `title` | string | ❌ | New title |
| `body` | string | ❌ | New body |
| `tags` | string[] | ❌ | Replace tags |

---

### publish_content

Publish a draft — makes it live on the website. **Risk: external_write**

**Parameters:**
| Name | Type | Required | Description |
|------|------|:---:|-------------|
| `id` | string | ✅ | Content ID to publish |

---

### unpublish_content

Take published content offline. **Risk: external_write**

**Parameters:**
| Name | Type | Required | Description |
|------|------|:---:|-------------|
| `id` | string | ✅ | Content ID to unpublish |

---

## Social Media

### list_social_accounts

List all connected social media accounts with follower counts and status.

**Parameters:** None

---

### schedule_social_post

Schedule a post to one or more social platforms. **Risk: external_write**

**Parameters:**
| Name | Type | Required | Description |
|------|------|:---:|-------------|
| `platforms` | string[] | ✅ | Platforms: `twitter`, `linkedin`, `instagram`, `facebook` |
| `content` | string | ✅ | Post text |
| `media_url` | string | ❌ | Attached image/video URL |
| `scheduled_at` | string | ✅ | ISO 8601 datetime |

**Example:**
```json
{"name": "schedule_social_post", "arguments": {"platforms": ["twitter", "linkedin"], "content": "Check out our new feature! 🚀", "scheduled_at": "2026-05-27T10:00:00Z"}}
```

---

### get_social_metrics

Get engagement metrics for a social post.

**Parameters:**
| Name | Type | Required | Description |
|------|------|:---:|-------------|
| `id` | string | ✅ | Post ID |

**Returns:** likes, shares, comments, reach, impressions, engagement_rate

---

### list_scheduled_posts

List all upcoming scheduled posts.

**Parameters:** None

---

### delete_scheduled_post

Cancel a scheduled post before it's published.

**Parameters:**
| Name | Type | Required | Description |
|------|------|:---:|-------------|
| `id` | string | ✅ | Scheduled post ID |

---

## Video / YouTube

### list_videos

List all videos with titles, stats, and status.

**Parameters:** None

---

### get_video

Get full video details including description, tags, stats, and transcript.

**Parameters:**
| Name | Type | Required | Description |
|------|------|:---:|-------------|
| `id` | string | ✅ | Video ID |

---

### upload_video

Upload a new video. **Risk: external_write**

**Parameters:**
| Name | Type | Required | Description |
|------|------|:---:|-------------|
| `title` | string | ✅ | Video title |
| `description` | string | ✅ | Video description |
| `tags` | string[] | ❌ | Tags for discovery |
| `privacy` | string | ❌ | `private`, `unlisted`, `public` (default: `private`) |

---

### update_video

Update video metadata.

**Parameters:**
| Name | Type | Required | Description |
|------|------|:---:|-------------|
| `id` | string | ✅ | Video ID |
| `title` | string | ❌ | New title |
| `description` | string | ❌ | New description |
| `tags` | string[] | ❌ | Replace tags |

---

### get_video_analytics

Get video performance metrics.

**Parameters:**
| Name | Type | Required | Description |
|------|------|:---:|-------------|
| `id` | string | ✅ | Video ID |

**Returns:** views, watch_time_hours, avg_retention_pct, ctr_pct, likes, comments

---

## Media Library

### list_media

List all media assets.

**Parameters:**
| Name | Type | Required | Description |
|------|------|:---:|-------------|
| `media_type` | string | ❌ | Filter: `image`, `video`, `file` |

---

### upload_media

Upload a media asset to the library.

**Parameters:**
| Name | Type | Required | Description |
|------|------|:---:|-------------|
| `url` | string | ✅ | URL of the asset to upload |
| `alt_text` | string | ❌ | Alt text for accessibility |
| `filename` | string | ❌ | Display filename |

---

### get_media

Get media asset details and URL.

**Parameters:**
| Name | Type | Required | Description |
|------|------|:---:|-------------|
| `id` | string | ✅ | Media ID |

---

## Taxonomy & SEO

### list_categories

List all categories and tags.

**Parameters:** None

---

### create_category

Create a new category or tag.

**Parameters:**
| Name | Type | Required | Description |
|------|------|:---:|-------------|
| `name` | string | ✅ | Category name |
| `parent_id` | string | ❌ | Parent category ID (for nesting) |

---

### get_seo_metadata

Get SEO metadata for a content item.

**Parameters:**
| Name | Type | Required | Description |
|------|------|:---:|-------------|
| `id` | string | ✅ | Content ID |

**Returns:** seo_title, seo_description, keywords

---

### update_seo

Update SEO metadata for content.

**Parameters:**
| Name | Type | Required | Description |
|------|------|:---:|-------------|
| `content_id` | string | ✅ | Content ID |
| `title` | string | ❌ | SEO title (50-60 chars recommended) |
| `description` | string | ❌ | Meta description (150-160 chars) |
| `keywords` | string[] | ❌ | Target keywords |

---

## Publishing Workflow

### get_content_calendar

View all scheduled content across all channels (blog, social, video).

**Parameters:**
| Name | Type | Required | Description |
|------|------|:---:|-------------|
| `start_date` | string | ❌ | Start of range (ISO date) |
| `end_date` | string | ❌ | End of range (ISO date) |

---

### request_review

Submit content for editorial review.

**Parameters:**
| Name | Type | Required | Description |
|------|------|:---:|-------------|
| `content_id` | string | ✅ | Content ID |
| `note` | string | ❌ | Note for the reviewer |

---

### get_publish_status

Get the current workflow state of content.

**Parameters:**
| Name | Type | Required | Description |
|------|------|:---:|-------------|
| `id` | string | ✅ | Content ID |

**Returns:** status (`draft`, `in_review`, `scheduled`, `published`)
