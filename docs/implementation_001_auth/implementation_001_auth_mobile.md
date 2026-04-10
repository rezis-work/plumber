# Implementation 001 — Auth mobile (Expo / React Native)

**Purpose:** Step-by-step instructions to integrate the mobile app (`apps/mobile`) with the auth API from [implementation_001_auth_api.md](./implementation_001_auth_api.md). Instructions only—no embedded code.

**Important platform note:** Browsers automatically attach **httpOnly** cookies to same-origin credentialed requests. **React Native does not behave like a browser cookie jar by default.** You must choose how the mobile app will participate in refresh-token flows:

1. **Preferred alignment with API (if feasible):** Use an HTTP client that **persists cookies** for your API domain (e.g. OkHttp cookie jar on Android, shared cookie store if using a library that supports it) and send `credentials`/cookie headers consistently—still verify Expo/React Native networking stack supports this for your API host.
2. **Common pragmatic approach:** Keep API as designed for web (httpOnly refresh cookie), and add a **mobile-specific** refresh mechanism (e.g. refresh token in JSON body or `Authorization` header) **only for native clients**—this is a **product/API decision** and must be implemented deliberately with equivalent security (rotation, revocation, storage in Keychain/Keystore via SecureStore).

**This guide assumes you will either (A) make cookie jar work end-to-end, or (B) extend the API with a parallel, equally secure native refresh path.** Document the choice before implementation.

Coordinate with the API guide so **logout**, **rotation**, and **revocation** work for the chosen approach.

---

## Phase M0 — Decision record (do this first)

### Step M0.1 — Choose refresh transport for native

1. **Option Cookie-jar:** API unchanged; mobile HTTP client stores `Set-Cookie` and sends cookies on `/auth/refresh` and `/auth/logout`.
2. **Option Native refresh token:** API adds endpoints or negotiates refresh token in response body for mobile registration/login only; store refresh token in **expo-secure-store** (or react-native-keychain); rotation updates stored refresh.

### Step M0.2 — Threat model parity

Regardless of option:

- Refresh token must be **rotatable** and **revocable** server-side.
- Access token remains short-lived in memory (secure volatile storage).
- Never log passwords or tokens.

---

## Phase M1 — Configuration

### Step M1.1 — API base URL

1. Use `app.config` / `expo-constants` `extra` or `.env` (via `expo-env` or your chosen pattern) for `API_URL`.
2. Separate **dev** (LAN IP or tunnel) vs **prod** base URL.

### Step M1.2 — TLS

1. Production must use HTTPS.
2. Android cleartext and iOS ATS: document exceptions only for local dev if needed.

---

## Phase M2 — Secure storage and session state

### Step M2.1 — Access token

1. Store access token in **memory** while app is foregrounded (React state/context).
2. On app background/kill, restore session by:
   - **Cookie path:** call `/auth/refresh` with cookie jar (no body), or
   - **Native path:** read refresh from SecureStore, call dedicated refresh contract.

### Step M2.2 — User profile

1. After login or refresh, call `GET /auth/me` with Bearer access to populate profile in global state (React Context, Zustand, Jotai, etc.).
2. Treat server response as source of truth for `role`, `is_active`, and **`is_email_verified`** (gate features or banners until verified).

### Step M2.3 — Logout local cleanup

1. Clear memory state.
2. **Cookie path:** logout endpoint clears cookie via client jar; clear any local cookie store if library requires.
3. **Native path:** delete refresh secret from SecureStore on logout.

---

## Phase M3 — HTTP client

### Step M3.1 — Single API wrapper

1. Create a module that wraps `fetch` (or axios) with:
   - Base URL.
   - JSON helpers.
   - Hook to attach `Authorization: Bearer` when access token present.
   - **401 retry:** one refresh attempt then retry original request (mirror web guide).
2. Disable excessive logging in release builds.

### Step M3.2 — Credentials / cookies

1. If using cookie jar, ensure redirects and Android/iOS cookie persistence are tested against your real API domain.
2. If using native refresh body/header, never store refresh in plain AsyncStorage without encryption—use SecureStore.

---

## Phase M4 — Screens and flows

Use **two registration flows** (separate screens/stacks), matching the API and web guide.

### Step M4.1a — Client signup

1. **Fields:** email, password (and optional confirm-password in UI only).
2. **API:** `POST /auth/register/client`.
3. **Success:** Navigate to **email verification** messaging; optionally show verification token **only in __DEV__** builds—never in production store builds.
4. Validation aligned with API (password length, email shape).
5. Map **409** / **400** like web.

### Step M4.1b — Become a plumber

1. **Fields:** email, password, **full name**, **phone**, **years of experience**.
2. **API:** `POST /auth/register/plumber`.
3. **Success:** Distinct confirmation screen (aligned with product: immediate access vs pending approval later).
4. Validate phone and non-negative years per API contract.

### Step M4.2 — Login

1. `POST /auth/login`; capture access token from JSON.
2. If cookie-based refresh: confirm subsequent `/auth/refresh` works without extra setup.
3. If native refresh: capture refresh from agreed field and save to SecureStore (if API supports).
4. Fetch `/auth/me`; navigate to main app stack.

### Step M4.3 — Session restore on cold start

1. On app launch, show splash/loading while attempting refresh (or validating stored refresh).
2. If success, fetch `/auth/me` and enter app.
3. If failure, show login.

### Step M4.4 — Logout

1. Call `/auth/logout` (with cookie or body per contract).
2. Clear SecureStore + memory; reset navigation to auth stack.

### Step M4.5 — Logout all

1. From settings, call `/auth/logout-all` with Bearer access.
2. Clear all local tokens; reset navigation.

### Step M4.6 — Token refresh interval (optional)

1. Optionally refresh access before expiry using `expires_in` from login/refresh response and a timer—still handle 401 retry defensively.

---

## Phase M5 — Navigation guards

### Step M5.1 — Auth stack vs app stack

1. Use Expo Router or React Navigation pattern: unauthenticated stack (login/register) vs authenticated stack (tabs/drawer).
2. Gate switch on “has valid access or successful silent refresh”.

### Step M5.2 — Role-based screens

1. Restrict plumber-only and admin-only screens by `role` from `/auth/me`.
2. Deep links: if user opens a plumber URL without role, show forbidden or redirect.

---

## Phase M6 — Platform-specific considerations

### Step M6.1 — iOS

1. Test backgrounding: does cookie jar survive? If not, rely on refresh-on-launch path.
2. Keychain access: SecureStore behavior with backups—understand Expo docs.

### Step M6.2 — Android

1. Test network security config for dev.
2. Cookie persistence across process death—verify.

### Step M6.3 — Expo Go vs dev client

1. Some secure features differ; document if testing only in Expo Go.

---

## Phase M7 — Testing checklist

- [ ] **Client** signup → verification UX (dev token handling if applicable).
- [ ] **Plumber** signup with full profile → success path.
- [ ] Login → call protected endpoint → success.
- [ ] Kill app → reopen → still logged in (refresh path works).
- [ ] Logout → reopen → login required.
- [ ] Logout-all from another device/session invalidates mobile session on next refresh attempt.
- [ ] Wrong login shows generic error only.
- [ ] No passwords or tokens in Metro logs in production mode.

---

## Mobile deliverables checklist

- [ ] Documented choice: cookie jar vs native refresh contract.
- [ ] **Two registration screens** (client vs become plumber) wired to the correct endpoints.
- [ ] API client with Bearer + refresh retry.
- [ ] Secure storage for any long-lived secret (native path).
- [ ] Navigation split authenticated vs not.
- [ ] Role-based UI gating (client / plumber / admin).

---

## Handoff to API team (if native refresh is chosen)

Provide a short spec:

- Which login/refresh responses include the native refresh token.
- How rotation updates the stored refresh on the client.
- How logout and logout-all map to revocation for native sessions.

This keeps backend, web, and mobile guides consistent.
