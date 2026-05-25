# CMS Example Backend

A reference implementation of the mcp-cms Custom API spec. Built with **Rust (Axum) + SQLite + React**.

## Quick Start

```bash
cd example-backend
cargo run
```

Then:
- **Dashboard:** http://localhost:7799
- **API:** http://localhost:7799/api/v1

## Connect mcp-cms

```bash
CMS_API_URL="http://localhost:7799/api/v1" mcp-cms
```

Or in your MCP config:
```json
{
  "mcpServers": {
    "cms": {
      "command": "mcp-cms",
      "env": { "CMS_API_URL": "http://localhost:7799/api/v1" }
    }
  }
}
```

## What's Included

- **SQLite database** (`cms.db`) — auto-created with schema on first run
- **Seeded data** — 4 articles, 2 social posts, 2 videos, 3 media assets, 4 categories
- **React dashboard** — dark theme, shows all content at a glance
- **Full API** — implements all 26 endpoints from the mcp-cms spec

## API Endpoints

| Method | Path | Description |
|--------|------|-------------|
| GET | /content | List content |
| GET | /content/:id | Get content |
| POST | /content | Create content |
| PATCH | /content/:id | Update content |
| POST | /content/:id/publish | Publish |
| POST | /content/:id/unpublish | Unpublish |
| POST | /content/:id/review | Request review |
| GET | /content/:id/seo | Get SEO |
| PATCH | /content/:id/seo | Update SEO |
| GET | /content/:id/status | Get status |
| GET | /social/accounts | List accounts |
| POST | /social/posts | Schedule post |
| GET | /social/posts/scheduled | List scheduled |
| DELETE | /social/posts/:id | Delete post |
| GET | /social/metrics | Get metrics |
| GET | /videos | List videos |
| GET | /videos/:id | Get video |
| POST | /videos | Upload video |
| PATCH | /videos/:id | Update video |
| GET | /videos/:id/analytics | Video analytics |
| GET | /media | List media |
| POST | /media | Upload media |
| GET | /media/:id | Get media |
| GET | /categories | List categories |
| POST | /categories | Create category |
| GET | /calendar | Content calendar |

## Tech Stack

- **Axum** — async web framework
- **SQLite** (rusqlite with bundled) — zero-config database
- **React 18** — CDN-loaded, no build step
- **tower-http** — CORS + static file serving

## Customizing

Fork this as a starting point for your own CMS backend. The schema is simple — extend the tables, add auth, connect to your actual content sources.
