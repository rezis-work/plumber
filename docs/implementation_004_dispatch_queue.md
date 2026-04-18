# Implementation 004 — Dispatch work queue (index)

This feature set adds a **reliable work queue** so a new **`orders`** row in **`searching`** automatically triggers **first-round dispatch** (and optionally other async dispatch work) **without** blocking `POST /orders` and **without** relying on a human calling internal advance routes.

**Source of truth:** **PostgreSQL** — `orders`, `order_dispatches`, statuses, and (recommended) a small **outbox / jobs** table. **Redis** holds **transient queue entries** and optional **locks / TTL mirrors** already described in Implementation 003.

**Stack:** **Rust** API in [`apps/api`](../apps/api/), **PostgreSQL**, **Redis (Upstash)** — same assumptions as [Implementation 003 — orders / dispatch / tokens](./implementation_003_domain_modeling/implementation_003_orders_dispatch_tokens_redis.md).

---

## Guides

| Document | Purpose |
|----------|---------|
| [implementation_004_dispatch_queue/implementation_004_dispatch_queue_redis_postgres.md](./implementation_004_dispatch_queue/implementation_004_dispatch_queue_redis_postgres.md) | **Read this:** queue shapes, outbox pattern, Redis keys, worker loop, idempotency, reconciliation, rollout phases — **ordered build steps in §12** |

**Folder overview:** [implementation_004_dispatch_queue/README.md](./implementation_004_dispatch_queue/README.md)

---

## Suggested order

1. Read **Implementation 004** (this index’s linked guide) and agree **outbox vs Redis-only** approach.
2. Add **PostgreSQL** migration for the outbox table (if you choose the recommended pattern).
3. Implement **enqueue** after `create_order` commits (API) and **consumer** (same binary or dedicated worker).
4. Keep **cron `expire-due`** and **reject-path advance** as today; the queue is for **bootstrap** and optional **backpressure**, not a replacement for round advancement.

**Prerequisite:** [Implementation 003 — orders, dispatch, tokens, Redis](./implementation_003_domain_modeling/implementation_003_orders_dispatch_tokens_redis.md) (schema + dispatch writer behavior).
