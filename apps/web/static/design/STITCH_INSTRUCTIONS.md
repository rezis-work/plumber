# Stitch instructions (Design System)

Use this block when asking an agent to pull **images and code** from Stitch.

## Stitch Instructions

Get the images and code for the following Stitch project's screens:

## Project

ID: `9702559548791545108`

## Screens

1. **Design System**  
   ID: `asset-stub-assets-547841dc1b4545db8471e31333de0ce8-1775829756314`

Use a utility like `curl -L` to download the hosted URLs.

### Phase 0B — Fixavon Landing Page (Desktop)

| | |
|--|--|
| **Screen** | **Fixavon Landing Page - Desktop** |
| **Screen ID** | `54764d32cd774878a96490bdfc6b3f72` |

1. **MCP:** `list_screens` (`projectId` `9702559548791545108`) then **`get_screen`** with `name` `projects/9702559548791545108/screens/54764d32cd774878a96490bdfc6b3f72`, plus `projectId` and `screenId` (both required by the Stitch MCP `get_screen` tool schema in Cursor).
2. **`curl -L`** `screenshot.downloadUrl` → e.g. `static/marketing/landing-reference.png`; `htmlCode.downloadUrl` → `static/marketing/landing-export.html`; embed any `aida-public` image URLs from the HTML into the same folder (see [`static/marketing/README.md`](../marketing/README.md)).

---

## Cursor MCP (same project)

In Cursor, the **Stitch** MCP server (`user-stitch`) can fetch data directly:

| Goal | Tool | Notes |
|------|------|--------|
| Theme + palette + DTCG-style tokens | `list_design_systems` | `projectId`: `9702559548791545108` — returns **Caucasus Blue** (`assets/547841dc1b4545db8471e31333de0ce8`) |
| HTML + screenshot URLs per screen | `list_screens` then `get_screen` | Use `screenshot.downloadUrl` / `htmlCode.downloadUrl` with `curl -L` |

Canonical web tokens: [`src/lib/design/tokens.json`](../../src/lib/design/tokens.json) and [`src/app.css`](../../src/app.css).
