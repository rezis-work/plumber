# Stitch design exports (Phase 0)

Drop images and binary exports from **Google Stitch** here after fetching.

**Step-by-step prompt for agents:** see **[STITCH_INSTRUCTIONS.md](./STITCH_INSTRUCTIONS.md)** (project ID, Design System screen ID, `curl -L`, and Cursor MCP tool hints).

| | |
|--|--|
| **Stitch project ID** | `9702559548791545108` |
| **Design system (Fixavon)** | **Caucasus Blue** — asset `assets/547841dc1b4545db8471e31333de0ce8` |
| **Legacy / UI stub id** | `asset-stub-assets-547841dc1b4545db8471e31333de0ce8-1775829756314` |

Canonical tokens in repo:

- **CSS variables:** [`src/app.css`](../../src/app.css)
- **JSON (mobile / tooling):** [`src/lib/design/tokens.json`](../../src/lib/design/tokens.json)

Refresh tokens by running Stitch MCP **`list_design_systems`** with `projectId` `9702559548791545108`, then update `tokens.json` and mirror changes into `:root` in `app.css`.

**Phase 0B (marketing landing):** committed assets and MCP notes live under [`../marketing/README.md`](../marketing/README.md) (see **Phase 0B** in [STITCH_INSTRUCTIONS.md](./STITCH_INSTRUCTIONS.md)).
