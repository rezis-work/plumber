# ADR 001 — CORS, credentials, and refresh cookies (web + API)

## Context

The auth API issues an **httpOnly** refresh cookie on login (`Path=/auth`, configurable via `AUTH_REFRESH_COOKIE_*`). The SvelteKit app calls `/auth/*` from the browser with `credentials: 'include'` for login, refresh, and logout.

Browsers enforce **same-origin policy** and **SameSite** rules. Cross-origin XHR/fetch with cookies requires explicit **CORS** on the API and compatible cookie attributes.

## Decision — local development (default)

1. **Vite dev server** proxies **`/auth` → `http://127.0.0.1:3001/auth`** (path unchanged).
2. The browser uses **same-origin** URLs such as `POST /auth/login` on `http://localhost:5173`.
3. **`PUBLIC_API_URL` is unset** so the client uses relative paths (see `apps/web/src/lib/api/publicOrigin.ts`).
4. Refresh cookies with **`Path=/auth`** and **`SameSite=Lax`** work for this setup.

## Alternatives — cross-origin

1. Set **`PUBLIC_API_URL`** to the API origin (e.g. `http://localhost:3001`).
2. Set **`CORS_ALLOWED_ORIGINS`** on the API to an explicit list (comma-separated), e.g. `http://localhost:5173`. No wildcard with credentials.
3. Use **`AUTH_REFRESH_COOKIE_SAMESITE=none`** and **`AUTH_REFRESH_COOKIE_SECURE=true`**. **`SameSite=None` requires `Secure`**, so this is intended for **HTTPS** deployments. Plain HTTP localhost is a poor fit for third-party cookie semantics.

## Production

- **Preferred:** One public host (reverse proxy) so the browser sees **same origin** for HTML and `/auth/*`; keep cookies **Lax** or **Strict** as appropriate.
- **Or:** Separate web/API hosts with HTTPS, **`CORS_ALLOWED_ORIGINS`**, and **`None` + `Secure`** cookies.

## Checklist (Phase A2)

| Requirement | How we satisfy it |
|-------------|-------------------|
| `Authorization` header from browser | CORS `allow_headers`: `authorization`, `content-type` when `CORS_ALLOWED_ORIGINS` is set |
| Cookies on refresh/logout when cross-origin | `allow_credentials(true)` + non-wildcard `Allow-Origin` + cookie `SameSite=None; Secure` over HTTPS |
| Preflight for `POST`/`GET` | `OPTIONS` allowed; methods `GET`, `POST`, `OPTIONS` |

## References

- `apps/web/vite.config.ts` — dev proxy
- `apps/api/src/main.rs` — `CORS_ALLOWED_ORIGINS` → `tower_http::cors::CorsLayer`
- `apps/api/src/modules/auth/cookie_config.rs` — refresh cookie attributes
