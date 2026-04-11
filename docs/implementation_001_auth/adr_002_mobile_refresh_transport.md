# ADR 002 — Mobile refresh transport (Expo / React Native)

## Status

Accepted — **Option B (native refresh token in app secure storage)**. **Option A (browser-style cookie jar)** is not pursued for the initial mobile implementation.

## Context

- The API issues a **short-lived access JWT** in JSON and a **long-lived refresh JWT** in an **httpOnly** `Set-Cookie` on **`POST /auth/login`** and **`POST /auth/refresh`**, and reads refresh from the **`Cookie`** header on refresh and logout ([`apps/api/src/modules/auth/handler.rs`](../../apps/api/src/modules/auth/handler.rs), [`cookie_config.rs`](../../apps/api/src/modules/auth/cookie_config.rs)). Web clients rely on the browser cookie jar ([ADR 001](./adr_001_cors_and_cookies.md)).
- **React Native / Expo** does not provide an httpOnly cookie jar equivalent to browsers. Default **`fetch`** does not reliably **persist** and **replay** cookies across app restarts and is a poor match for `Path=/auth` and native TLS stacks without extra native configuration.
- **Option A** would require a dependency or native HTTP stack that persists cookies for the API host and is validated on **both** Android and iOS (including Expo Go constraints if applicable). That path is viable for some teams but carries higher integration and test surface area.
- **Option B** adds a **deliberate, API-visible** way for native clients to receive and send the refresh credential (same JWT semantics and **same** DB session rotation/revocation as today), stored in **expo-secure-store** (Keychain / Keystore).

## Decision

1. **Mobile uses Option B:** after the API is extended (see **Contract intent** below), the Expo app stores the **refresh token** in **SecureStore**, keeps the **access token** in **memory** (React state / context), and sends **`Authorization: Bearer`** for protected requests.
2. **Option A is rejected for v1** to avoid blocking mobile on native cookie-jar spikes and dual-platform cookie debugging. A future revision could revisit A if product requires **zero** API surface change.
3. **Cookie-jar spike:** A time-boxed device spike for Option A was **not run** because Option B was selected first for predictability with Expo and alignment with the mobile guide’s “native refresh” path. If product later mandates no API change, run the spike in [implementation_001_auth_mobile.md — Phase M0](./implementation_001_auth_mobile.md#phase-m0--decision-record-do-this-first) before implementation.

## Contract intent (API work — out of scope for this ADR)

The API **today** does not expose refresh in JSON; the following is the **intended direction** for implementers (exact shape to be finalized in the API task and OpenAPI/docs):

| Concern | Direction |
|---------|-----------|
| **Client identification** | Use a stable request header (e.g. `X-Auth-Client: native` or `X-Plumber-Client: expo`) on mobile-only flows so the server can include **`refresh_token`** in JSON **without** changing browser behavior. Alternatively, separate **`/auth/native/*`** routes—pick one style and document it. |
| **Login** | `POST /auth/login`: same request body; response JSON includes existing `access_token`, `token_type`, `expires_in`, plus **`refresh_token`** for native clients. Server still may set `Set-Cookie` for debugging; **mobile must ignore** persistence via cookies and use JSON + SecureStore. |
| **Refresh** | `POST /auth/refresh`: accept refresh from **JSON body** (e.g. `{ "refresh_token": "..." }`) **or** from `Cookie` when body absent, so web and mobile share one route **or** native uses a dedicated route that reuses the same `service::refresh` logic. Response: same access fields + **new** `refresh_token` in JSON for native; rotation and `Set-Cookie` behavior stay consistent with [implementation_001_auth_api.md](./implementation_001_auth_api.md). |
| **Logout** | `POST /auth/logout`: accept refresh from body when cookie absent; revoke session; mobile clears SecureStore and in-memory access token. |
| **Logout-all** | `POST /auth/logout-all`: unchanged for Bearer access; mobile clears SecureStore after success. |

**Registration:** Current client/plumber **register** endpoints do not need to issue refresh for mobile until product requires “logged-in immediately after signup”; mobile can **login** after verify-email per [implementation_001_auth_mobile.md](./implementation_001_auth_mobile.md) Phase M4.

## Mobile behavior summary (after API supports B)

| Flow | Behavior |
|------|----------|
| **401 → refresh → retry** | If a request returns 401, call refresh once using **refresh_token from SecureStore**, store new refresh (and access), retry the original request; on refresh failure, clear session and show login. |
| **Logout** | `POST /auth/logout` with refresh from SecureStore (per final API contract); clear SecureStore and memory; reset navigation (see mobile guide Phase M4/M5/MQ). |
| **Logout-all** | `POST /auth/logout-all` with Bearer access; clear local refresh + access; reset navigation. |
| **Cold start** | If SecureStore holds refresh, call refresh to obtain access; then fetch `/auth/me` as needed. |

## M0.2 — Threat model parity (checklist)

| Requirement | Satisfied |
|-------------|-----------|
| **Refresh rotatable** | Yes — same server rotation as cookie flow; each refresh mints a new refresh JWT and revokes the prior session in DB ([implementation_001_auth_api.md](./implementation_001_auth_api.md) Step 9). Mobile **must** replace SecureStore value on every successful refresh. |
| **Refresh revocable** | Yes — logout / logout-all revoke server-side; mobile **must** delete local refresh on logout and on auth failure after refresh. |
| **Access token short-lived, in memory** | Yes — store access token only in volatile app state; do **not** persist access token in AsyncStorage. |
| **No passwords or tokens in logs** | Yes — match API and web policy; no logging of `refresh_token`, `access_token`, or passwords ([implementation_001_auth_api.md](./implementation_001_auth_api.md) logging rules). |

## Consequences

- **API and mobile must be implemented together** for auth session establishment; mobile M3/M4 cannot rely on cookies alone until the native contract exists.
- **Web remains unchanged** in spirit: browsers continue to use httpOnly cookies; native clients use JSON + SecureStore.
- **Security review:** Treat `refresh_token` in JSON as **equivalent sensitivity** to the cookie value (TLS only in production; no disk logging; SecureStore only).

## References

- [implementation_001_auth_mobile.md](./implementation_001_auth_mobile.md) — Phase M0, M2, M3, MQ
- [implementation_001_auth_api.md](./implementation_001_auth_api.md) — Steps 6–11
- [ADR 001 — CORS and cookies (web)](./adr_001_cors_and_cookies.md)
