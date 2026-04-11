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

## Step 3 — Public registration (split: client vs plumber)

**Goal:** Two **different** public flows—no shared “role picker” body. **Admin** is never creatable via either route.

**Product rules**

- **Clients:** minimal signup (**email + password** only). Account is created with **`is_email_verified = false`**. API returns a **one-time email verification token** (plus expiry) in the JSON response. **Outbound email is not implemented yet**; the token exists so the client app can show “check your email” later and so you can add `POST /auth/verify-email` + mailer in a follow-up step.
- **Plumbers:** “**Become a plumber**” application—**email, password, full name, phone number, years of experience**. Same password/email rules as Step 2; additional field validation (length, phone format, non-negative years). Distinct route and DTOs from client registration.

---

### Step 3a — Prerequisite migration (schema)

Add a SQLx migration **before** implementing the handlers:

1. On **`users`** (all roles):
   - `is_email_verified` **BOOLEAN** NOT NULL DEFAULT **false**.
   - `email_verification_token_hash` **TEXT** NULL (store **only a hash** of the verification token, never the raw token at rest).
   - `email_verification_expires_at` **TIMESTAMPTZ** NULL.
2. Table **`plumber_profiles`** (one row per plumber user):
   - `user_id` **UUID** PRIMARY KEY, **FK** to `users(id)` ON DELETE CASCADE.
   - `full_name` **TEXT** NOT NULL.
   - `phone` **TEXT** NOT NULL (normalize/strip in app; optional `CHECK` or length bound).
   - `years_of_experience` **INTEGER** NOT NULL with **CHECK (years_of_experience >= 0)** (add a sane upper bound in app if desired).

**Rust:** extend `User` / repository as needed; add `PlumberProfile` model + `PlumberProfileRepository` (or methods on `UserRepository`) for insert-after-user.

---

### Step 3b — `POST /auth/register/client`

**Request body (DTO)**

- `email` (string)
- `password` (string)

**Service rules**

1. Validate email and password (Step 2); normalize email.
2. If email already exists → **409 Conflict**.
3. Hash password (Step 2); insert `users` with `role = client`, `is_active = true`, **`is_email_verified = false`**.
4. Generate a **cryptographically random** verification token (e.g. 32+ bytes); **hash** it (e.g. SHA-256 or HMAC with a server secret—document choice); store hash + **`email_verification_expires_at`** (e.g. now + 24–48h, configurable via env).
5. Response JSON:
   - **Sanitized user:** `id`, `email`, `role`, `is_active`, **`is_email_verified`**, `created_at`, `updated_at` (no `password_hash`).
   - **`email_verification_token`**: raw token string **once** (client must not log it; UI may show dev-only copy in non-prod).
   - **`email_verification_expires_at`** (ISO 8601) so the client can show countdown or disable resend until later.

**Security:** never log the raw verification token; never return the stored hash; treat token like a secret in transit (HTTPS only in production).

**Handler:** JSON 400 for validation failures (stable error shape). No admin check (route is client-only).

---

### Step 3c — `POST /auth/register/plumber`

**Request body (DTO)**

- `email` (string)
- `password` (string)
- `full_name` (string)
- `phone` (string)
- `years_of_experience` (integer, >= 0)

**Service rules**

1. Validate email + password (Step 2); validate **full_name** (non-empty, max length), **phone** (minimal format/length—align with frontend/mobile), **years_of_experience** (>= 0).
2. Normalize email; normalize phone (e.g. strip spaces) consistently before store.
3. If email already exists → **409 Conflict**.
4. Hash password; insert `users` with `role = plumber`, `is_active = true`.
   - **Email verification policy for plumbers (choose and document one):**
     - **Option A:** also `is_email_verified = false` and issue the same verification token pattern as clients in this response; or
     - **Option B:** set `is_email_verified = true` for plumbers on create and skip client-style token until you need parity.
   - Pick one and keep **frontend/mobile** copy aligned.
5. Insert **`plumber_profiles`** row for the new `user_id`.
6. Response: sanitized user DTO + **plumber profile** fields (`full_name`, `phone`, `years_of_experience`) **without** password or hashes. If you return a verification token for plumbers (Option A), mirror the client response fields.

**Handler:** 400 validation; 409 duplicate email.

---

### Routing

- `POST /auth/register/client`
- `POST /auth/register/plumber`

Do **not** expose a single `POST /auth/register` with a `role` field for end users—avoids “admin” injection and keeps contracts explicit.

---

### Follow-up (document only; not part of Step 3 minimum)

- **`POST /auth/verify-email`** with body `{ "token": "..." }`: look up by hash, check expiry, set `is_email_verified = true`, clear verification columns.
- **Login policy:** decide whether **unverified clients** may log in (e.g. allow login but gate features via `is_email_verified` from `/auth/me`) or block login until verified—document in Step 6 when implementing login.

### Verification (Step 3)

- [ ] Client register: 201/200 with user + **`email_verification_token`** + expiry; DB holds **hash** only; `is_email_verified` false.
- [ ] Plumber register: success with profile fields persisted; duplicate email → 409.
- [ ] Response bodies never include `password_hash` or verification **hash**.
- [ ] Invalid payloads → 400; malformed JSON handled consistently (Axum may use 422—document if you keep default).

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
5. **Unverified email (optional policy):** If `!user.is_email_verified`, either allow login and let `/auth/me` drive UI gating, or reject with the **same generic 401** as bad credentials, or a distinct **403** with a stable code like `email_not_verified`—**pick one** and align with frontend/mobile; document here when implementing Step 6.
6. Generate `jti` for refresh; mint **refresh** JWT with that `jti`.
7. Compute `token_hash` for DB; `expires_at` from refresh TTL; `create_refresh_session`.
8. Mint **access** JWT.
9. Build response JSON: `{ "access_token": "...", "token_type": "Bearer", "expires_in": <seconds> }` (shape can vary but keep stable).
10. Set **Set-Cookie** header for refresh token:
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
3. Return DTO: `id`, `email`, `role`, `is_active`, **`is_email_verified`**, `created_at` (and `updated_at` if desired). If `role` is **plumber**, include **profile** (`full_name`, `phone`, `years_of_experience`) from `plumber_profiles` (or `null`/omit for non-plumbers). **No** `password_hash`, no verification secrets, no refresh data.

### Verification

- With Bearer access, returns correct user; without token, 401.

---

## Final API checklist (before frontend/mobile)

- [x] Neon branch (or project) chosen; `DATABASE_URL` uses **pooled** host for the running API; migrations tested (direct URL if pooler blocks DDL). *(Deploy: you set `DATABASE_URL`; API connects and runs migrations on startup.)*
- [x] Migrations applied for `users`, **`plumber_profiles`**, email verification columns, and `refresh_tokens` (see [`apps/api/migrations/`](../../apps/api/migrations/); `sqlx::migrate!` in `main.rs`).
- [x] **`POST /auth/register/client`** and **`POST /auth/register/plumber`** implemented per Step 3; no public admin creation.
- [x] Login returns generic error for bad credentials (`login_wrong_password_matches_unknown_email_body` + same `LoginError::InvalidCredentials` body).
- [x] Refresh rotates session and cookie (`refresh_rotates_and_old_refresh_rejected`).
- [x] Logout and logout-all behave as specified (Step 10/11 + tests).
- [x] Access middleware rejects refresh tokens used as Bearer (`verify_access_token` + `token_type`; `auth_me_401_refresh_jwt_as_bearer`).
- [x] RBAC returns 403 for wrong role (router tests under `rbac_*_403_*`).
- [x] CORS and cookie attributes documented for your web origin — [implementation_001_auth_frontend.md](implementation_001_auth_frontend.md) (Step A2 CORS/credentials; cookie path/attributes in API steps above). *(API binary does not add a CORS layer yet; coordinate proxy or add `tower-http` CORS when wiring the web app.)*

---

## Suggested route map (reference)

| Method | Path | Auth | Purpose |
|--------|------|------|---------|
| POST | `/auth/register/client` | No | Register client (email + password); returns email verification token |
| POST | `/auth/register/plumber` | No | Become plumber (email, password, full name, phone, years experience) |
| POST | `/auth/verify-email` | No | (Follow-up) Consume email verification token |
| POST | `/auth/login` | No | Login; set refresh cookie |
| POST | `/auth/refresh` | Cookie | Rotate refresh; new access |
| POST | `/auth/logout` | Optional cookie | Revoke session; clear cookie |
| POST | `/auth/logout-all` | Bearer access | Revoke all sessions |
| GET | `/auth/me` | Bearer access | Current user profile |

Add a **protected ping** route under a future `modules/jobs` or `api` namespace to test middleware in integration tests.
