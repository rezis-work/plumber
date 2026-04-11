# Auth API routes (browser / Phase B client)

Base path: `/auth` on the Rust API (default **http://127.0.0.1:3001**). With the Vite dev proxy, call **same-origin** paths `/auth/...` from the browser.

**Typed client:** [`client.ts`](./client.ts) (`authLogin`, `authRefresh`, `authLogout`, `authLogoutAll`, `authMe`) and [`types.ts`](./types.ts).

| Method | Path | Notes |
|--------|------|--------|
| `POST` | `/auth/register/client` | JSON `{ email, password }` |
| `POST` | `/auth/register/plumber` | JSON `{ email, password, full_name, phone, years_of_experience }` |
| `POST` | `/auth/login` | JSON `{ email, password }`; sets httpOnly refresh cookie |
| `POST` | `/auth/refresh` | Uses refresh cookie; returns new access token |
| `POST` | `/auth/logout` | Clears refresh session |
| `GET` | `/auth/me` | Bearer access token |
| `POST` | `/auth/logout-all` | Bearer access token |

**Planned (not implemented yet):** `POST /auth/verify-email` — see [implementation_001_auth_api.md](../../../docs/implementation_001_auth/implementation_001_auth_api.md) for contracts and errors.
