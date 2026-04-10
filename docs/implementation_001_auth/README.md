# Implementation 001 — Authentication & users

This folder contains **step-by-step implementation guides** (instructions only, no embedded code) for adding secure authentication across the Plumber platform: **API first**, then **web**, then **mobile**.

## Documents

| Document | Scope |
|----------|--------|
| [implementation_001_auth_api.md](./implementation_001_auth_api.md) | Rust Axum backend: users, passwords, JWT, refresh sessions, routes, middleware, RBAC |
| [implementation_001_auth_frontend.md](./implementation_001_auth_frontend.md) | SvelteKit web app: API client, tokens, cookies, protected routes, UX |
| [implementation_001_auth_mobile.md](./implementation_001_auth_mobile.md) | Expo/React Native app: secure storage, API client, session lifecycle |

Read the API guide before frontend or mobile so contracts and security rules stay aligned.

## Database (PostgreSQL on Neon)

- **Provider:** [Neon](https://neon.tech) — serverless Postgres, branches for dev/staging, compatible with SQLx and standard Postgres features used in the API guide.
- **Details:** Connection URL shape, **pooled vs direct** endpoints for migrations vs runtime, and operational notes live in [implementation_001_auth_api.md](./implementation_001_auth_api.md) (Neon section under prerequisites).

## Product goals (summary)

- **Roles:** `client`, `plumber`, `admin`.
- **Credentials:** email + password.
- **Access token:** short-lived JWT in `Authorization: Bearer <token>`.
- **Refresh token:** long-lived JWT in **httpOnly** cookie; **sessions persisted in DB** with rotation and revocation.
- **RBAC:** enforce role on protected routes after authentication.
- **Public registration:** only `client` and `plumber`. **Admin** is never created via public API (seed or internal tooling).

## Cross-cutting security rules (all layers)

1. **Passwords:** hash only (server); never log or return plaintext; validate length/complexity before hashing.
2. **Email:** normalize (trim + lowercase) before storage and lookup.
3. **Login errors:** generic “invalid credentials” (do not reveal whether email exists).
4. **Access tokens:** short TTL; only as Bearer; never in URLs or logs.
5. **Refresh tokens:** httpOnly cookie (web); rotation on every refresh; store session metadata + **hashed** refresh identifier in DB; revoke on logout.
6. **Roles in requests:** after verification, trust **claims from the verified access token** (or DB user for `/auth/me`), not raw client input.

## Suggested order of execution

1. Complete **API** guide through Step 12 and verify with HTTP client (curl, Bruno, etc.).
2. Implement **frontend** against the live API contract and CORS/cookie policy.
3. Implement **mobile**, resolving cookie vs secure-storage constraints as described in the mobile guide.

## Long-term product note

Plumbers may self-register; **admin approval** of plumber accounts/profiles can be a follow-up feature (separate from this auth track).
