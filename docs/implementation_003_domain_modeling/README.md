# Implementation 003 — Domain modeling

PostgreSQL schema and **Rust + SQLx** migration workflow for Phase 2 (users, profiles, geography, services, orders, dispatch rows, reviews, audit).

| Order | Document |
|-------|----------|
| **Start here** | [implementation_003_phase2_rust_sqlx_step_by_step.md](./implementation_003_phase2_rust_sqlx_step_by_step.md) — step-by-step migrations, `sqlx` commands, Parts 1–4, verification |
| **Reference** | [implementation_003_domain_modeling.md](./implementation_003_domain_modeling.md) — full enums, tables, FKs, indexes, identity model |
| **After core schema** | [implementation_003_orders_dispatch_tokens_redis.md](./implementation_003_orders_dispatch_tokens_redis.md) — media, dispatch rounds, tokens, Redis |
| **Auto-start dispatch (queue)** | [../implementation_004_dispatch_queue/implementation_004_dispatch_queue_redis_postgres.md](../implementation_004_dispatch_queue/implementation_004_dispatch_queue_redis_postgres.md) — PostgreSQL outbox + Redis; workers |

**Repo index:** [../implementation_003_domain_modeling.md](../implementation_003_domain_modeling.md) · [../implementation_004_dispatch_queue.md](../implementation_004_dispatch_queue.md)

**API migrations:** [`../../apps/api/migrations/`](../../apps/api/migrations/)
