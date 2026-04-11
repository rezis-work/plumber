# Implementation 001 — Authentication & users (index)

This is the **entry document** for Feature 001: auth and users (login, tokens, RBAC, sessions).

**Detailed guides (step-by-step instructions for implementation planning; no code):**

| Guide | File |
|--------|------|
| **Backend (Rust Axum)** | [implementation_001_auth/implementation_001_auth_api.md](./implementation_001_auth/implementation_001_auth_api.md) |
| **Frontend (SvelteKit)** | [implementation_001_auth/implementation_001_auth_frontend.md](./implementation_001_auth/implementation_001_auth_frontend.md) |
| **Mobile (Expo)** | [implementation_001_auth/implementation_001_auth_mobile.md](./implementation_001_auth/implementation_001_auth_mobile.md) |

**Order:** Implement and verify the **API** guide first, then **frontend**, then **mobile**.

**Design (Stitch):** Project **`9702559548791545108`**; Stitch MCP + `curl -L` for hosted URLs—details in the frontend guide. **Web:** **Phase 0** Design System; **Phase 0B** landing desktop (`54764d32cd774878a96490bdfc6b3f72`); **Phase C** Client Registration (`b4e85f70dda04fd18cbc2ded66367040`), Verify Email (`2895e33e817143538c664b94e7538991`), Plumber Registration (`2c2497ba2dee4162a6abd45a76f45ff0`), Login (`3d5928e9ca844ea9a955ca21a06f0f52`). **Mobile (native UI exports):** [implementation_001_auth_mobile.md](./implementation_001_auth/implementation_001_auth_mobile.md) **Step MS.3** — Landing Mobile (`bd10bc970a374954b5a531dafcb83847`), Client Registration Mobile (`1c635e4ac20c483588a505eb52dcb53c`), Plumber Registration Mobile (`99473da95838498aa626cd37f11153b7`), Verify Email Mobile (`5c38516278094c3ca521320fbd3edf1a`), Login Mobile (`669b5b6b8ed24c179ed828e95c4fc88a`). **Mobile server state:** **TanStack Query** (`useMutation` / `useQuery`) per mobile guide **Phase MQ**.

**Database:** Use **[Neon](https://neon.tech)** as the managed **PostgreSQL** provider for the API (connection strings, pooling, and migration workflow are spelled out in the API guide).

**Folder overview:** [implementation_001_auth/README.md](./implementation_001_auth/README.md)

---

## Goals (reminder)

- Roles: `client`, `plumber`, `admin`.
- **Two public registration flows:** **client** (email + password; email verification token returned in API response until email sending exists) and **plumber** (“become a plumber”: email, password, full name, phone, years of experience)—different endpoints and UIs.
- **access** JWT in `Authorization: Bearer`; **refresh** JWT in **httpOnly** cookie with **DB-backed** sessions, rotation, and revocation.
- **Admin** never via public API.
- RBAC on protected routes; generic login errors.

Use the linked guides when prompting an AI or assigning tasks: each is split into ordered steps with acceptance-style verification.
