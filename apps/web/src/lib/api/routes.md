# Auth API routes (browser / Phase B client)

Base path: `/auth` on the Rust API (default **http://127.0.0.1:3001**). With the Vite dev proxy, call **same-origin** paths `/auth/...` from the browser.

**Typed client:** [`client.ts`](./client.ts) (`authLogin`, `authRegisterClient`, `authRegisterPlumber`, `authVerifyEmail`, `authRefresh`, `authLogout`, `authLogoutAll`, `authMe`) and [`types.ts`](./types.ts). After **`authLogin`**, the web app calls **`setSessionFromLogin`** in [`session.svelte.ts`](../../lib/auth/session.svelte.ts) (in-memory access token + `GET /auth/me`); the refresh JWT stays in the httpOnly cookie from the API.

**Protected resources (browser):** use [`authenticatedRequest.ts`](./authenticatedRequest.ts) **`apiRequestAuthenticated`**, which attaches `Authorization: Bearer` from [`session`](../../lib/auth/session.svelte.ts) (unless you pass `accessToken` explicitly), and on the first **401** runs one cookie-backed **`POST /auth/refresh`**, reloads user via **`GET /auth/me`**, retries the original request once, then **`clearSession`** + **`goto`** [`/login`](../../routes/login/+page.svelte) if refresh fails. **`apiRequest`** stays for public and auth endpoints (no session import, no 401 loop). **`apiRequestAuthenticated` throws if called when not in the browser** (SSR `load` must not rely on it without forwarding cookies).

**Logout (browser):** [`logout.ts`](../../lib/auth/logout.ts) — **`logoutFromApp`** (`POST /auth/logout` with credentials, **`clearSession`**, **`goto`** home) and **`logoutEverywhere`** (`POST /auth/logout-all` with Bearer + credentials when `session.accessToken` is set, **`clearSession`**, **`goto`** login). Marketing nav: [`LandingNav.svelte`](../../lib/marketing/LandingNav.svelte).

**Phase D (routes):** Guest-only subtree **`(guest)/`** (landing, login, register, verify-email) vs **`(protected)/`** (`/client/profile`, `/plumber/profile`, `/admin/profile`). Helpers: [`profilePaths.ts`](../../lib/auth/profilePaths.ts). See [implementation doc](../../../../docs/implementation_001_auth/implementation_001_auth_frontend.md) Phase D.

| Method | Path | Notes |
|--------|------|--------|
| `POST` | `/auth/register/client` | JSON `{ email, password }` — see `authRegisterClient` in [`client.ts`](./client.ts) |
| `POST` | `/auth/register/plumber` | JSON `{ email, password, full_name, phone, years_of_experience }` — `authRegisterPlumber` |
| `POST` | `/auth/verify-email` | JSON `{ token }` (64-char hex) — `authVerifyEmail`; errors: [API doc](../../../docs/implementation_001_auth/implementation_001_auth_api.md) |
| `POST` | `/auth/login` | JSON `{ email, password }`; sets httpOnly refresh cookie |
| `POST` | `/auth/refresh` | Uses refresh cookie; returns new access token |
| `POST` | `/auth/logout` | Clears refresh session |
| `GET` | `/auth/me` | Bearer access token |
| `POST` | `/auth/logout-all` | Bearer access token |
