# Stitch instructions (mobile — Fixavon)

Use **Stitch MCP** in Cursor, then **`curl -L -o <file> "<url>"`** for hosted asset URLs (same workflow as `apps/web/static/design/STITCH_INSTRUCTIONS.md`).

## Design System (tokens)

| | |
|--|--|
| **Project ID** | `9702559548791545108` |
| **Design System screen** | **1. Design System** |
| **Screen / asset ID** | `asset-stub-assets-547841dc1b4545db8471e31333de0ce8-1775829756314` |

Machine-readable tokens live in [`src/design/tokens.json`](../src/design/tokens.json) (copy of web `src/lib/design/tokens.json`).

## Mobile marketing and auth screens (Phase MS.3)

See [implementation_001_auth_mobile.md — Step MS.3](../../../docs/implementation_001_auth/implementation_001_auth_mobile.md). Place downloads under `assets/stitch/<folder>/` (folders are scaffolded with `.gitkeep`).

| Screen | Screen ID | Folder |
|--------|-----------|--------|
| Fixavon Landing Page - Mobile | `bd10bc970a374954b5a531dafcb83847` | `assets/stitch/landing-mobile/` |
| Client Registration - Mobile | `1c635e4ac20c483588a505eb52dcb53c` | `assets/stitch/register-client-mobile/` |
| Plumber Registration - Mobile | `99473da95838498aa626cd37f11153b7` | `assets/stitch/register-plumber-mobile/` |
| Verify Email - Mobile | `5c38516278094c3ca521320fbd3edf1a` | `assets/stitch/verify-email-mobile/` |
| Login - Mobile | `669b5b6b8ed24c179ed828e95c4fc88a` | `assets/stitch/login-mobile/` |

**Typography:** Web uses **Inter** (`app.html`). For pixel parity on native, add **`expo-font`** + **@expo-google-fonts/inter** in a follow-up; until then the app uses the system sans-serif.
