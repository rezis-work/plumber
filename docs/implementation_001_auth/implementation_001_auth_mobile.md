# Implementation 001 — Auth mobile (Expo / React Native)

**Purpose:** Step-by-step instructions to integrate the mobile app (`apps/mobile`) with the auth API from [implementation_001_auth_api.md](./implementation_001_auth_api.md). Instructions only—no embedded code.

**Important platform note:** Browsers automatically attach **httpOnly** cookies to same-origin credentialed requests. **React Native does not behave like a browser cookie jar by default.** You must choose how the mobile app will participate in refresh-token flows:

1. **Preferred alignment with API (if feasible):** Use an HTTP client that **persists cookies** for your API domain (e.g. OkHttp cookie jar on Android, shared cookie store if using a library that supports it) and send `credentials`/cookie headers consistently—still verify Expo/React Native networking stack supports this for your API host.
2. **Common pragmatic approach:** Keep API as designed for web (httpOnly refresh cookie), and add a **mobile-specific** refresh mechanism (e.g. refresh token in JSON body or `Authorization` header) **only for native clients**—this is a **product/API decision** and must be implemented deliberately with equivalent security (rotation, revocation, storage in Keychain/Keystore via SecureStore).

**This guide assumes you will either (A) make cookie jar work end-to-end, or (B) extend the API with a parallel, equally secure native refresh path.** Document the choice before implementation.

Coordinate with the API guide so **logout**, **rotation**, and **revocation** work for the chosen approach.

**Order:** Complete **Phase MS (design system)** in parallel with or immediately after web **Phase 0** so **mobile uses the same colors and tokens** as SvelteKit—one palette for both clients. Pull **mobile Stitch** exports (**MS.3**) via **Stitch MCP** + `curl -L` the same way as the web guide. Implement **server state** with **TanStack Query** (**Phase MQ**): `useMutation` for auth-changing requests, `useQuery` for **`GET /auth/me`** and similar reads.

---

## Phase MS — Design system (shared with web)

Native UI must match the **Stitch**-driven design system used on the web. Do **not** invent a separate mobile-only color scheme unless product explicitly asks for it.

### Step MS.1 — Same sources as frontend

1. Follow [implementation_001_auth_frontend.md — Phase 0](./implementation_001_auth_frontend.md#phase-0--design-system-google-stitch) for Stitch MCP workflow and downloads.
2. **Project ID:** `9702559548791545108`
3. **Design System screen:** `asset-stub-assets-547841dc1b4545db8471e31333de0ce8-1775829756314`
4. Use `curl -L` (or equivalent) for any hosted asset URLs Stitch returns; keep a **single token source** (e.g. shared `tokens.json` or copy the web app’s CSS variables into a RN theme module).

### Step MS.2 — Apply in Expo

1. Map semantic colors into your RN theme (constants, NativeWind, Tamagui, etc.) using the **same names and hex values** as the web app.
2. When Stitch exports update, update **web first** (Phase 0.2), then sync mobile in one pass.

**MS.1–MS.2 in this repo:** Web canonical file is `apps/web/src/lib/design/tokens.json`; mobile keeps a copy at `apps/mobile/src/design/tokens.json`. After any web token update, run **`pnpm sync-design-tokens`** from the repository root, or **`pnpm sync-tokens`** from `apps/mobile`, to copy web → mobile in one step. Semantic colors and spacing for React Native live under `apps/mobile/src/theme/` (`colors.ts`, `spacing.ts`). Stitch IDs and `curl -L` workflow for mobile are summarized in `apps/mobile/docs/STITCH_INSTRUCTIONS.md`; see also `apps/mobile/src/design/README.md`.

### Step MS.3 — Mobile marketing & auth screens (Stitch MCP + assets)

Build **native** layouts from Stitch’s **mobile** exports (not the desktop-only web screens). Use **Stitch MCP** in Cursor the same way as [Phase 0 / 0B in the frontend guide](./implementation_001_auth_frontend.md#phase-0--design-system-google-stitch): list or pull screens by **project** and **screen ID**, then download any hosted asset or export URLs.

| | |
|--|--|
| **Project ID** | `9702559548791545108` |

**Screens (mobile):**

| # | Screen (Stitch) | Screen ID | Suggested local folder (example) |
|---|-----------------|-----------|----------------------------------|
| 1 | **Fixavon Landing Page - Mobile** | `bd10bc970a374954b5a531dafcb83847` | `apps/mobile/assets/stitch/landing-mobile/` |
| 2 | **Client Registration - Mobile** | `1c635e4ac20c483588a505eb52dcb53c` | `apps/mobile/assets/stitch/register-client-mobile/` |
| 3 | **Plumber Registration - Mobile** | `99473da95838498aa626cd37f11153b7` | `apps/mobile/assets/stitch/register-plumber-mobile/` |
| 4 | **Verify Email - Mobile** | `5c38516278094c3ca521320fbd3edf1a` | `apps/mobile/assets/stitch/verify-email-mobile/` |
| 5 | **Login - Mobile** | `669b5b6b8ed24c179ed828e95c4fc88a` | `apps/mobile/assets/stitch/login-mobile/` |

**Workflow**

1. For each row, use **Stitch MCP** to fetch images/code for project **`9702559548791545108`** and the given **screen ID**.
2. Download hosted URLs with e.g. **`curl -L -o <local-filename> "<url-from-stitch>"`**.
3. Commit raster assets (PNG/SVG) and any HTML/CSS reference exports under the suggested folder (adjust path if your app lives elsewhere).
4. Implement **React Native / Expo** screens using **Phase MS** tokens for colors and type; map Stitch layout to RN components (ScrollView, SafeAreaView, TextInput, Pressable, etc.)—same policy as web: **no one-off hex** outside the shared theme.

**Desktop parity (optional reference):** The [frontend guide](./implementation_001_auth_frontend.md) lists **desktop** Stitch IDs for the same flows (landing, C1a/C1b/C1c, C2). Use them only when you need layout hints; **ship mobile against the mobile screen IDs above.**

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

**TanStack Query:** On logout / logout-all success, **remove** or **reset** cached queries (e.g. `queryClient.removeQueries({ queryKey: ['auth'] })`) so **`/auth/me`** and other authenticated data cannot flash stale UI—see **Phase MQ**.

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

## Phase MQ — TanStack Query (React Query)

Use [**TanStack Query**](https://tanstack.com/query/latest) (`@tanstack/react-query`) for **server state**: mutations for writes, queries for reads, consistent loading/error states, and deduplication. Keep **volatile auth secrets** (access token) in memory / secure storage per **M2**; use the query layer for **API synchronization**, not as the long-term store for refresh tokens.

### Step MQ.1 — Provider and client

1. Add **`@tanstack/react-query`** to the Expo app; wrap the root layout with **`QueryClientProvider`** and a stable **`QueryClient`** instance (create outside render or with a small factory).
2. Optional: **`@tanstack/react-query-persist-client`** only if you clearly exclude auth queries from persistence—do **not** persist access tokens or **`/auth/me`** to AsyncStorage without an explicit threat review.

### Step MQ.2 — Mutations (auth flows)

Implement **`useMutation`** (or thin wrappers) for each **state-changing** auth call, calling your **M3** API wrapper:

| Mutation | Typical endpoint | Notes |
|----------|------------------|--------|
| Register client | `POST /auth/register/client` | `onSuccess` → navigate to verify-email; optional **`__DEV__`** surfacing of verification token. |
| Register plumber | `POST /auth/register/plumber` | `onSuccess` → confirmation screen. |
| Verify email | `POST /auth/verify-email` | `onSuccess` → login or main per product. |
| Login | `POST /auth/login` | `onSuccess` → persist session per **M2**, **`queryClient.invalidateQueries`** for **`['auth','me']`**, navigate to app stack. |
| Logout | `POST /auth/logout` | `onSuccess` → clear **M2** + **MQ** cache, reset navigation. |
| Logout all | `POST /auth/logout-all` | Same as logout locally. |
| Refresh (if invoked explicitly) | `POST /auth/refresh` | Usually triggered from **M3** 401 retry; optional dedicated mutation if you model it in UI. |

Use **`isPending`** / **`isError`** from mutations for button disabled states and inline errors (generic messages for login, same policy as web).

### Step MQ.3 — Queries (reads)

1. **`GET /auth/me`:** **`useQuery`** with queryKey e.g. **`['auth','me']`**, **`enabled: !!accessToken`** (or equivalent “session ready” flag). After login or silent refresh, **invalidate** or **setQueryData** so profile and **`role`** stay in sync.
2. **`staleTime`:** moderate values (e.g. tens of seconds) for **`/me`** if you want fewer refetches; still rely on **401 + refresh retry** in the HTTP layer for correctness.
3. Do not put the **Bearer string** in query keys or logs.

### Step MQ.4 — Integration with navigation (M5)

1. Prefer **one** place (e.g. root layout) that subscribes to “has session” derived from **M2** + **`useQuery` success** for **`me`** when needed.
2. On **mutation** success for logout, clear queries before navigating to the auth stack to avoid flicker.

---

## Phase M4 — Screens and flows

Use **two registration flows** (separate screens/stacks), matching the API and web guide. **UI source:** **Phase MS.3** (mobile Stitch). Wire actions with **`useMutation`** from **Phase MQ** where applicable.

### Step M4.0 — Marketing landing (mobile)

**Visual reference:** **Fixavon Landing Page - Mobile** — **MS.3** (screen ID `bd10bc970a374954b5a531dafcb83847`).

1. Public screen: CTAs to **client register**, **plumber register**, **login** (Expo Router / React Navigation).
2. No Bearer token; same token theme as **MS.2**.

### Step M4.1a — Client signup

**Visual reference:** **Client Registration - Mobile** — **MS.3** (screen ID `1c635e4ac20c483588a505eb52dcb53c`). *Desktop reference:* [frontend C1a](./implementation_001_auth_frontend.md#step-c1a--client-signup) (`b4e85f70dda04fd18cbc2ded66367040`).

1. **Fields:** email, password (and optional confirm-password in UI only).
2. **API:** `POST /auth/register/client` via **`useMutation`** (**MQ.2**).
3. **Success:** Navigate to **email verification** (**M4.1c**); optionally show verification token **only in __DEV__** builds—never in production store builds.
4. Validation aligned with API (password length, email shape).
5. Map **409** / **400** like web.

### Step M4.1c — Verify email

**Visual reference:** **Verify Email - Mobile** — **MS.3** (screen ID `5c38516278094c3ca521320fbd3edf1a`). *Desktop reference:* [frontend C1c](./implementation_001_auth_frontend.md#step-c1c--verify-email) (`2895e33e817143538c664b94e7538991`).

1. **Entry:** After client signup or from a deep link when email sending exists.
2. **API:** **`POST /auth/verify-email`** via **`useMutation`** (**MQ.2**).
3. **Success:** Navigate to login or main stack per product; **failure:** map API errors when available.

### Step M4.1b — Become a plumber

**Visual reference:** **Plumber Registration - Mobile** — **MS.3** (screen ID `99473da95838498aa626cd37f11153b7`). *Desktop reference:* [frontend C1b](./implementation_001_auth_frontend.md#step-c1b--become-a-plumber) (`2c2497ba2dee4162a6abd45a76f45ff0`).

1. **Fields:** email, password, **full name**, **phone**, **years of experience**.
2. **API:** `POST /auth/register/plumber` via **`useMutation`** (**MQ.2**).
3. **Success:** Distinct confirmation screen (aligned with product: immediate access vs pending approval later).
4. Validate phone and non-negative years per API contract.

### Step M4.2 — Login

**Visual reference:** **Login - Mobile** — **MS.3** (screen ID `669b5b6b8ed24c179ed828e95c4fc88a`). *Desktop reference:* [frontend C2](./implementation_001_auth_frontend.md#step-c2--login-page) (`3d5928e9ca844ea9a955ca21a06f0f52`).

1. `POST /auth/login` via **`useMutation`** (**MQ.2**); capture access token from JSON.
2. If cookie-based refresh: confirm subsequent `/auth/refresh` works without extra setup.
3. If native refresh: capture refresh from agreed field and save to SecureStore (if API supports).
4. Invalidate or fetch **`/auth/me`** via **`useQuery`** (**MQ.3**); navigate to main app stack.

### Step M4.3 — Session restore on cold start

1. On app launch, show splash/loading while attempting refresh (or validating stored refresh).
2. If success, **`invalidateQueries`** / refetch **`['auth','me']`** (**MQ.3**) and enter app.
3. If failure, show login.

### Step M4.4 — Logout

1. Call `/auth/logout` via **`useMutation`** (**MQ.2**) (with cookie or body per contract).
2. Clear SecureStore + memory; **clear TanStack Query auth cache** (**MQ.2**); reset navigation to auth stack.

### Step M4.5 — Logout all

1. From settings, call `/auth/logout-all` with Bearer access (**`useMutation`**).
2. Clear all local tokens; **clear query cache**; reset navigation.

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

- [ ] **Client** signup → **verify-email** screen (Stitch **Verify Email - Mobile**, **MS.3**; dev token handling if applicable).
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
- [ ] **Stitch MCP** mobile exports committed (**MS.3**): landing, client register, plumber register, verify email, login (`curl -L` for hosted URLs).
- [ ] **TanStack Query** (**MQ**): `QueryClientProvider`, **`useMutation`** for auth writes, **`useQuery`** for **`/auth/me`**, cache cleared on logout.
- [ ] **Client** signup, **verify-email**, and **plumber** signup screens (see **M4.1a** / **M4.1c** / **M4.1b**) wired to the correct endpoints.
- [ ] API client with Bearer + refresh retry (**M3**).
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
