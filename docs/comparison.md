# Competitive Comparison

How mcp-cms compares to other CMS and content MCP servers.

## Overview Matrix

| Server | Tools | Content | Social | Video | Media | SEO | Workflow | Multi-backend | Language |
|--------|:-----:|:-------:|:------:|:-----:|:-----:|:---:|:--------:|:-------------:|----------|
| **mcp-cms** | **26** | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ 7 backends | Rust |
| Ghost CMS MCP | 44 | ✅ | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ Ghost only | TypeScript |
| Sanity MCP | 41 | ✅ | ❌ | ❌ | ✅ | ❌ | ✅ | ❌ Sanity only | TypeScript |
| qiq/social | 25 | ✅ | ✅ | ❌ | ❌ | ❌ | ✅ | ❌ Own platform | TypeScript |
| WordPress MCP | 20 | ✅ | ❌ | ❌ | ✅ | ❌ | ❌ | ❌ WordPress only | TypeScript |
| Strapi MCP | 20 | ✅ | ❌ | ❌ | ✅ | ❌ | ✅ | ❌ Strapi only | TypeScript |
| Webflow MCP | 19 | ✅ | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ Webflow only | TypeScript |
| YouTube MCP | 20 | ❌ | ❌ | ✅ | ❌ | ❌ | ❌ | ❌ YouTube only | TypeScript |
| Contentful MCP | 3 | ✅ | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ Contentful only | TypeScript |
| PostPulse | 4 | ❌ | ✅ | ❌ | ❌ | ❌ | ❌ | ✅ Multi-social | TypeScript |

## Key Differentiators

### 1. Unified Content + Social + Video

Every other MCP server handles **one thing**:
- Ghost MCP = Ghost blog only
- YouTube MCP = YouTube only
- WordPress MCP = WordPress only
- PostPulse = social scheduling only

**mcp-cms** is the only server that unifies content management, social media, and video in one tool set. An agent can create a blog post, schedule social promotion, and upload a companion video — all in one session.

### 2. Multi-Backend (7 platforms)

| mcp-cms backends | Competitors |
|-----------------|-------------|
| WordPress | WordPress MCP (single-platform) |
| Contentful | Contentful MCP (3 tools only) |
| YouTube | YouTube MCP (single-platform) |
| Twitter/X | No dedicated CMS+social combo |
| LinkedIn | No MCP exists |
| Meta (FB/IG) | No dedicated CMS+social combo |
| Custom API | Unique — build your own |

Competitors lock you into one platform. mcp-cms lets you mix: WordPress for blog + YouTube for video + Twitter for social.

### 3. Custom API Spec + Reference Backend

No other CMS MCP provides:
- A documented REST API spec you can implement
- A working reference backend (Rust + SQLite + React)
- The ability to build your own backend in any language

### 4. Governance

| Feature | mcp-cms | Others |
|---------|:-------:|:------:|
| Risk classification per tool | ✅ | ❌ |
| External write flagging (publish, social) | ✅ | ❌ |
| Publishing workflow (draft → review → publish) | ✅ | ⚠️ Sanity, Strapi |
| No data stored (pure API client) | ✅ | ❌ most store state |

### 5. Performance & Deployment

| | mcp-cms | Typical competitor |
|---|:---:|:---:|
| Language | Rust | TypeScript/Node |
| Binary size | ~8MB | ~100MB+ (node_modules) |
| Startup time | <50ms | 1-3s |
| Memory usage | ~10MB | ~80-150MB |
| Dependencies | Minimal (reqwest, rmcp) | Heavy (npm ecosystem) |

## Detailed Comparisons

### vs. Ghost CMS MCP (44 tools)

Ghost MCP has more tools but is **Ghost-only**:
- ✅ Deep Ghost integration (members, tiers, newsletters, offers)
- ❌ No social media
- ❌ No video
- ❌ No SEO tools
- ❌ No multi-backend
- ❌ No custom API option

**When to use Ghost MCP:** You're all-in on Ghost and need member management.
**When to use mcp-cms:** You need content + social + video, or use WordPress/Contentful.

### vs. Sanity MCP (41 tools)

Sanity MCP is powerful but **Sanity-only**:
- ✅ Schema management, versioning, image generation
- ✅ Publishing workflow
- ❌ No social media
- ❌ No video (YouTube)
- ❌ No SEO metadata tools
- ❌ Sanity-specific (GROQ queries, structured content)

**When to use Sanity MCP:** You're on Sanity and need schema/version management.
**When to use mcp-cms:** You need cross-platform content distribution.

### vs. qiq/social (25 tools)

qiq/social focuses on social automation:
- ✅ Multi-platform social posting
- ✅ Automations and RSS feeds
- ❌ No blog/article management
- ❌ No video (YouTube)
- ❌ No media library
- ❌ No SEO
- ❌ Locked to qiq platform

**When to use qiq/social:** You only need social media automation.
**When to use mcp-cms:** You need the full content lifecycle (create → publish → promote).

### vs. WordPress MCP (20 tools)

- ✅ Good WordPress coverage (posts, pages, media, comments, taxonomy)
- ❌ WordPress only
- ❌ No social
- ❌ No video
- ❌ No SEO tools
- ❌ No publishing workflow

**When to use WordPress MCP:** You only use WordPress and need comment/user management.
**When to use mcp-cms:** You use WordPress AND want social/video/SEO from the same agent.

### vs. YouTube MCP (20 tools)

- ✅ Deep YouTube integration (playlists, comments, captions)
- ❌ YouTube only
- ❌ No content management
- ❌ No social

**When to use YouTube MCP:** You only manage YouTube and need playlist/caption tools.
**When to use mcp-cms:** You want video as part of a broader content strategy.

## Summary

| Need | Best choice |
|------|-------------|
| Ghost-only blog with members | Ghost CMS MCP |
| Sanity with schema management | Sanity MCP |
| Social-only automation | qiq/social |
| YouTube-only deep management | YouTube MCP |
| **Unified content + social + video** | **mcp-cms** |
| **Multi-platform (WordPress + YouTube + Twitter)** | **mcp-cms** |
| **Custom backend / self-hosted** | **mcp-cms** |
| **Governed publishing with risk levels** | **mcp-cms** |
