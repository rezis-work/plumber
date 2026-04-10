# Implementation 001 — Authentication & users (index)

This is the **entry document** for Feature 001: auth and users (login, tokens, RBAC, sessions).

**Detailed guides (step-by-step instructions for implementation planning; no code):**

| Guide | File |
|--------|------|
| **Backend (Rust Axum)** | [implementation_001_auth/implementation_001_auth_api.md](./implementation_001_auth/implementation_001_auth_api.md) |
| **Frontend (SvelteKit)** | [implementation_001_auth/implementation_001_auth_frontend.md](./implementation_001_auth/implementation_001_auth_frontend.md) |
| **Mobile (Expo)** | [implementation_001_auth/implementation_001_auth_mobile.md](./implementation_001_auth/implementation_001_auth_mobile.md) |

**Order:** Implement and verify the **API** guide first, then **frontend**, then **mobile**.

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
