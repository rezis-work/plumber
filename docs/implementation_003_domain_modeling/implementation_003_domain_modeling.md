# Implementation 003 — Core database and domain modeling

**Purpose:** Define the **full PostgreSQL relational model** for the on-demand plumber platform: users/profiles, geography (human + coordinates), services and price guides, plumber capabilities, orders, dispatch tracking, reviews, and admin audit logs.

**Phase boundary (this document):** schema, constraints, indexes, migration ordering, and **seed-ready** reference data—**not** dispatch algorithms, payment capture, or notification workers. Application validation can be added in Rust (or another layer) later; this phase focuses on the database.

**Implementation order:** Follow [implementation_003_phase2_rust_sqlx_step_by_step.md](./implementation_003_phase2_rust_sqlx_step_by_step.md) first (this file is the **reference** / data dictionary).

**Product context:** Clients create plumbing requests; the backend will eventually match **approved, online, available** plumbers by **service**, **working area**, **distance** (lat/lng + radius), and **ranking**; send to **top N** (e.g. 3); **first accept** wins; others move to **`lost_race`** / **`expired`**. Geography must support **admin/UI filtering** (city/area/street) and **dispatch** (coordinates + radius + recency).

---

## 1. Goals and non-goals

### Goals

- One coherent **identity model** for FKs (`users` vs `client_profiles` / `plumber_profiles`).
- **Enums** in PostgreSQL for roles, statuses, urgency, dispatch outcomes, plumber telemetry.
- **Reference geography** (cities → areas → streets) and **lat/lng** on orders and plumber live location.
- **Orders** and **order_dispatches** ready for race and analytics.
- **Indexes** aligned with admin filters, dispatch pre-filtering, and time-series analytics.
- **Soft deactivation** where it reduces support pain (users, reference rows) without destroying history.

### Non-goals (later phases)

- Implementing matching, notifications, or state machines in application code.
- Pricing engines, payouts, or tax.
- Real-time WebSocket layer (schema only prepares fields such as `last_location_updated_at`).

---

## 2. Backend implementation path (this repo)

**Primary:** Extend **[`apps/api`](../../apps/api)** — **Rust**, **PostgreSQL**, **`sqlx` migrations** in [`apps/api/migrations/`](../../apps/api/migrations/). See [implementation_003_phase2_rust_sqlx_step_by_step.md](./implementation_003_phase2_rust_sqlx_step_by_step.md) for `sqlx migrate add` / `sqlx migrate run` and the ordered migration checklist.

The **database** is the contract. Another language stack must **mirror the same SQL**; **table and enum names** stay stable.

---

## 3. Conventions

- **Identifiers:** `UUID` primary keys (`gen_random_uuid()`), except where a natural 1:1 suggests `user_id` as alternate unique key.
- **Timestamps:** `created_at`, `updated_at` as `TIMESTAMPTZ NOT NULL` with `DEFAULT now()`; app or trigger updates `updated_at` on row change (existing `users` note applies).
- **Soft delete:** `deleted_at TIMESTAMPTZ NULL` on `users` and optionally on reference entities; **orders** and **dispatches** remain **hard rows** for audit (cancel/expire via `status` + timestamps).
- **Money:** `NUMERIC(12, 2)` for `min_price`, `max_price`, `final_price`; **`currency`** as `CHAR(3)` (ISO 4217, e.g. `GEL`) with a `CHECK (currency ~ '^[A-Z]{3}$')` or a small `currency` enum if you prefer closed set.
- **Text search (later):** optional `tsvector` / GIN not required in this phase; document as future optimization.
- **Naming:** `snake_case` in SQL to match existing migrations.

---

## 4. Identity model (critical decision)

**Principle:** Operational tables that belong to a **person** use **`users.id`** for clients; tables that belong to the **plumber as a service provider** use **`plumber_profiles.id`** because dispatch, services, and service areas are **profile-scoped**.

| Table | Column | References | Rationale |
|-------|--------|------------|-----------|
| `client_profiles` | `user_id` | `users.id` | One row per client user; order’s “who asked” is the **account**. |
| `orders` | `client_id` | **`users.id`** | Matches auth and reviews; enforce **role = client** in app or deferred constraint. |
| `reviews` | `client_id` | **`users.id`** | Same as order’s client. |
| `orders` | `assigned_plumber_id` | **`plumber_profiles.id`** | Joins dispatch, `plumber_services`, `plumber_service_areas` on **profile id**. |
| `order_dispatches` | `plumber_id` | **`plumber_profiles.id`** | Same. |
| `plumber_services` | `plumber_id` | **`plumber_profiles.id`** | |
| `plumber_service_areas` | `plumber_id` | **`plumber_profiles.id`** | |
| `plumber_status_history` | `plumber_id` | **`plumber_profiles.id`** | |
| `admin_audit_logs` | `admin_id` | **`users.id`** | Admin is a **user**; role enforced in app. |

**`plumber_profiles` primary key (evolution from current repo):** Today `plumber_profiles.user_id` is the **PRIMARY KEY**. For cleaner FKs from many child tables, **add** `id UUID PRIMARY KEY DEFAULT gen_random_uuid()`, keep **`user_id UUID NOT NULL UNIQUE REFERENCES users`**, backfill `id`, then migrate FKs to `plumber_profiles.id`. Until then, new tables can reference `user_id` as “plumber id” but the **target end state** is **`plumber_profiles.id`** as documented here.

---

## 5. Enums

Implement as PostgreSQL `CREATE TYPE ... AS ENUM (...)` (or check constraints + lookup tables if you avoid enums—enums are fine for stable closed sets).

| Enum | Values | Used on |
|------|--------|---------|
| `user_role` | `client`, `plumber`, `admin` | `users.role` (exists) |
| `user_status` | `active`, `blocked`, `pending` | `users.user_status` (column name; type `user_status`) — **migrate** from `is_active` (see §6.1) |
| `order_urgency` | `normal`, `urgent`, `emergency` | `orders.urgency` |
| `order_status` | `searching`, `dispatched`, `accepted`, `in_progress`, `completed`, `cancelled`, `expired` | `orders.status` |
| `dispatch_status` | `sent`, `viewed`, `accepted`, `rejected`, `expired`, `lost_race` | `order_dispatches.status` |
| `plumber_status_type` | `online`, `offline`, `available`, `unavailable` | `plumber_status_history.status_type` |

**Note:** `plumber_profiles.is_online` / `is_available` are **booleans** for fast filtering; **history** uses `plumber_status_type` for analytics (you can log transitions when either flag changes).

Optional: `currency` enum restricted to `{ GEL, USD, EUR }` if you never need arbitrary ISO codes. **Repo:** not implemented yet; add the PostgreSQL type and a matching Rust `sqlx::Type` in the same migration that introduces money columns, if the product locks to those three codes only.

---

## 6. Tables by domain

### 6.1 `users` (extend existing)

**Purpose:** Single identity and auth row for all roles.

#### As implemented (repo)

**Columns:** `id`, `email` (CITEXT UNIQUE), `password_hash`, `role`, `user_status`, `last_login_at`, `blocked_at`, `deleted_at`, `is_email_verified`, verification token columns, `created_at`, `updated_at`. (`is_active` removed; see migration `20260210120001_users_user_status`.)

**Indexes:** `(role, user_status, created_at DESC)`, `(user_status, created_at DESC)`; UNIQUE on `email`.

**Runtime behavior:**

- Successful **password login** updates `last_login_at` (and `updated_at`).
- **Refresh** re-loads the user and rejects the rotation if `login_allowed()` is false (non-active `user_status` or `deleted_at` set).
- **Admin** (Bearer access JWT, `role = admin`): `POST /auth/admin/users/{user_id}/block` sets `user_status = blocked` and sets `blocked_at` only when transitioning from a non-blocked status; `POST /auth/admin/users/{user_id}/soft-delete` sets `deleted_at` (idempotent no-op if already deleted). Both return `403` if `user_id` is the caller’s own id.

#### Reference shape (columns)

| Column | Type | Notes |
|--------|------|--------|
| `user_status` | `user_status` NOT NULL | Migrated from `is_active` (`true` → `active`, `false` → `blocked`). |
| `last_login_at` | `TIMESTAMPTZ` NULL | Updated on successful password login. |
| `blocked_at` | `TIMESTAMPTZ` NULL | Set when `user_status` becomes `blocked` (backfill + admin block). |
| `deleted_at` | `TIMESTAMPTZ` NULL | Soft delete; hides from login / refresh; admin soft-delete route. |

**Constraints:** `email` UNIQUE (existing). Consider partial unique index on `lower(email)` if you drop `citext` (keep `citext` as today).

#### Follow-ups (product / later)

- **New client signups → `pending` until email verified:** not the current default (registrations use `active`); would require tightening login/refresh for unverified clients.
- **Richer admin UX:** list/filter users, unblock, clear `deleted_at`, audit logs.
- **Refresh as “activity”:** optional update of `last_login_at` on refresh (not implemented; field name suggests login only).

---

### 6.2 `client_profiles` (new)

**Purpose:** Client-only attributes; optional default address and coordinates for faster repeat requests.

| Column | Type | FK / notes |
|--------|------|------------|
| `id` | UUID PK | |
| `user_id` | UUID NOT NULL UNIQUE | → `users.id` **ON DELETE CASCADE** |
| `full_name` | TEXT NOT NULL | |
| `phone` | TEXT NOT NULL | |
| `default_city_id` | UUID NULL | → `cities.id` **ON DELETE SET NULL** |
| `default_area_id` | UUID NULL | → `areas.id` **ON DELETE SET NULL** |
| `default_street_id` | UUID NULL | → `streets.id` **ON DELETE SET NULL** |
| `address_line` | TEXT NULL | Free-form complement |
| `lat` | DOUBLE PRECISION NULL | Optional home / default pin |
| `lng` | DOUBLE PRECISION NULL | |
| `created_at`, `updated_at` | TIMESTAMPTZ | |

**Indexes:** `user_id` (unique already). Optional `(default_city_id)` for regional analytics.

**Rule:** At most one profile per user; app enforces **role = client** on insert.

---

### 6.3 `plumber_profiles` (extend existing)

**Purpose:** Plumber public/dispatch profile: approval, live location, radius, aggregates for ranking.

**Current (repo):** `user_id` PK, `full_name`, `phone`, `years_of_experience`.

**Target shape:**

| Column | Type | FK / notes |
|--------|------|------------|
| `id` | UUID PK | **New**; backfill migration |
| `user_id` | UUID NOT NULL UNIQUE | → `users.id` **ON DELETE CASCADE** |
| `full_name` | TEXT NOT NULL | |
| `phone` | TEXT NOT NULL | |
| `experience_years` | INTEGER NOT NULL | Rename from `years_of_experience` in migration if desired |
| `bio` | TEXT NULL | |
| `avatar_url` | TEXT NULL | |
| `is_approved` | BOOLEAN NOT NULL DEFAULT false | |
| `approved_at` | TIMESTAMPTZ NULL | |
| `approved_by` | UUID NULL | → `users.id` **ON DELETE SET NULL** (admin user) |
| `is_online` | BOOLEAN NOT NULL DEFAULT false | |
| `is_available` | BOOLEAN NOT NULL DEFAULT false | |
| `current_city_id` | UUID NULL | → `cities.id` **ON DELETE SET NULL** |
| `current_area_id` | UUID NULL | → `areas.id` **ON DELETE SET NULL** |
| `current_street_id` | UUID NULL | → `streets.id` **ON DELETE SET NULL** |
| `current_lat` | DOUBLE PRECISION NULL | |
| `current_lng` | DOUBLE PRECISION NULL | |
| `service_radius_km` | NUMERIC(8, 3) NOT NULL DEFAULT 5 | Dispatch eligibility |
| `last_location_updated_at` | TIMESTAMPTZ NULL | Stale location filtering |
| `rating_avg` | NUMERIC(4, 3) NOT NULL DEFAULT 0 | Denormalized |
| `rating_count` | INTEGER NOT NULL DEFAULT 0 | |
| `completed_orders_count` | INTEGER NOT NULL DEFAULT 0 | |
| `cancelled_orders_count` | INTEGER NOT NULL DEFAULT 0 | |
| `created_at`, `updated_at` | TIMESTAMPTZ | |

**Indexes (dispatch + admin):**

- `(is_approved, is_online, is_available)` — partial `WHERE is_approved = true AND is_online = true AND is_available = true` optional for hot path.
- `(current_city_id)`, `(current_area_id)`.
- `(last_location_updated_at DESC)` — freshness.
- Composite example: `(is_approved, is_online, is_available, current_city_id)` if most queries filter by city.

**Check:** `service_radius_km > 0`, `experience_years >= 0`, `rating_avg BETWEEN 0 AND 5` (if 5-star scale).

---

### 6.4 Geography — `cities`

| Column | Type | Notes |
|--------|------|--------|
| `id` | UUID PK | |
| `name` | TEXT NOT NULL | Display |
| `slug` | TEXT NOT NULL UNIQUE | URL/admin keys |
| `is_active` | BOOLEAN NOT NULL DEFAULT true | Soft deactivation |
| `created_at`, `updated_at` | TIMESTAMPTZ | |

**Indexes:** UNIQUE(`slug`), `(is_active)`.

---

### 6.5 Geography — `areas`

| Column | Type | FK |
|--------|------|-----|
| `id` | UUID PK | |
| `city_id` | UUID NOT NULL | → `cities.id` **ON DELETE CASCADE** |
| `name` | TEXT NOT NULL | |
| `slug` | TEXT NOT NULL | |
| `is_active` | BOOLEAN NOT NULL DEFAULT true | |
| `created_at`, `updated_at` | TIMESTAMPTZ | |

**Constraints:** **UNIQUE (`city_id`, `slug`)**.

**Indexes:** `(city_id)`, `(city_id, is_active)`.

---

### 6.6 Geography — `streets`

| Column | Type | FK |
|--------|------|-----|
| `id` | UUID PK | |
| `city_id` | UUID NOT NULL | → `cities.id` **ON DELETE CASCADE** |
| `area_id` | UUID NULL | → `areas.id` **ON DELETE SET NULL** |
| `name` | TEXT NOT NULL | |
| `slug` | TEXT NOT NULL | |
| `is_active` | BOOLEAN NOT NULL DEFAULT true | |
| `created_at`, `updated_at` | TIMESTAMPTZ | |

**Constraints:** PostgreSQL treats NULLs as distinct in UNIQUE; use:

- **Partial unique:** `UNIQUE (city_id, area_id, slug) WHERE area_id IS NOT NULL`
- **Partial unique:** `UNIQUE (city_id, slug) WHERE area_id IS NULL`

**Indexes:** `(city_id)`, `(area_id)`, `(city_id, area_id)`.

---

### 6.7 `service_categories`

| Column | Type | Notes |
|--------|------|--------|
| `id` | UUID PK | |
| `name` | TEXT NOT NULL | |
| `slug` | TEXT NOT NULL UNIQUE | |
| `description` | TEXT NULL | |
| `icon` | TEXT NULL | URL or icon key |
| `is_active` | BOOLEAN NOT NULL DEFAULT true | |
| `sort_order` | INTEGER NOT NULL DEFAULT 0 | |
| `created_at`, `updated_at` | TIMESTAMPTZ | |

**Indexes:** UNIQUE(`slug`), `(is_active, sort_order)`.

---

### 6.8 `service_price_guides`

**Purpose:** Approximate market range per category and optional geography—not final invoice price.

| Column | Type | FK |
|--------|------|-----|
| `id` | UUID PK | |
| `service_category_id` | UUID NOT NULL | → `service_categories.id` **ON DELETE CASCADE** |
| `city_id` | UUID NULL | → `cities.id` **ON DELETE CASCADE** |
| `area_id` | UUID NULL | → `areas.id` **ON DELETE CASCADE** |
| `min_price`, `max_price` | NUMERIC(12,2) NOT NULL | |
| `currency` | CHAR(3) NOT NULL | e.g. `GEL` |
| `estimated_duration_minutes` | INTEGER NOT NULL | |
| `is_emergency_supported` | BOOLEAN NOT NULL DEFAULT false | |
| `notes` | TEXT NULL | |
| `created_at`, `updated_at` | TIMESTAMPTZ | |

**Constraints:** `CHECK (min_price <= max_price)`, `CHECK (estimated_duration_minutes > 0)`.

**Indexes:** `(service_category_id)`, `(city_id)`, `(area_id)`, composite `(service_category_id, city_id, area_id)` for lookup.

**Uniqueness (optional):** prevent duplicate rows per scope, e.g. **UNIQUE (`service_category_id`, `city_id`, `area_id`)** with sentinel for NULL geography if you use NULL = “national default” (otherwise use partial uniques).

---

### 6.9 `plumber_services`

| Column | Type | FK |
|--------|------|-----|
| `id` | UUID PK | |
| `plumber_id` | UUID NOT NULL | → `plumber_profiles.id` **ON DELETE CASCADE** |
| `service_category_id` | UUID NOT NULL | → `service_categories.id` **ON DELETE CASCADE** |
| `created_at` | TIMESTAMPTZ NOT NULL DEFAULT now() | |

**Constraints:** **UNIQUE (`plumber_id`, `service_category_id`)**.

**Indexes:** `(service_category_id)` — “all plumbers offering X”, `(plumber_id)`.

---

### 6.10 `plumber_service_areas`

**Purpose:** Where the plumber **accepts work** (policy), distinct from **current location**.

| Column | Type | FK |
|--------|------|-----|
| `id` | UUID PK | |
| `plumber_id` | UUID NOT NULL | → `plumber_profiles.id` **ON DELETE CASCADE** |
| `city_id` | UUID NOT NULL | → `cities.id` **ON DELETE CASCADE** |
| `area_id` | UUID NULL | → `areas.id` **ON DELETE CASCADE** — NULL = whole city |
| `created_at` | TIMESTAMPTZ NOT NULL DEFAULT now() | |

**Constraints:** **UNIQUE (`plumber_id`, `city_id`, `area_id`)** — use partial unique for NULL `area_id` like streets, or use sentinel UUID; document chosen pattern in migration.

**Indexes:** `(city_id, area_id)`, `(plumber_id)`.

---

### 6.11 `plumber_status_history`

| Column | Type | Notes |
|--------|------|--------|
| `id` | UUID PK | |
| `plumber_id` | UUID NOT NULL | → `plumber_profiles.id` **ON DELETE CASCADE** |
| `status_type` | `plumber_status_type` NOT NULL | |
| `meta` | JSONB NULL | Optional device, reason, source |
| `created_at` | TIMESTAMPTZ NOT NULL DEFAULT now() | |

**Indexes:** `(plumber_id, created_at DESC)`, `(status_type, created_at DESC)`.

---

### 6.12 `orders`

**Purpose:** Single operational record for lifecycle + reporting; carries both **structured geography** and **lat/lng**.

| Column | Type | FK |
|--------|------|-----|
| `id` | UUID PK | |
| `client_id` | UUID NOT NULL | → `users.id` **ON DELETE RESTRICT** (keep history) |
| `assigned_plumber_id` | UUID NULL | → `plumber_profiles.id` **ON DELETE SET NULL** |
| `service_category_id` | UUID NOT NULL | → `service_categories.id` **ON DELETE RESTRICT** |
| `city_id` | UUID NOT NULL | → `cities.id` **ON DELETE RESTRICT** |
| `area_id` | UUID NULL | → `areas.id` **ON DELETE SET NULL** |
| `street_id` | UUID NULL | → `streets.id` **ON DELETE SET NULL** |
| `address_line` | TEXT NOT NULL | Required human detail |
| `lat` | DOUBLE PRECISION NOT NULL | Dispatch |
| `lng` | DOUBLE PRECISION NOT NULL | |
| `description` | TEXT NULL | |
| `urgency` | `order_urgency` NOT NULL | |
| `status` | `order_status` NOT NULL | |
| `estimated_price_min`, `estimated_price_max` | NUMERIC(12,2) NULL | From guide or quote |
| `final_price` | NUMERIC(12,2) NULL | Set on completion |
| `requested_at` | TIMESTAMPTZ NOT NULL DEFAULT now() | |
| `accepted_at` | TIMESTAMPTZ NULL | |
| `started_at` | TIMESTAMPTZ NULL | |
| `completed_at` | TIMESTAMPTZ NULL | |
| `cancelled_at` | TIMESTAMPTZ NULL | |
| `cancel_reason` | TEXT NULL | |
| `created_at`, `updated_at` | TIMESTAMPTZ | |

**Indexes:**

- `(status, requested_at DESC)` — dispatch worker, dashboards.
- `(client_id, requested_at DESC)` — client history.
- `(assigned_plumber_id)` — plumber’s jobs.
- `(service_category_id)`, `(city_id)`, `(area_id)`.
- `(requested_at DESC)` — time-series / charts.
- Optional **partial** `(status) WHERE status IN ('searching','dispatched')` for hot queue.

**PostGIS (optional later):** `geography(Point)` + GIST for heavy spatial load; v1 can use **Haversine in app** with btree on `(city_id)` pre-filter.

---

### 6.13 `order_dispatches`

**Purpose:** One row per (order, plumber) attempt; supports top-3 send, race, and analytics.

| Column | Type | Notes |
|--------|------|--------|
| `id` | UUID PK | |
| `order_id` | UUID NOT NULL | → `orders.id` **ON DELETE CASCADE** |
| `plumber_id` | UUID NOT NULL | → `plumber_profiles.id` **ON DELETE CASCADE** |
| `dispatch_rank` | SMALLINT NOT NULL | e.g. 1–3 |
| `status` | `dispatch_status` NOT NULL | |
| `sent_at` | TIMESTAMPTZ NOT NULL DEFAULT now() | |
| `responded_at` | TIMESTAMPTZ NULL | |
| `created_at` | TIMESTAMPTZ NOT NULL DEFAULT now() | |

**Constraints:** **UNIQUE (`order_id`, `plumber_id`)**; `CHECK (dispatch_rank >= 1)`.

**Indexes:**

- `(order_id)`, `(plumber_id)`.
- `(status, sent_at DESC)`.
- `(plumber_id, status, sent_at DESC)` — response analytics.

---

### 6.14 `reviews`

| Column | Type | FK |
|--------|------|-----|
| `id` | UUID PK | |
| `order_id` | UUID NOT NULL UNIQUE | → `orders.id` **ON DELETE CASCADE** — **one review per order** |
| `client_id` | UUID NOT NULL | → `users.id` **ON DELETE CASCADE** |
| `plumber_id` | UUID NOT NULL | → `plumber_profiles.id` **ON DELETE CASCADE** |
| `rating` | SMALLINT NOT NULL | `CHECK (rating BETWEEN 1 AND 5)` |
| `comment` | TEXT NULL | |
| `created_at`, `updated_at` | TIMESTAMPTZ | |

**Indexes:** `(plumber_id, created_at DESC)` — aggregates and listing; `(client_id)`.

**Consistency:** Enforce in app that `orders.client_id` / `assigned_plumber_id` match review row; optional **deferrable constraint trigger** later.

---

### 6.15 `admin_audit_logs`

| Column | Type | Notes |
|--------|------|--------|
| `id` | UUID PK | |
| `admin_id` | UUID NOT NULL | → `users.id` **ON DELETE SET NULL** or RESTRICT — if SET NULL, allow NULL `admin_id` for system actions |
| `action` | TEXT NOT NULL | e.g. `plumber.approved`, `city.created` |
| `entity_type` | TEXT NOT NULL | e.g. `plumber_profile`, `city` |
| `entity_id` | TEXT NOT NULL | String UUID or natural key |
| `meta` | JSONB NULL | Diff, IP, payload snapshot |
| `created_at` | TIMESTAMPTZ NOT NULL DEFAULT now() | |

**Indexes:** `(admin_id, created_at DESC)`, `(entity_type, entity_id)`, `(created_at DESC)`.

---

## 7. Foreign key ON DELETE summary

| From | ON DELETE | Rationale |
|------|-----------|-----------|
| Profiles → `users` | CASCADE | User removal wipes profile |
| Geography children | CASCADE from city/area as appropriate | Hierarchy cleanup |
| `orders.client_id` | RESTRICT | Prevent deleting users with orders (use `deleted_at` on user instead) |
| `orders.assigned_plumber_id` | SET NULL | Plumber profile removed → order history stays |
| `order_dispatches` → order | CASCADE | |
| `reviews` → order | CASCADE | |

---

## 8. Migration sequencing (recommended)

1. Create new **enums** (`user_status`, `order_*`, `dispatch_status`, `plumber_status_type`) without breaking existing `user_role`.
2. Add **`cities`, `areas`, `streets`**, **`service_categories`** (empty or seed).
3. **`plumber_profiles`**: add surrogate `id`, backfill, add new columns; migrate FKs from `user_id` to `id` on new tables only.
4. **`users`**: add `user_status`, backfill from `is_active`; add `last_login_at`, `blocked_at`, `deleted_at`.
5. **`client_profiles`**.
6. **`service_price_guides`**, **`plumber_services`**, **`plumber_service_areas`**, **`plumber_status_history`**.
7. **`orders`**, **`order_dispatches`**, **`reviews`**, **`admin_audit_logs`**.
8. **Indexes** (can be combined into the same migrations where table is created).

---

## 9. How this design supports later work

### Dispatch engine

- Pre-filter: `plumber_profiles` (**approved**, **online**, **available**, **service_radius_km**, **last_location_updated_at**).
- Capability: **`plumber_services`** + **`plumber_service_areas`** vs order’s **category** and **city/area**.
- Distance: **`orders.lat/lng`** vs **`plumber_profiles.current_lat/lng`** (app-side Haversine or PostGIS later).
- Race: **`order_dispatches`** with **`dispatch_rank`**, **`status`**, **`sent_at`**, **`responded_at`**.

### Admin and analytics

- User lists: **`users (role, user_status, created_at)`**.
- Geography CRUD: hierarchical tables + **`is_active`**.
- Orders: filters on **`status`, `city_id`, `service_category_id`, `requested_at`**.
- Dispatch funnel: **`order_dispatches`** grouped by **`status`** and time.
- Plumber quality: **`reviews`** + denormalized counters on **`plumber_profiles`**.

---

## 10. Modular layout in Rust (`apps/api`)

Keep **SQL migrations** as the source of truth; map tables to **`sqlx::FromRow`** structs and repositories under `src/modules/`:

- `users` — extend `User`, `PlumberProfile`; add `ClientProfile`
- `geography` — cities, areas, streets
- `services` — categories, price guides
- `plumbers` or extend `users` — `plumber_services`, `plumber_service_areas`, `plumber_status_history`
- `orders` — `orders`, `order_dispatches`
- `reviews` — `reviews`
- `admin_audit` — `admin_audit_logs`

Concrete folder suggestions: [step-by-step guide § Rust module layout](./implementation_003_phase2_rust_sqlx_step_by_step.md).

---

## 11. Seed data (structure only this phase)

- **Cities / areas / streets:** JSON or SQL seeds aligned with Tbilisi rollout; use **`slug`** stability for idempotent seeds.
- **Service categories:** seed rows matching marketing copy (leak repair, drain cleaning, …).
- **Price guides:** optional seed per city + category with **GEL** ranges.

Do **not** seed fake orders in production migrations; use dev-only seeds or fixtures.

---

## 12. Verification checklist (schema phase)

- [ ] All enums created; **user** `user_status` migrated from `is_active` without losing data.
- [ ] **`plumber_profiles.id`** backfilled and all new plumber FKs use it.
- [ ] Geography uniqueness rules enforced (especially **streets** with NULL `area_id`).
- [ ] **`orders`** enforce non-null **lat/lng**; **price guide** `min <= max`.
- [ ] **`order_dispatches`**: UNIQUE **(order_id, plumber_id)**.
- [ ] **`reviews`**: UNIQUE **(order_id)**.
- [ ] Indexes from §6 present for admin/dispatch/analytics paths.
- [ ] Down migrations or rollback notes documented for each step.

---

## Appendix A — Reference: `order_status` vs `dispatch_status` (lifecycle)

- **Order** moves **`searching` → `dispatched` → `accepted` → `in_progress` → `completed`** (or **`cancelled` / `expired`**).
- **Dispatch** rows: **`sent` → `viewed` → `accepted` | `rejected` | `expired` | `lost_race`** when another plumber wins.

Exact state machine is application-defined; the **columns** above are sufficient to persist outcomes.

---

## Appendix B — Alignment with existing Rust API

After migrations, update Rust models and handlers to:

- Read/write **`users.user_status`** instead of `is_active`.
- Use **`plumber_profiles.id`** for any new join tables.
- Keep **email verification** columns as today until a dedicated cleanup migration merges them with **`user_status.pending`**.

This document should stay the **authoritative data model**; link it from future **Implementation 004 — Dispatch** and **Implementation 005 — Admin dashboard** specs.
