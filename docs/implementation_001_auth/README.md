# Implementation 001 — Authentication & users

This folder contains **step-by-step implementation guides** (instructions only, no embedded code) for adding secure authentication across the Plumber platform: **API first**, then **web**, then **mobile**.

## Documents

| Document | Scope |
|----------|--------|
| [implementation_001_auth_api.md](./implementation_001_auth_api.md) | Rust Axum backend: users, passwords, JWT, refresh sessions, routes, middleware, RBAC |
| [implementation_001_auth_frontend.md](./implementation_001_auth_frontend.md) | SvelteKit web app: API client, tokens, cookies, protected routes, UX |
| [implementation_001_auth_mobile.md](./implementation_001_auth_mobile.md) | Expo/React Native app: secure storage, API client, session lifecycle |

## ADRs

| ADR | Topic |
|-----|--------|
| [adr_001_cors_and_cookies.md](./adr_001_cors_and_cookies.md) | Web: Vite proxy, CORS, SameSite / Secure refresh cookies |
| [adr_002_mobile_refresh_transport.md](./adr_002_mobile_refresh_transport.md) | Mobile: **Option B** native refresh (JSON + SecureStore); API contract intent |

Read the API guide before frontend or mobile so contracts and security rules stay aligned.

## Database (PostgreSQL on Neon)

- **Provider:** [Neon](https://neon.tech) — serverless Postgres, branches for dev/staging, compatible with SQLx and standard Postgres features used in the API guide.
- **Details:** Connection URL shape, **pooled vs direct** endpoints for migrations vs runtime, and operational notes live in [implementation_001_auth_api.md](./implementation_001_auth_api.md) (Neon section under prerequisites).

## Product goals (summary)

- **Roles:** `client`, `plumber`, `admin`.
- **Credentials:** email + password (plus plumber-specific profile fields for the plumber flow).
- **Registration is split by role (different API + UI):**
  - **Client:** `POST /auth/register/client` — body: **email + password** only. User is created with **`is_email_verified = false`**. Response includes a **one-time email verification token** (and expiry) for the client to complete verification later; **sending email is a later step**—for now the token is only returned in JSON.
  - **Plumber (become a plumber):** `POST /auth/register/plumber` — body: **email, password, full name, phone number, years of experience** (not a generic “pick role” signup). Distinct **UI** from client signup.
- **Access token:** short-lived JWT in `Authorization: Bearer <token>`.
- **Refresh token:** long-lived JWT in **httpOnly** cookie; **sessions persisted in DB** with rotation and revocation.
- **RBAC:** enforce role on protected routes after authentication.
- **Public registration:** only **client** and **plumber** flows above. **Admin** is never created via public API (seed or internal tooling).

## Cross-cutting security rules (all layers)

1. **Passwords:** hash only (server); never log or return plaintext; validate length/complexity before hashing.
2. **Email:** normalize (trim + lowercase) before storage and lookup.
3. **Login errors:** generic “invalid credentials” (do not reveal whether email exists).
4. **Access tokens:** short TTL; only as Bearer; never in URLs or logs.
5. **Refresh tokens:** httpOnly cookie (web); rotation on every refresh; store session metadata + **hashed** refresh identifier in DB; revoke on logout.
6. **Roles in requests:** after verification, trust **claims from the verified access token** (or DB user for `/auth/me`), not raw client input.

## Suggested order of execution

1. Complete **API** guide through Step 12 and verify with HTTP client (curl, Bruno, etc.).
2. **Design system (first frontend step):** Use **Google Stitch** (MCP in Cursor)—project `9702559548791545108`, **Design System** (`asset-stub-assets-547841dc1b4545db8471e31333de0ce8-1775829756314`). Export tokens; `curl -L` for hosted URLs. Align **mobile** tokens: [implementation_001_auth_mobile.md](./implementation_001_auth_mobile.md) **Phase MS**.
3. **Marketing landing (second frontend step):** Same Stitch project—**Fixavon Landing Page - Desktop** (`54764d32cd774878a96490bdfc6b3f72`). Public SvelteKit route, CTAs to auth flows. See [implementation_001_auth_frontend.md](./implementation_001_auth_frontend.md) **Phase 0B**.
4. Implement **frontend** auth (API client, CORS/cookies, protected routes) per the frontend guide. **Stitch Fixavon screens (same project):** client signup [C1a](implementation_001_auth_frontend.md#step-c1a--client-signup) (`b4e85f70dda04fd18cbc2ded66367040`), verify email [C1c](implementation_001_auth_frontend.md#step-c1c--verify-email) (`2895e33e817143538c664b94e7538991`), plumber [C1b](implementation_001_auth_frontend.md#step-c1b--become-a-plumber) (`2c2497ba2dee4162a6abd45a76f45ff0`), login [C2](implementation_001_auth_frontend.md#step-c2--login-page) (`3d5928e9ca844ea9a955ca21a06f0f52`).
5. Implement **mobile** per [ADR 002](./adr_002_mobile_refresh_transport.md) and the mobile guide (**native refresh + SecureStore**; API extensions as documented there), reusing the same design tokens as the web app.

## Long-term product note

Plumbers self-serve via the **become a plumber** application endpoint; **admin approval** of plumber accounts/profiles remains a natural follow-up (separate migration/feature). Client **email verification** consume endpoint and outbound email are documented as follow-ups in the API guide after Step 3.
