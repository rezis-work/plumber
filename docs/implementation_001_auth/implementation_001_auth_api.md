# Implementation 001 — Auth API (Rust Axum)

**Purpose:** Step-by-step instructions to implement authentication and user foundation in `apps/api`. This document describes **what** to build and **how to verify** it—not source code.

**Prerequisites:** SQLx with **PostgreSQL**, migrations folder, Axum app that can mount routers and share state (config, pools).

### PostgreSQL on Neon (required stack)

Use **[Neon](https://neon.tech)** as the managed Postgres backend.

1. **Projects and branches:** Create a Neon project; use separate **branches** (or projects) for local dev, preview, and production so migrations and data stay isolated.
2. **Connection string:** Store the full URL in `DATABASE_URL` (or split host/user/password if your config layer prefers). Never commit secrets; use `.env` / deployment env only.
3. **Pooled vs direct URL:** Neon provides a **pooled** connection string (via their proxy, e.g. `-pooler` host) ideal for **runtime** app pools and serverless-style many-short-connections. For **`sqlx migrate run`** and some DDL-heavy operations, Neon recommends using the **direct** (non-pooler) connection when you hit pooler limitations—document both URLs in your internal setup (e.g. `DATABASE_URL` pooled for the API, `DATABASE_URL_DIRECT` for migrations only in CI/local).
4. **TLS:** Use the connection string as Neon provides it; require SSL for remote connections.
5. **SQLx:** Use `PgPool` against Neon like any Postgres host. For `sqlx-cli` offline mode / compile-time checks, keep `sqlx-data.json` or use `SQLX_OFFLINE` per your team policy; run `cargo sqlx prepare` against a schema that matches Neon (e.g. after migrations on a dev branch).
6. **Extensions:** If you rely on `pgcrypto` for `gen_random_uuid()`, enable it in Neon for the database (Neon supports common extensions—verify in dashboard or SQL `CREATE EXTENSION` where allowed). If you use `citext`, enable that extension before migrations that create `CITEXT` columns.
7. **Limits:** Respect Neon **compute** and **connection** limits; prefer the pooler URL for the API and a small, bounded pool size in Rust.

**Recommended module layout:**

- `modules/users/` — domain `User` model, user persistence (repository).
- `modules/auth/` — routes, handlers, services, auth-specific DTOs, JWT claims, password helpers, cookie helpers, middleware, RBAC, refresh-token repository (or split refresh into `modules/auth/repository.rs`).

---

## Global conventions (apply to every step)

1. **Layering:** Prefer `handler → service → repository`. Handlers parse HTTP; services enforce rules and orchestrate; repositories run SQL only.
2. **Errors:** Map domain errors to HTTP status codes with stable, non-leaky messages (especially for login).
3. **Configuration:** JWT secrets, issuer/audience (if used), access/refresh TTLs, cookie name, `SameSite`, and `Secure` flag must come from **environment** (or config file loaded at startup).
4. **Logging:** Never log passwords, refresh tokens, or access tokens in full.
5. **IDs:** User `id` is UUID; refresh session `id` can be UUID or bigserial—pick one and stay consistent.

---

## Step 1 — Auth foundation: users table and domain model

**Goal:** Persist users with roles and metadata; no login yet.

### Database

1. Add a SQLx migration that creates:
   - Table `users`.
   - Column `id` UUID primary key (default `gen_random_uuid()` if using Postgres).
   - Column `email` `CITEXT` or `TEXT` with **unique constraint** (case-insensitive behavior is ideal; if not using `CITEXT`, enforce lowercase in application layer only and still use unique index on normalized email).
   - Column `password_hash` `TEXT` (or `VARCHAR` large enough for Argon2/bcrypt strings).
   - Column `role` as a Postgres **enum type** with values exactly: `client`, `plumber`, `admin`.
   - Column `is_active` `BOOLEAN` NOT NULL default `true`.
   - Column `created_at` `TIMESTAMPTZ` NOT NULL default `now()`.
   - Column `updated_at` `TIMESTAMPTZ` NOT NULL default `now()` (optional: trigger to bump `updated_at` on update).
2. Run migration locally and confirm schema matches.

### Rust domain

1. Define a `User` struct (or similar) mirroring the row **without** exposing `password_hash` in any public API response type.
2. Define a `Role` enum in Rust matching the DB enum (derive appropriate traits for SQLx if using enums).

### Repository (`modules/users/repository.rs` or equivalent)

Implement async methods:

- `find_by_email(normalized_email) -> Option<User>` (or `Result`).
- `find_by_id(id) -> Option<User>`.
- `create_user(email, password_hash, role, is_active) -> User` (return full row for internal use only).

### Wiring

1. Expose repository through application state (e.g. `Arc<PgPool>` + functions, or a struct).
2. **Do not** add HTTP routes in this step unless you want a health-only stub.

### Verification

- Insert a user via SQL or temporary test harness; `find_by_email` and `find_by_id` return expected data.

---

## Step 2 — Password hashing and validation utilities

**Goal:** Centralize password security and input validation before any register/login.

### Password hashing

1. Choose a **strong** algorithm (Argon2id recommended; bcrypt acceptable if already standardized in org).
2. Implement `hash_password(plain) -> Result<String, DomainError>` and `verify_password(plain, hash) -> Result<bool, DomainError>` (or `Result<(), Error>`).
3. Ensure cost parameters are suitable for production (configurable via env).

### Validation

1. **Email:** validate format with a reasonable rule (length, structure); then **normalize**: trim + lowercase. Apply the same normalization in every code path that looks up or stores email.
2. **Password:** enforce minimum length (document the number, e.g. 8+); optionally complexity rules—keep rules consistent with frontend/mobile copy later.
3. **Structured errors:** define domain error variants for “invalid email”, “invalid password”, “weak password”, without leaking hashes or inputs in messages.

### Placement

- Put helpers in `modules/auth/passwords.rs` (or `modules/auth/validation.rs` split if preferred).

### Verification

- Unit tests: hash → verify succeeds; wrong password fails; invalid inputs return expected error kinds.

---

## Step 3 — `POST /auth/register`

**Goal:** Public registration for **client** and **plumber** only.

### Request body (DTO)

- `email` (string)
- `password` (string)
- `role` (string or enum): **only** `client` or `plumber`

### Service rules

1. Validate email and password (Step 2).
2. Normalize email.
3. If `role` is `admin`, reject with **403** or **400** (choose one policy and document; 403 emphasizes “forbidden”, 400 “bad request”—be consistent).
4. If email already exists, return **409 Conflict** (registration may disclose existence; acceptable for register—**not** for login).
5. Hash password; insert user with `is_active = true`.
6. Return **sanitized** user DTO: `id`, `email`, `role`, `is_active`, `created_at` (and `updated_at` if desired)—**never** `password_hash`.

### Handler

1. JSON extract + map validation errors to 400 with stable shape (e.g. `{ "error": "validation_error", "fields": ... }` or simple message).
2. Call service; map domain errors to status codes.

### Routing

- Mount under `/auth` prefix: `POST /auth/register`.

### Verification

- Register client and plumber succeed.
- Register admin fails.
- Duplicate email returns 409.
- Response body contains no hash.

---

## Step 4 — JWT token service

**Goal:** Issue and verify JWTs for access and refresh **before** login persists refresh sessions.

### Configuration (env)

- Secret(s) for signing (prefer **separate** secrets for access vs refresh, or one secret with distinct `token_type` validation—separate secrets is stronger).
- Access TTL (short, e.g. 15 minutes).
- Refresh TTL (longer, e.g. 7–30 days).
- Optional: issuer, audience.

### Claims (both token types)

Include at minimum:

- `sub` — user id (UUID as string)
- `role` — string matching `Role`
- `token_type` — literal `"access"` or `"refresh"`
- `exp`, `iat` — standard JWT claims
- `jti` — unique string per token (UUID)

### Functions

1. `create_access_token(user_id, role) -> String`
2. `create_refresh_token(user_id, role, jti) -> String`
3. `verify_access_token(token) -> Claims` — reject if `token_type != access` or signature/TTL invalid.
4. `verify_refresh_token(token) -> Claims` — reject if `token_type != refresh`.

### File organization

- `modules/auth/claims.rs` — claim struct(s) and enums.
- `modules/auth/service_token.rs` or inside `service.rs` if small.

### Verification

- Unit tests: mint access, verify; mint refresh, verify; cross-type verification fails; expired token fails (mock clock or very short TTL in test).

---

## Step 5 — Refresh token persistence

**Goal:** Database-backed refresh sessions for rotation, logout, and revocation.

### Migration: table `refresh_tokens` (or `sessions`)

Columns:

- `id` — UUID PK (recommended) or bigserial
- `user_id` — UUID FK to `users(id)` ON DELETE CASCADE (recommended)
- `jti` — unique string (the JWT `jti` for this session)
- `token_hash` — store a **hash** of the refresh JWT (or of a random session secret), never the raw JWT
- `expires_at` — timestamptz
- `revoked_at` — nullable timestamptz
- `created_at` — timestamptz
- Optional: `user_agent`, `ip_address` (text / inet) for future auditing

Indexes:

- Unique on `jti` where not revoked (or unique on `jti` always if `jti` never reused).
- Index on `user_id` for “revoke all”.

### Hashing strategy

1. When issuing refresh JWT, compute `token_hash = HMAC-SHA256(secret, raw_jwt)` or hash a random **session secret** embedded in the JWT instead of hashing the whole JWT—pick one documented approach. **Never** store plaintext refresh token.

### Repository methods

1. `create_refresh_session(user_id, jti, token_hash, expires_at, meta...)`
2. `find_active_by_jti(jti)` — row where `revoked_at IS NULL` AND `expires_at > now()`
3. `revoke_by_jti(jti)` — set `revoked_at = now()`
4. `revoke_all_for_user(user_id)`

### Verification

- Create session; find active; revoke; find returns none; revoke_all clears all for user.

---

## Step 6 — `POST /auth/login`

**Goal:** Issue access JWT in body and refresh JWT in httpOnly cookie; persist refresh session.

### Request

- `email`, `password`

### Service flow

1. Normalize email.
2. Load user by email. If not found, fail with **generic** invalid credentials (same as wrong password).
3. Verify password hash. On failure, same generic error (status **401**).
4. If `!user.is_active`, reject (401 or 403—pick one; document).
5. Generate `jti` for refresh; mint **refresh** JWT with that `jti`.
6. Compute `token_hash` for DB; `expires_at` from refresh TTL; `create_refresh_session`.
7. Mint **access** JWT.
8. Build response JSON: `{ "access_token": "...", "token_type": "Bearer", "expires_in": <seconds> }` (shape can vary but keep stable).
9. Set **Set-Cookie** header for refresh token:
   - `HttpOnly`
   - `Path` (e.g. `/auth` or `/`—if path too narrow, refresh route must match)
   - `Max-Age` or `Expires` aligned with refresh TTL
   - `SameSite` from config (`Lax` or `Strict`; `None` only if cross-site and HTTPS)
   - `Secure=true` in production
   - Cookie name from config (e.g. `refresh_token`)

### Security

- Response must **not** include `password_hash` or raw refresh token in JSON (cookie carries refresh).

### Verification

- Login success returns access token; `Set-Cookie` present; DB row exists for `jti`.
- Wrong password and unknown email yield **identical** error body.

---

## Step 7 — Access token middleware

**Goal:** Reusable Axum layer that authenticates requests via Bearer access token.

### Behavior

1. Read `Authorization` header; require scheme `Bearer`.
2. Parse token; `verify_access_token`.
3. If missing/invalid, return **401** JSON body (stable shape).
4. If token verifies but `token_type` is not access, reject **401**.
5. Insert **auth context** into request extensions: at least `user_id`, `role` (email optional if you want to avoid DB hit).

### Extractor (optional but recommended)

- Implement `FromRequestParts` for `AuthUser` (or similar) that depends on middleware or duplicate verification (middleware is usually enough if it attaches extensions).

### Verification

- Call protected test route with valid access token → 200.
- No header / wrong token / refresh token used as Bearer → 401.

---

## Step 8 — RBAC guards

**Goal:** After authentication, enforce role requirements.

### Helpers

Implement composable checks (functions or tower layers) such as:

- `require_authenticated()` — same as middleware (if not already applied globally).
- `require_role(Role::Admin)` etc.
- Optional: `require_any_role(&[Role::Admin, Role::Plumber])`.

### Behavior

1. Read `role` from auth context (from access token).
2. If role insufficient, return **403** with stable message (`forbidden`), distinct from 401.

### Verification

- User with `client` token cannot access plumber-only test route.
- Admin can access admin route.

---

## Step 9 — `POST /auth/refresh`

**Goal:** Rotate refresh token and issue new access token.

### Input

- Refresh JWT from **cookie** (same name as login).

### Flow

1. If cookie missing, **401** (generic).
2. Verify refresh JWT; ensure `token_type == refresh`; read `sub`, `role`, `jti`.
3. `find_active_by_jti(jti)`. If none, **401** (generic—could be reuse attack or invalid).
4. Compare stored `token_hash` with recomputed hash from presented JWT (constant-time compare). Mismatch → **401**.
5. Check `expires_at` still valid (DB and JWT should align; JWT check already done).
6. **Rotate:** `revoke_by_jti(old_jti)`; generate **new** `jti_new`, new refresh JWT, new `token_hash`, `create_refresh_session`.
7. Return new access token JSON; **Set-Cookie** new refresh (same attributes as login).
8. Old cookie value becomes useless after rotation.

### Security

- Do not leak whether failure was “unknown jti” vs “revoked”—same 401 body.

### Verification

- Refresh returns new access + new cookie; old session revoked; second use of old refresh fails.

---

## Step 10 — `POST /auth/logout`

**Goal:** Revoke current refresh session and clear cookie.

### Flow

1. Read refresh cookie if present.
2. If verifiable and session active, `revoke_by_jti`.
3. If cookie missing or invalid, still return **204** or **200** success (idempotent).
4. **Clear cookie** via `Set-Cookie` with empty value and `Max-Age=0` (or `Expires` in the past), matching `Path`, `SameSite`, and `Secure` as on set.

### Verification

- After logout, refresh with old cookie fails.
- Calling logout twice succeeds.

---

## Step 11 — `POST /auth/logout-all`

**Goal:** Revoke every refresh session for the current user.

### Auth

- Requires **valid access token** (middleware).

### Flow

1. From auth context, take `user_id`.
2. `revoke_all_for_user(user_id)`.
3. Clear refresh cookie in response (user may still hold old cookie string locally—server-side all sessions dead).
4. Return structured success JSON.

### Verification

- Two devices logged in; logout-all revokes both refresh sessions.

---

## Step 12 — `GET /auth/me`

**Goal:** Return current user profile from DB (or token + DB hybrid).

### Auth

- Requires valid access token.

### Flow

1. Parse `user_id` from auth context.
2. `find_by_id`; if missing, **404** (rare if token valid).
3. Return DTO: `id`, `email`, `role`, `is_active`, `created_at` (and `updated_at` if desired). **No** `password_hash`, no refresh data.

### Verification

- With Bearer access, returns correct user; without token, 401.

---

## Final API checklist (before frontend/mobile)

- [ ] Neon branch (or project) chosen; `DATABASE_URL` uses **pooled** host for the running API; migrations tested (direct URL if pooler blocks DDL).
- [ ] Migrations applied for `users` and `refresh_tokens`.
- [ ] Register restricts roles to client/plumber.
- [ ] Login returns generic error for bad credentials.
- [ ] Refresh rotates session and cookie.
- [ ] Logout and logout-all behave as specified.
- [ ] Access middleware rejects refresh tokens used as Bearer.
- [ ] RBAC returns 403 for wrong role.
- [ ] CORS and cookie attributes documented for your web origin (frontend guide).

---

## Suggested route map (reference)

| Method | Path | Auth | Purpose |
|--------|------|------|---------|
| POST | `/auth/register` | No | Register client/plumber |
| POST | `/auth/login` | No | Login; set refresh cookie |
| POST | `/auth/refresh` | Cookie | Rotate refresh; new access |
| POST | `/auth/logout` | Optional cookie | Revoke session; clear cookie |
| POST | `/auth/logout-all` | Bearer access | Revoke all sessions |
| GET | `/auth/me` | Bearer access | Current user profile |

Add a **protected ping** route under a future `modules/jobs` or `api` namespace to test middleware in integration tests.
