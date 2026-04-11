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

See [implementation_001_auth_mobile.md — Step MS.3](../../../docs/implementation_001_auth/implementation_001_auth_mobile.md). Place downloads under `assets/stitch/<folder>/`. See [`assets/stitch/README.md`](../assets/stitch/README.md): committed PNGs are **placeholders** until Stitch exports replace them.

| Screen | Screen ID | Folder |
|--------|-----------|--------|
| Fixavon Landing Page - Mobile | `bd10bc970a374954b5a531dafcb83847` | `assets/stitch/landing-mobile/` |
| Client Registration - Mobile | `1c635e4ac20c483588a505eb52dcb53c` | `assets/stitch/register-client-mobile/` |
| Plumber Registration - Mobile | `99473da95838498aa626cd37f11153b7` | `assets/stitch/register-plumber-mobile/` |
| Verify Email - Mobile | `5c38516278094c3ca521320fbd3edf1a` | `assets/stitch/verify-email-mobile/` |
| Login - Mobile | `669b5b6b8ed24c179ed828e95c4fc88a` | `assets/stitch/login-mobile/` |

### MS.3 status (Expo Router)

| Stitch screen | Route | Raster files used in UI |
|---------------|-------|-------------------------|
| Landing (mobile) | `/` — `app/(guest)/index.tsx` | `landing-mobile/coverage-map.png` (from Stitch HTML export); `hero.png` = MCP screenshot URL; `reference.html` = MCP `htmlCode` export |
| Client registration | `/register` — `app/(guest)/register/index.tsx` | `register-client-mobile/illustration.png` (MCP screenshot); `reference.html` = MCP `htmlCode` |
| Plumber registration | `/register/plumber` — `app/(guest)/register/plumber.tsx` | `register-plumber-mobile/illustration.png` (MCP screenshot); `reference.html` = MCP `htmlCode` |
| Verify email | `/verify-email` — `app/(guest)/verify-email.tsx` | `verify-email-mobile/illustration.png` (MCP screenshot); `reference.html` = MCP `htmlCode` |
| Login | `/login` — `app/(guest)/login.tsx` | `login-mobile/illustration.png` (MCP screenshot); `reference.html` = MCP `htmlCode` |

**Typography:** Web uses **Inter** (`app.html`). MS.3 screens use the **system sans-serif** for now. For pixel parity, add **`expo-font`** + **`@expo-google-fonts/inter`** in a follow-up.
