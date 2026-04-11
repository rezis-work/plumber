# Implementation 001 — Auth frontend (SvelteKit)

**Purpose:** Step-by-step instructions to integrate the web app (`apps/web`) with the auth API described in [implementation_001_auth_api.md](./implementation_001_auth_api.md). Instructions only—no embedded code.

**Assumptions:**

- API base URL is configurable (e.g. environment variable for dev vs prod).
- API implements cookie-based refresh and Bearer access tokens as in the API guide.
- You will align **CORS** and **cookie** settings on the API with how the browser reaches the API (same-site vs cross-origin).

**Order:** (1) **Phase 0** — design tokens from Stitch. (2) **Phase 0B** — marketing landing from Stitch. (3) **Phase A onward** — API client, auth flows, protected routes. **Mobile** reuses the same palette (Phase MS); a native marketing screen can follow the same Fixavon layout when you add a Stitch mobile export.

---

## Phase 0 — Design system (Google Stitch)

Visual design is sourced from **Stitch** (MCP in Cursor). Do this phase **first** so registration, login, and shell layouts use the canonical colors and typography.

### Stitch instructions (reference)

Use the Stitch MCP server to fetch images and code for the project screens, then materialize assets locally.

| | |
|--|--|
| **Project ID** | `9702559548791545108` |
| **Screens (start here)** | **1. Design System** |
| **Design System screen / asset id** | `asset-stub-assets-547841dc1b4545db8471e31333de0ce8-1775829756314` |

### Step 0.1 — Export tokens from Stitch

1. In Cursor, use **Stitch MCP** to list or pull the **Design System** screen for project `9702559548791545108` (asset id above).
2. For any **hosted URLs** (images, exports), download with a utility such as:

   `curl -L -o <local-filename> "<url-from-stitch>"`

3. Store exports in a predictable place (e.g. `apps/web/static/design/` or `docs/design/stitch/`) and **commit** the token summary (CSS variables, Tailwind extension, or a small `tokens.json`) derived from the Design System—not only binary screenshots.

### Step 0.2 — Wire into SvelteKit

1. Define **semantic colors** (primary, surface, error, success, text, border) and typography scale in one place (`app.css`, Tailwind theme, or CSS variables).
2. **Do not** hard-code one-off hex values in auth components; import from the shared theme.
3. Document in a one-line comment or README where Stitch project IDs live so future prompt updates stay traceable.

### Step 0.3 — Shared palette for mobile

**Expo / React Native** must reuse **these same tokens** (duplicate the committed token file or import a shared package if you introduce a monorepo design package later). The mobile guide [Phase MS](./implementation_001_auth_mobile.md#phase-ms--design-system-shared-with-web) points back here.

---

## Phase 0B — Marketing landing page (Stitch)

**Second step** after the Design System: ship a **public marketing / home** experience before (or in parallel with) wiring auth API details. This is **not** authenticated; it drives users toward client signup, plumber application, and login.

### Stitch instructions (Fixavon landing — desktop)

Use the **Stitch MCP** server to pull images and code for this screen, same project as Phase 0.

| | |
|--|--|
| **Project ID** | `9702559548791545108` |
| **Screen** | **Fixavon Landing Page - Desktop** |
| **Screen ID** | `54764d32cd774878a96490bdfc6b3f72` |

### Step 0B.1 — Export assets

1. Fetch **Fixavon Landing Page - Desktop** via Stitch MCP for project `9702559548791545108`.
2. Download any hosted URLs with e.g. `curl -L -o <local-filename> "<url-from-stitch>"` (hero images, logos, icons).
3. Keep exports next to other Stitch assets (e.g. `apps/web/static/marketing/` or under `docs/design/stitch/fixavon/`).

### Step 0B.2 — Implement in SvelteKit

1. Add a **public route** (typically `/` or `/home`) that renders the landing layout; **no** Bearer token and **no** requirement for refresh cookie.
2. Reuse **Phase 0** tokens (colors, type, spacing); align any Stitch-generated markup/CSS with your Svelte components (do not leave auth pages and marketing on different ad-hoc palettes).
3. Wire **primary CTAs** to real routes you will build in Phase C (e.g. “Sign up” → client register, “Become a plumber” → plumber register, “Log in” → login).
4. **Desktop-first:** this Stitch screen is **desktop**; plan a responsive collapse or a follow-up **mobile** Stitch screen for small viewports if the export does not cover them.

### Step 0B.3 — Acceptance

- Landing loads without API auth.
- CTAs navigate to the correct future auth routes (placeholders OK until Phase C exists).
- Visuals match the Stitch export within reasonable engineering tolerance (assets committed, theme tokens used).

---

## Phase A — Environment and API contract

### Step A1 — Configuration

1. Define a single source of truth for **public API origin** (e.g. `PUBLIC_API_URL` or `VITE_*` / SvelteKit `env` pattern your project uses).
2. Document which routes the frontend will call: **`POST /auth/register/client`**, **`POST /auth/register/plumber`**, (later) verify-email, login, refresh, logout, logout-all, me.
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
2. Expose derived state: `isAuthenticated`, `role`, **`isEmailVerified`** (from `/auth/me`), etc., from verified server data—not from client-forged values.

---

## Phase C — Auth actions (user flows)

Registration is **two separate UX paths** (different pages/components and API calls)—do **not** use one form with a “role” toggle for production.

**Stitch (project `9702559548791545108`):** Dedicated Fixavon screens for **client registration** (**C1a**), **verify email** (**C1c**), **plumber registration** (**C1b**), and **login** (**C2**). Use Stitch MCP + `curl -L` for hosted URLs; reuse **Phase 0** tokens on every screen.

### Step C1a — Client signup

Typical routes: `/register` or `/signup`.

#### Stitch — Client Registration (Fixavon)

| | |
|--|--|
| **Project ID** | `9702559548791545108` |
| **Screen** | **Client Registration - Fixavon** |
| **Screen ID** | `b4e85f70dda04fd18cbc2ded66367040` |

1. Use **Stitch MCP** to fetch **Client Registration - Fixavon** (ids above).
2. Download hosted URLs with e.g. `curl -L -o <local-filename> "<url-from-stitch>"`; store under e.g. `apps/web/static/register/client/`.
3. Build the Svelte page from the export; map fields below to the layout; use Phase 0 semantic tokens only.

#### Behaviour and API

1. **Fields:** email, password (confirm-password optional UX-only).
2. **Submit:** `POST /auth/register/client` with JSON `{ email, password }`.
3. **Success:** Navigate to the **verify-email** experience (**Step C1c**). API returns **`email_verification_token`** and expiry—pass token into C1c via state, query, or secure handoff as appropriate; until outbound email exists, **development** builds may surface the token only behind an env gate (see **C1c**).
4. Client-side validation: mirror API rules (min password length, email shape).
5. Map API errors: **409** duplicate email vs **400** validation—specific messages OK here (unlike login).

### Step C1c — Verify email

Typical routes: `/verify-email`, `/register/verify`, or similar.

#### Stitch — Verify Email (Fixavon)

| | |
|--|--|
| **Project ID** | `9702559548791545108` |
| **Screen** | **Verify Email - Fixavon** |
| **Screen ID** | `2895e33e817143538c664b94e7538991` |

1. Use **Stitch MCP** to fetch **Verify Email - Fixavon** (ids above).
2. Download assets with `curl -L` as needed; store e.g. under `apps/web/static/verify-email/`.
3. Implement the page from the export (instructions, token/code field, “Resend” placeholder if product adds it later).

#### Behaviour and API

1. **Entry:** User arrives after **C1a** success, or via **email deep link** (query param or path token) when you add real email sending.
2. **Submit:** When the API exposes **`POST /auth/verify-email`** (see API guide follow-up), send the token from the UI per contract. Until then, this screen is still valuable for **layout and copy**; in **dev**, you may pre-fill or show the token from registration only when `import.meta.env.DEV` (or equivalent)—**never** in production builds.
3. **Success:** Show “Email verified” and route to **login (C2)** or auto-login per product choice.
4. **Failure:** Map **400** / **401** / **410** (expired) per API when implemented; generic friendly copy for users.

### Step C1b — Become a plumber

Typical routes: `/register/plumber`, `/apply/plumber`, or similar.

#### Stitch — Plumber Registration (Fixavon)

Implement this flow from the **Stitch** export so the plumber apply experience matches product design (reuse **Phase 0** tokens).

| | |
|--|--|
| **Project ID** | `9702559548791545108` |
| **Screen** | **Plumber Registration - Fixavon** |
| **Screen ID** | `2c2497ba2dee4162a6abd45a76f45ff0` |

1. Use **Stitch MCP** in Cursor to fetch images and code for **Plumber Registration - Fixavon** (ids above).
2. Download hosted URLs with e.g. `curl -L -o <local-filename> "<url-from-stitch>"`; store under e.g. `apps/web/static/register/plumber/` or your Stitch asset folder.
3. Build the Svelte page from the export: map fields below to the Stitch layout; keep semantic colors/typography from Phase 0 (no one-off palette).

#### Behaviour and API

1. **Fields:** email, password, **full name**, **phone number**, **years of experience** (numeric stepper or validated input)—must match API body.
2. **Submit:** `POST /auth/register/plumber` with JSON matching the API guide.
3. **Success:** distinct confirmation (e.g. “Application received” or redirect to login)—copy should reflect your policy (instant access vs pending admin approval if you add that later).
4. **Validation:** phone format and non-negative years aligned with API; full name length bounds.
5. Same error-mapping policy as client signup for 409/400.

### Step C2 — Login page

Typical routes: `/login`, `/sign-in`, or similar.

#### Stitch — Login (Fixavon)

| | |
|--|--|
| **Project ID** | `9702559548791545108` |
| **Screen** | **Login - Fixavon** |
| **Screen ID** | `3d5928e9ca844ea9a955ca21a06f0f52` |

1. Use **Stitch MCP** to fetch **Login - Fixavon** (ids above).
2. Download hosted URLs with `curl -L`; store e.g. under `apps/web/static/login/`.
3. Build the Svelte page from the export; link **“Sign up”** / **“Become a plumber”** to **C1a** / **C1b** routes; use Phase 0 tokens.

#### Behaviour and API

1. **Form:** email, password.
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

**Implemented (SvelteKit web):** [`apps/web/src/routes/+layout.svelte`](../../apps/web/src/routes/+layout.svelte) runs `hydrateFromRefresh()` from [`apps/web/src/lib/auth/session.svelte.ts`](../../apps/web/src/lib/auth/session.svelte.ts) in `onMount` when `session.accessToken === null` and `browser`. `hydrateFromRefresh` issues a single `POST /auth/refresh` (via [`client.ts`](../../apps/web/src/lib/api/client.ts), default `credentials: 'include'`), then `GET /auth/me`; overlapping calls await one in-flight refresh; any failure clears session without retries. SSR does not run refresh (httpOnly cookie); see [`token-storage.md`](../../apps/web/src/lib/auth/token-storage.md).

### Step C4 — Access token attachment

1. For authenticated API calls from the browser, attach `Authorization: Bearer <access_token>`.
2. Implement a **401 handler** path: on 401 from a protected resource, try `POST /auth/refresh` once, retry original request; if refresh fails, clear session and redirect to login.

**Implemented (SvelteKit web):** [`apps/web/src/lib/api/authenticatedRequest.ts`](../../apps/web/src/lib/api/authenticatedRequest.ts) exports **`apiRequestAuthenticated`** (browser-only): merges Bearer from [`session.svelte.ts`](../../apps/web/src/lib/auth/session.svelte.ts) unless `accessToken` is passed in options; on first **401**, **`authRefresh`** + **`authMe`** via [`client.ts`](../../apps/web/src/lib/api/client.ts), then retries once with the new session token; on refresh/`/me` failure, **`clearSession`** and **`goto`** to **`/login`** (with SvelteKit **`base`**). Low-level **`apiRequest`** remains unchanged for auth routes. See [`routes.md`](../../apps/web/src/lib/api/routes.md).

### Step C5 — Logout

1. Call `POST /auth/logout` with credentials so server revokes refresh session and clears cookie.
2. Clear client access token and user store client-side.
3. Redirect to public page.

**Implemented (SvelteKit web):** [`apps/web/src/lib/auth/logout.ts`](../../apps/web/src/lib/auth/logout.ts) **`logoutFromApp`**: `authLogout()` from [`client.ts`](../../apps/web/src/lib/api/client.ts) (default `credentials: 'include'`), then **`clearSession`**, then **`goto`** to home with SvelteKit **`base`**. API errors still clear client state. Entry point: [`LandingNav.svelte`](../../apps/web/src/lib/marketing/LandingNav.svelte) when signed in.

### Step C6 — Logout everywhere

1. From an account/security UI, call `POST /auth/logout-all` with Bearer access token + credentials if needed for cookie clearing.
2. Clear local session state; redirect to login.

**Implemented (SvelteKit web):** same [`logout.ts`](../../apps/web/src/lib/auth/logout.ts) **`logoutEverywhere`**: `authLogoutAll(session.accessToken)` (skipped if no access token), **`clearSession`**, **`goto`** login with **`base`**. Shown in [`LandingNav.svelte`](../../apps/web/src/lib/marketing/LandingNav.svelte) when `session.accessToken` is set. See [`routes.md`](../../apps/web/src/lib/api/routes.md).

---

## Phase D — Protected routes and RBAC in the UI

### Step D1 — Route protection (server vs client)

1. **Server-side (load functions):** For pages that must not flash private content, validate session early:
   - Either pass cookies/headers through SSR to API (if SSR calls API with refresh or forwarded cookie—complex), or
   - Use client-side guards for MVP and accept a brief flash, **or**
   - Use a **SvelteKit hook** that runs on server and checks a session cookie you control (only if you add one—your API uses httpOnly refresh; SSR may not see refresh JWT if httpOnly—plan accordingly).

2. **Pragmatic MVP:** client-side guard in `+layout.svelte` under `/app` routes: if `!isAuthenticated`, `goto('/login')`.

Document limitations if SSR cannot see httpOnly refresh JWT.

**Implemented (SvelteKit web):** Route groups **`(guest)/`** and **`(protected)/`** under [`apps/web/src/routes/`](../../apps/web/src/routes/). Both use **`export const ssr = false`** in **`+layout.ts`** so session/hydration guards run client-only (httpOnly refresh not available to SSR). **`(protected)/+layout.svelte`**: while **`session.hydrating`** show loading; then if missing **`session.user`** or **`session.accessToken`**, **`goto`** [`/login`](../../apps/web/src/routes/(guest)/login/+page.svelte). **`(guest)/+layout.svelte`**: same hydration awareness; if authenticated, **`goto`** role profile via [`profilePaths.ts`](../../apps/web/src/lib/auth/profilePaths.ts) (covers **`/`**, **`/login`**, **`/register`**, **`/verify-email`**). Minimal signed-in chrome: [`AppShellNav.svelte`](../../apps/web/src/lib/account/AppShellNav.svelte).

### Step D2 — Role-based UI

1. Hide navigation entries (client vs plumber vs admin) using `role` from `/auth/me`.
2. Do not rely on hidden buttons alone for security; API must still enforce RBAC.

**Implemented:** Profile URLs are role-segmented (**`/client/profile`**, **`/plumber/profile`**, **`/admin/profile`**); [`AppShellNav`](../../apps/web/src/lib/account/AppShellNav.svelte) links to the current user’s profile. Marketing [`LandingNav`](../../apps/web/src/lib/marketing/LandingNav.svelte) remains on guest landing only (logged-in users are redirected off guest routes).

### Step D3 — Admin-only and plumber-only pages

1. Add route segments e.g. `/admin/...`, `/plumber/...`.
2. On navigation, if `role` insufficient, redirect to **403** page or home with message.

**Implemented:** [`(protected)/client/+layout.svelte`](../../apps/web/src/routes/(protected)/client/+layout.svelte), [`plumber/+layout.svelte`](../../apps/web/src/routes/(protected)/plumber/+layout.svelte), [`admin/+layout.svelte`](../../apps/web/src/routes/(protected)/admin/+layout.svelte) send wrong **`role`** to **`/forbidden`**. Public [`forbidden/+page.svelte`](../../apps/web/src/routes/forbidden/+page.svelte) (access denied copy; link to profile if signed in, else login + home).

**Profiles:** Each **`profile/+page.svelte`** calls **`GET /auth/me`** via [`apiRequestAuthenticated`](../../apps/web/src/lib/api/authenticatedRequest.ts), updates **`session.user`**, and renders [`ProfileMePanel.svelte`](../../apps/web/src/lib/account/ProfileMePanel.svelte) (account fields, plumber **`profile`** when present, **Log out** / **Log out everywhere** via [`logout.ts`](../../apps/web/src/lib/auth/logout.ts)). Post-login redirect: [`(guest)/login/+page.svelte`](../../apps/web/src/routes/(guest)/login/+page.svelte) **`goto`** role profile.

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

**Status (web MVP):** **E1** — **Content-Security-Policy** (and related) headers are **not** set by the SvelteKit app yet; add at **adapter/host** or **`hooks.server.ts`** for production hardening; prefer avoiding new inline scripts. **E2** — **SameSite** / cross-origin cookie **`POST`** behavior is covered in [ADR 001 — CORS and cookies](./adr_001_cors_and_cookies.md); refresh cookie is **httpOnly** on **`/auth`**. **E3** — **HTTPS** and **`Secure`** cookies are enforced via **API** deployment config, not only the frontend repo.

---

## Phase F — Testing and handoff

### Step F1 — Manual test script

1. **Client:** register via client endpoint → see verification UX (and dev token if applicable) → login (per your API policy for unverified users) → protected page → refresh tab → still authenticated.
2. **Plumber:** register via plumber endpoint with full profile → login → plumber-only UI if applicable.
3. Logout → protected routes inaccessible.
4. Login on two browsers (if applicable); logout-all → both lose refresh.

### Step F2 — Documentation

1. Add `.env.example` entries for API URL.
2. Document known dev proxy setup (e.g. Vite proxy to API) if used.

**Status:** [`apps/web/.env.example`](../../apps/web/.env.example) documents **`PUBLIC_API_URL`** (optional; empty = same-origin `/auth/...`). Dev proxy: [`apps/web/vite.config.ts`](../../apps/web/vite.config.ts) (`/auth` → API); see also [ADR 001](./adr_001_cors_and_cookies.md). **F1** remains a **manual** regression script for humans before release.

---

## Frontend deliverables checklist

- [x] Configured API base URL and credentials mode for cookie endpoints ([`publicOrigin.ts`](../../apps/web/src/lib/api/publicOrigin.ts), [`client.ts`](../../apps/web/src/lib/api/client.ts) defaults, `.env.example`, proxy).
- [x] **C3:** Bootstrap session on full load (silent refresh + `/auth/me` when no in-memory access token; see Step C3 above).
- [x] **C4:** Bearer on protected calls + 401 → one refresh + retry; clear session + redirect to login if refresh fails (see Step C4 above).
- [x] **C5:** Logout — `POST /auth/logout`, clear session, redirect public (see Step C5 above).
- [x] **C6:** Logout everywhere — `POST /auth/logout-all`, clear session, redirect login (see Step C6 above).
- [x] **Separate** client signup vs **become plumber** flows (routes + forms + API calls): [`(guest)/register`](../../apps/web/src/routes/(guest)/register/+page.svelte), [`(guest)/register/plumber`](../../apps/web/src/routes/(guest)/register/plumber/+page.svelte).
- [x] Client post-signup **email verification** UX; dev token surfaced when API returns it: [`(guest)/verify-email`](../../apps/web/src/routes/(guest)/verify-email/+page.svelte).
- [x] Login/logout flows wired with correct error messaging policy (login/register per Phase C; logout C5/C6 in nav).
- [x] Access token lifecycle with refresh-on-401 (or equivalent); see Step C4.
- [x] Protected routes and role-based redirects; symmetric guest vs protected guards; role profiles and `/forbidden` (Phase D above).
- [x] No secrets in client bundle; no password/token logging in production (only **`PUBLIC_`** in `.env.example`; [`client.ts`](../../apps/web/src/lib/api/client.ts) documents no token/password logging; avoid adding `console.log` around auth payloads).
