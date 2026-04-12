# Implementation 003 — Phase 2 step-by-step (Rust + PostgreSQL + SQLx)

**Purpose:** Ordered checklist to implement **core domain modeling** in **this repo’s stack**: **Rust**, **PostgreSQL**, **`sqlx` migrations** under [`apps/api/migrations/`](../../apps/api/migrations/), and future **modular Rust code** under [`apps/api/src/modules/`](../../apps/api/src/). No Drizzle, no Node schema layer—**SQL files are the source of truth**.

**Canonical field-by-field spec:** [implementation_003_domain_modeling.md](./implementation_003_domain_modeling.md) (enums, every table, FKs, indexes, identity rules).

**After Phase 2 schema:** [implementation_003_orders_dispatch_tokens_redis.md](./implementation_003_orders_dispatch_tokens_redis.md) (orders with media, dispatch rounds, tokens, Redis)—**do not** start that until core tables from this guide exist.

---

## Part 0 — Stack and conventions

| Piece | Location / command |
|--------|-------------------|
| Migrations | `apps/api/migrations/*.up.sql` + matching `*.down.sql` |
| Apply | From `apps/api`: `sqlx migrate run` (see `.env.example` for `DATABASE_URL`) |
| Add migration | `sqlx migrate add <descriptive_name>` |
| Runtime | `sqlx::migrate!("./migrations")` in [`main.rs`](../../apps/api/src/main.rs) |
| Enums in Rust | `#[sqlx(type_name = "...", rename_all = "lowercase")]` on enums matching PG enums |

**Naming:** PostgreSQL `snake_case` for tables/columns. Rust structs `PascalCase` / `snake_case` fields with `sqlx::FromRow`.

---

## Part 1 — Migration sequence (what to build, in order)

Each step = **one or more** migration files. Keep **down** migrations reversible where practical (drop in reverse dependency order).

### Step 1 — New enums (no breaking changes yet)

Create types (names align with [domain doc §5](./implementation_003_domain_modeling.md)):

- `user_status` — `active`, `blocked`, `pending`
- `order_urgency` — `normal`, `urgent`, `emergency`
- `order_status` — `searching`, `dispatched`, `accepted`, `in_progress`, `completed`, `cancelled`, `expired`
- `dispatch_status` — `sent`, `viewed`, `accepted`, `rejected`, `expired`, `lost_race`
- `plumber_status_type` — `online`, `offline`, `available`, `unavailable`

**Do not** drop existing `user_role` enum.

**Status (foundations): done** — see migration `20260210120000_phase2_domain_enums` in [`apps/api/migrations/`](../../apps/api/migrations/).

---

### Step 2 — Geography reference tables

1. **`cities`** — id, name, slug UNIQUE, is_active, timestamps  
2. **`areas`** — id, city_id → cities, name, slug, UNIQUE (city_id, slug), is_active, timestamps  
3. **`streets`** — id, city_id, area_id nullable, name, slug, partial UNIQUEs for (city_id, area_id, slug) when area set / (city_id, slug) when area null, is_active, timestamps  

Indexes: per [domain doc §6.4–6.6](./implementation_003_domain_modeling.md).

---

### Step 3 — Service catalog

1. **`service_categories`** — slug UNIQUE, sort_order, is_active, timestamps  
2. **`service_price_guides`** — FKs to category, optional city/area, min/max price, CHECK min ≤ max, currency, duration, emergency flag, timestamps  

Indexes: FK columns + composite for lookups.

---

### Step 4 — Extend `users` (migrate from `is_active`)

Today: `is_active boolean`. Target: **`user_status`** column.

1. Add `user_status` with default `active`.  
2. Backfill: `is_active = false` → `blocked`, `true` → `active`; optionally `pending` where email not verified (product choice).  
3. Add `last_login_at`, `blocked_at`, `deleted_at` nullable.  
4. Drop `is_active` **after** Rust code reads `user_status` (may be a **follow-up PR**: add column first, deploy app, then drop old column).

Indexes: `(role, user_status, created_at DESC)`, `(user_status, created_at DESC)` — repo uses column name **`user_status`** (type `user_status`); match Rust `User` field and `sqlx::Type`.

**Status (foundations): done** — migration `20260210120001_users_user_status` (drops `is_active`, adds timestamps + indexes); Rust `User` / `UserStatus` in [`apps/api/src/modules/users/`](../../apps/api/src/modules/users/).

---

### Step 5 — `plumber_profiles` → surrogate `id` + dispatch columns

**Current PK:** `user_id`. Target:

1. Add **`id UUID PRIMARY KEY DEFAULT gen_random_uuid()`** (or add column, backfill, swap PK in transaction).  
2. Keep **`user_id UUID NOT NULL UNIQUE REFERENCES users ON DELETE CASCADE`**.  
3. Rename `years_of_experience` → `experience_years` if you want parity with the doc (or keep old name and document mapping).  
4. Add all dispatch fields: approval, online/available, current geography FKs, lat/lng, `service_radius_km`, `last_location_updated_at`, rating aggregates, counts, bio, avatar, `approved_by` → users, timestamps.  

Indexes: approved/online/available, `current_city_id`, `current_area_id`, `last_location_updated_at`, optional composite for hot dispatch query.

**Follow-up:** Update [`PlumberProfile`](../../apps/api/src/modules/users/model.rs) and all SQL that selected `user_id` as PK to use **`id`** for new joins; `user_id` remains for auth linkage.

**Status (foundations): surrogate `id` done** — migration `20260210120002_plumber_profiles_surrogate_id`; model + repository return `id`. **Not yet:** dispatch columns, `experience_years` rename, dispatch indexes.

---

### Step 6 — `client_profiles`

New table: 1:1 with `users` (UNIQUE `user_id`), optional default city/area/street FKs, address_line, lat/lng, timestamps.

---

### Step 7 — Plumber capability tables

All reference **`plumber_profiles.id`**:

- **`plumber_services`** — UNIQUE (plumber_id, service_category_id)  
- **`plumber_service_areas`** — city required, area nullable; UNIQUE with partial indexes for NULL area if needed  
- **`plumber_status_history`** — plumber_id, status_type enum, meta jsonb, created_at  

---

### Step 8 — `orders`

FKs: `client_id` → **users.id**, `assigned_plumber_id` → **plumber_profiles.id** (nullable), category, city, optional area/street, address_line, lat/lng, description, urgency, status, price fields, lifecycle timestamps, cancel_reason, created_at/updated_at.

Indexes: status+requested_at, client_id, assigned_plumber_id, service_category_id, city_id, area_id, requested_at.

---

### Step 9 — `order_dispatches`

FKs: order_id, plumber_id → plumber_profiles.id, dispatch_rank, dispatch_status, sent_at, responded_at, created_at.  
**UNIQUE (order_id, plumber_id).**

Indexes: order_id, plumber_id, status+sent_at, composite for plumber analytics.

---

### Step 10 — `reviews`

UNIQUE(order_id). FKs: client_id → users, plumber_id → plumber_profiles.id. Rating CHECK 1–5.

---

### Step 11 — `admin_audit_logs`

`admin_id` → users.id. Prefer **nullable** `admin_id` if you need system-generated rows later; if NOT NULL, use RESTRICT on delete. `entity_id` as TEXT for flexibility.

Indexes: (admin_id, created_at DESC), (entity_type, entity_id), (created_at DESC).

---

### Step 12 — Seeds (optional, dev/staging)

- SQL or `sqlx` fixture: **Tbilisi** city + sample areas; **service_categories** rows (leak repair, drain cleaning, …).  
- No fake **orders** in production migrations.

---

## Part 2 — How to generate and apply migrations (SQLx)

1. **Database:** PostgreSQL running; `DATABASE_URL` in `apps/api/.env` (see [`.env.example`](../../apps/api/.env.example)).  
2. **Create:**  
   `cd apps/api && sqlx migrate add phase2_user_status`  
   Edit the generated `*_phase2_user_status.up.sql` / `.down.sql`.  
3. **Apply:**  
   `sqlx migrate run`  
4. **CI / compile-time:** If the project uses `sqlx` offline mode, run **`cargo sqlx prepare`** after schema changes and commit the query metadata (follow existing project practice).  
5. **Tests:** `#[sqlx::test(migrations = "./migrations")]` picks up new migrations automatically—extend repository tests when you add queries.

**Tip:** One logical “step” from Part 1 can be **split** into multiple migrate files to keep reviews small; keep **dependency order** (enums before tables that use them).

---

## Part 3 — Why this model fits admin, dispatch, and analytics

| Need | How the schema helps |
|------|----------------------|
| **Admin** user lists | `users(role, user_status, created_at)`; soft `deleted_at` |
| **Admin** geography & services | Normalized cities/areas/streets + `is_active`; categories + price guides |
| **Admin** orders | `orders` status, geography FKs, timestamps for filters |
| **Dispatch** pre-filter | `plumber_profiles` approval + online/available + location freshness + radius; `plumber_services` + `plumber_service_areas` |
| **Dispatch** distance | `orders.lat/lng` vs `plumber_profiles.current_lat/lng` (app or PostGIS later) |
| **Race / fairness** | `order_dispatches` per plumber attempt, ranks, statuses, timestamps |
| **Quality** | `reviews` + denormalized aggregates on plumber profile |
| **Charts** | Time-series on `orders.requested_at`; group by city, category, status; dispatch funnel from `order_dispatches.status` |
| **Audit** | `admin_audit_logs` for approvals, blocks, config changes |

**Identity coherence (recap):** **`orders.client_id` → users.id**; **plumber-scoped FKs → plumber_profiles.id** so joins to services/areas/dispatches stay on one surrogate. Documented in [domain doc §4](./implementation_003_domain_modeling.md).

---

## Part 4 — Recommendations before the next phase

1. **Surrogate `plumber_profiles.id`:** Do it early so you never FK to `user_id` from orders/dispatches.  
2. **`users.user_status` vs `is_active`:** Avoid long-term dual sources; migrate app + DB in two deploys if needed.  
3. **`admin_audit_logs.admin_id`:** If NOT NULL, deleting an admin user is awkward—prefer nullable + SET NULL or never delete admin rows.  
4. **Street uniqueness:** Use **partial unique indexes** for NULL `area_id` (see domain doc).  
5. **Price guides:** Defer strict UNIQUE (category, city, area) until product defines “one row per scope”; add when admin UI is clear.  
6. **PostGIS:** Not required for Phase 2; add `geography` column + GIST later if Haversine in SQL becomes hot.  
7. **Next doc:** Implement **`order_media`**, **`offer_round`**, token ledger per [orders/dispatch/redis guide](./implementation_003_orders_dispatch_tokens_redis.md)—not in minimal Phase 2 unless you want one fewer migration wave.

---

## Rust module layout (after migrations; no business logic required in Phase 2)

Mirror domains for **repositories / models** when you start CRUD:

```
apps/api/src/modules/
  users/          # extend User, PlumberProfile, add ClientProfile
  geography/      # City, Area, Street repos (read-heavy first)
  services/       # categories, price_guides
  plumbers/       # profile updates, plumber_services, service_areas (optional split from users)
  orders/         # orders + order_dispatches
  reviews/
  admin_audit/
```

Keep **auth** in existing `auth` / `users` split as you prefer; the important part is **one query module per bounded context** so dispatch and admin features do not become a single god file.

---

## Verification checklist (Phase 2 complete)

- [ ] All enums exist; Rust `Type` enums match PostgreSQL labels.  
- [ ] Geography + services tables seeded or empty but migratable.  
- [ ] `users` extended; app compiles against new columns.  
- [ ] `plumber_profiles.id` backfilled; new tables reference `plumber_profiles.id`.  
- [ ] `orders`, `order_dispatches`, `reviews`, `admin_audit_logs` created with documented FKs and indexes.  
- [ ] `sqlx migrate run` clean from empty DB to head.  
- [ ] Existing auth tests still pass; add minimal smoke test inserting one city + category if useful.

---

*This file is the **implementation order** for Phase 2. The **data dictionary** remains in `implementation_003_domain_modeling.md`.*
