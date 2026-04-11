# Access token storage (Phase B)

## Policy

- The **access JWT** is kept **in memory only** (see [`session.svelte.ts`](./session.svelte.ts): `session.accessToken`). It is **not** written to `localStorage` or `sessionStorage`.
- The **refresh JWT** stays in an **httpOnly** cookie set by the API (`Path=/auth`). JavaScript cannot read it.

## Silent refresh

On app load, the root layout calls **`hydrateFromRefresh()`** only when **`session.accessToken === null`** (see [`+layout.svelte`](../../routes/+layout.svelte)). That function:

1. `POST /auth/refresh` with **`credentials: 'include'`** (sends the refresh cookie)—**once** per hydration; concurrent callers share the same in-flight promise.
2. On success, stores the new `access_token` in memory and loads **`GET /auth/me`** with `Authorization: Bearer …`.
3. On **401** (or any failure), clears in-memory session; unauthenticated users have no cookie or an invalid session.

Full page reloads drop the access token from RAM; the next load repeats refresh + `/me` so the user stays signed in while the refresh cookie is valid.

## Same-origin vs cross-origin

Cookie + `fetch` behavior depends on deployment. See [ADR 001](../../../../docs/implementation_001_auth/adr_001_cors_and_cookies.md): dev default is the Vite **`/auth` proxy**; cross-origin needs `PUBLIC_API_URL`, `CORS_ALLOWED_ORIGINS`, and appropriate **`SameSite` / `Secure`** on the refresh cookie.

## SSR

Initial hydration runs in **`onMount`** (browser only). Server-rendered HTML does not include user-specific state until the client runs refresh + `/me`. A future improvement can use `load` + `fetch` when cookies are visible during SSR.

## Derived flags in UI

Svelte does not allow exporting `$derived` from `session.svelte.ts`. Import `session` and use local runes in components, for example:

`const isAuthenticated = $derived(session.user !== null && session.accessToken !== null);`
