# Fixavon (mobile)

Expo app for native clients. **Navigation:** [Expo Router](https://docs.expo.dev/router/introduction/) — guest marketing/auth routes under [`app/(guest)/`](app/(guest)/) (`/`, `/register`, `/register/plumber`, `/verify-email`, `/login`). **Design system (Phase MS):** shared palette with web via [`src/design/tokens.json`](src/design/tokens.json) and [`src/theme/`](src/theme/).

- **Stitch MCP + asset layout:** [`docs/STITCH_INSTRUCTIONS.md`](docs/STITCH_INSTRUCTIONS.md)
- **Implementation guide:** [`docs/implementation_001_auth/implementation_001_auth_mobile.md`](../../docs/implementation_001_auth/implementation_001_auth_mobile.md) (from monorepo root)

## API base URL (Phase M1)

Simulators and devices usually **cannot** call `localhost` on your machine. Use **HTTPS** tunneling (e.g. **ngrok**) to expose the API, then point the app at that origin.

1. Copy [`.env.example`](.env.example) to **`.env`** (gitignored).
2. Set **`EXPO_PUBLIC_API_URL`** to the HTTPS origin **without** a trailing slash, e.g. `https://abc123.ngrok-free.app`.
3. Restart **`pnpm dev`** after changing `.env`.

Runtime helpers: [`src/config/apiBaseUrl.ts`](src/config/apiBaseUrl.ts) — `apiBaseUrl`, `apiUrl('/auth/login')`. [`app.config.ts`](app.config.ts) also sets `extra.apiUrl` for inspection via `expo-constants`.

**TLS:** Prefer HTTPS (ngrok default). Plain `http://` to a LAN IP needs Android cleartext and ATS exceptions — avoid if you can use HTTPS.

```bash
pnpm dev
```
