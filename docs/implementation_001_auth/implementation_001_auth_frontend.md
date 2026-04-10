# Implementation 001 — Auth frontend (SvelteKit)

**Purpose:** Step-by-step instructions to integrate the web app (`apps/web`) with the auth API described in [implementation_001_auth_api.md](./implementation_001_auth_api.md). Instructions only—no embedded code.

**Assumptions:**

- API base URL is configurable (e.g. environment variable for dev vs prod).
- API implements cookie-based refresh and Bearer access tokens as in the API guide.
- You will align **CORS** and **cookie** settings on the API with how the browser reaches the API (same-site vs cross-origin).

---

## Phase A — Environment and API contract

### Step A1 — Configuration

1. Define a single source of truth for **public API origin** (e.g. `PUBLIC_API_URL` or `VITE_*` / SvelteKit `env` pattern your project uses).
2. Document which routes the frontend will call: register, login, refresh, logout, logout-all, me.
3. Decide **cookie behavior** for your deployment:
   - **Same origin** (e.g. reverse proxy: browser sees same host for web and API): cookies are straightforward.
   - **Cross origin** (e.g. web on `localhost:5173`, API on `localhost:3001`): you need API **CORS** with `credentials: true`, explicit **Allowed-Origin** (not `*`), and cookie `SameSite=None; Secure` for HTTPS—or use a dev proxy so the browser thinks it is same-origin.

### Step A2 — CORS and credentials checklist (coordinate with API team)

Ensure the API allows:

- `Authorization` header from browser requests.
- Cookies on refresh/logout if cross-origin credentialed requests are used.
- Preflight succeeds for `POST`/`GET` with your actual headers.

Document the chosen approach in the repo (short ADR or comment in env example).

---

## Phase B — HTTP client and token handling

### Step B1 — Central API client

1. Create a small module (e.g. `lib/api/client.ts`) that wraps `fetch` (or your HTTP library) with:
   - Base URL from config.
   - Default **`credentials: 'include'`** for routes that rely on refresh cookies (login, refresh, logout, logout-all if cookie cleared server-side).
   - JSON `Content-Type` where applicable.
2. Never log full access tokens or passwords.

### Step B2 — Access token storage strategy

Choose one approach and document it:

- **Memory only** (variable/store): safest against XSS persistence; lost on full page reload unless you refresh on load.
- **sessionStorage**: slightly more convenient; still XSS-sensitive—mitigate with CSP and careful deps.
- **localStorage**: generally discouraged for access tokens.

**Recommended:** memory + **silent refresh on app load** using the httpOnly refresh cookie (call `POST /auth/refresh` with credentials) to obtain a new access token after navigation.

### Step B3 — Auth store / session state

1. Create a reactive store (or context) holding:
   - Current user DTO (from `/auth/me`) or `null`.
   - Access token string (if kept in memory) or `null`.
   - Loading / error flags for UX.
2. Expose derived state: `isAuthenticated`, `role`, etc., from verified server data (`/auth/me`), not from client-forged values.

---

## Phase C — Auth actions (user flows)

### Step C1 — Register page

1. Form fields: email, password, role selector limited to **client** and **plumber** (do not offer admin).
2. Client-side validation mirroring API rules (min password length, email shape) to reduce round-trips.
3. On success: optionally auto-login or redirect to login with success message.
4. Map API errors: 409 duplicate email vs 400 validation—safe to show specific messages here (unlike login).

### Step C2 — Login page

1. Form: email, password.
2. On submit: `POST /auth/login` with credentials; API sets refresh cookie; response body contains access token.
3. Store access token per Step B2; then either:
   - Fetch `GET /auth/me` immediately to populate user store, or
   - Decode claims only if you **must** (prefer `/auth/me` for truth).
4. On failure: show **generic** invalid credentials message (no “user not found” vs “wrong password”).

### Step C3 — Bootstrap session on full load

1. In root `+layout.ts` / `+layout.server.ts` / `hooks` (choose the pattern that fits SvelteKit version and SSR needs):
   - If you have no access token in memory (first load), attempt **one** refresh call with `credentials: 'include'` to get a new access token.
   - If refresh fails, treat as logged out (clear state).
2. Avoid infinite refresh loops: cap retries, handle 401 once.

### Step C4 — Access token attachment

1. For authenticated API calls from the browser, attach `Authorization: Bearer <access_token>`.
2. Implement a **401 handler** path: on 401 from a protected resource, try `POST /auth/refresh` once, retry original request; if refresh fails, clear session and redirect to login.

### Step C5 — Logout

1. Call `POST /auth/logout` with credentials so server revokes refresh session and clears cookie.
2. Clear client access token and user store client-side.
3. Redirect to public page.

### Step C6 — Logout everywhere

1. From an account/security UI, call `POST /auth/logout-all` with Bearer access token + credentials if needed for cookie clearing.
2. Clear local session state; redirect to login.

---

## Phase D — Protected routes and RBAC in the UI

### Step D1 — Route protection (server vs client)

1. **Server-side (load functions):** For pages that must not flash private content, validate session early:
   - Either pass cookies/headers through SSR to API (if SSR calls API with refresh or forwarded cookie—complex), or
   - Use client-side guards for MVP and accept a brief flash, **or**
   - Use a **SvelteKit hook** that runs on server and checks a session cookie you control (only if you add one—your API uses httpOnly refresh; SSR may not see refresh JWT if httpOnly—plan accordingly).

2. **Pragmatic MVP:** client-side guard in `+layout.svelte` under `/app` routes: if `!isAuthenticated`, `goto('/login')`.

Document limitations if SSR cannot see httpOnly refresh JWT.

### Step D2 — Role-based UI

1. Hide navigation entries (client vs plumber vs admin) using `role` from `/auth/me`.
2. Do not rely on hidden buttons alone for security; API must still enforce RBAC.

### Step D3 — Admin-only and plumber-only pages

1. Add route segments e.g. `/admin/...`, `/plumber/...`.
2. On navigation, if `role` insufficient, redirect to **403** page or home with message.

---

## Phase E — Security hardening (web)

### Step E1 — Content Security Policy

1. Plan CSP headers (via adapter/host) to reduce XSS risk (especially if access token lives in JS memory).
2. Avoid inline scripts if possible.

### Step E2 — Forms and CSRF

1. For **cookie-authenticated** `POST` from browser, understand CSRF:
   - `SameSite=Lax` helps for many cases; `Strict` stricter.
   - If you use cross-site cookies, evaluate CSRF tokens for state-changing cookie endpoints.

### Step E3 — HTTPS

1. Production: HTTPS only; `Secure` cookies on API.

---

## Phase F — Testing and handoff

### Step F1 — Manual test script

1. Register → login → visit protected page → refresh browser tab → still authenticated (refresh flow).
2. Logout → protected routes inaccessible.
3. Login on two browsers (if applicable); logout-all → both lose refresh.

### Step F2 — Documentation

1. Add `.env.example` entries for API URL.
2. Document known dev proxy setup (e.g. Vite proxy to API) if used.

---

## Frontend deliverables checklist

- [ ] Configured API base URL and credentials mode for cookie endpoints.
- [ ] Login/register/logout flows wired with correct error messaging policy.
- [ ] Access token lifecycle with refresh-on-401 (or equivalent).
- [ ] Protected routes and role-based redirects.
- [ ] No secrets in client bundle; no password/token logging.
