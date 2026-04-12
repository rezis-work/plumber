# Implementation 003 — Core database and domain modeling (index)

This folder defines **Phase 2: full PostgreSQL foundation** for the plumber platform—**schema, migrations, relationships, indexes, enums**—so later work (admin, dispatch, closest-plumber matching, order lifecycle, charts, reviews) stays clean. **No full business logic** in this phase beyond what you need to validate migrations or seed reference data.

**Product context:** On-demand plumbing (Bolt/Uber-style): client request → match plumbers → offer to small batches (e.g. top 3) → first accept wins; **human-readable geography** (city/area/street) plus **lat/lng** for distance; roles **admin / client / plumber** (auth already exists in [`apps/api`](../apps/api/)).

**Backend stack for this repo (use this, not Drizzle/Node for Phase 2):**

| Layer | Choice |
|-------|--------|
| API | **Rust** — [`apps/api`](../apps/api/) |
| DB | **PostgreSQL** |
| Migrations | **SQL files** + **`sqlx migrate`** — [`apps/api/migrations/`](../apps/api/migrations/) |
| ORM | **`sqlx`** (`FromRow`, typed enums) — not Drizzle |

If you ever add a Node service, **mirror the same SQL**; PostgreSQL remains the contract.

---

## Guides (read in this order)

| Step | Guide | Purpose |
|------|--------|---------|
| **1 — Do this first** | [implementation_003_domain_modeling/implementation_003_phase2_rust_sqlx_step_by_step.md](./implementation_003_domain_modeling/implementation_003_phase2_rust_sqlx_step_by_step.md) | **Ordered migration steps**, `sqlx` workflow, Rust module layout, verification checklist, Parts 1–4 (stack-native) |
| **2 — Reference while coding** | [implementation_003_domain_modeling/implementation_003_domain_modeling.md](./implementation_003_domain_modeling/implementation_003_domain_modeling.md) | **Full data dictionary:** every enum, table, column, FK, index, identity rules (`orders.client_id` → `users.id`, plumber FKs → `plumber_profiles.id`), ON DELETE behavior |
| **3 — After Phase 2 schema** | [implementation_003_domain_modeling/implementation_003_orders_dispatch_tokens_redis.md](./implementation_003_domain_modeling/implementation_003_orders_dispatch_tokens_redis.md) | Order **media**, **multi-round** dispatch, **token ledger**, **Redis (Upstash)** — extends schema with extra tables/columns |

**Folder overview:** [implementation_003_domain_modeling/README.md](./implementation_003_domain_modeling/README.md)

---

## Quick implementation order (summary)

1. **Enums** — `user_status`, `order_*`, `dispatch_status`, `plumber_status_type` (keep existing `user_role`).  
2. **Geography** — `cities`, `areas`, `streets`.  
3. **Services** — `service_categories`, `service_price_guides`.  
4. **Users** — extend `users` (status, soft delete fields); migrate off `is_active` when app is ready.  
5. **Plumber profiles** — surrogate `id`, dispatch columns; keep `user_id` unique.  
6. **Client profiles** — `client_profiles`.  
7. **Plumber capabilities** — `plumber_services`, `plumber_service_areas`, `plumber_status_history`.  
8. **Operations** — `orders`, `order_dispatches`, `reviews`, `admin_audit_logs`.  
9. **Indexes** — as in the reference doc (admin filters, dispatch pre-filter, analytics).  
10. **Seeds** — optional cities/services for dev.  
11. **Next** — orders/dispatch/tokens/Redis guide.

Details and commands: **step-by-step guide** (row 1 in the table above).

---

## Deliverables checklist (Phase 2)

- [ ] **Migrations** — `.up.sql` / `.down.sql` covering all tables and enums above.  
- [ ] **Relationships** — FKs and ON DELETE as in the reference doc §7.  
- [ ] **Indexes** — per reference doc §6 and indexing notes.  
- [ ] **Rust** — `sqlx` enums and structs updated when you expose new data (can trail migrations slightly).  
- [ ] **Explanation** — Part 3–4 in the step-by-step guide + narrative in the reference doc §9.

Use the **step-by-step** file when assigning tasks or prompting implementation; use the **domain modeling** file as the single **schema truth** while writing SQL.
