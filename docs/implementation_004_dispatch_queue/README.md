# Implementation 004 — Dispatch work queue

Guides for a **Redis-backed queue** with **PostgreSQL as source of truth**, so new orders **auto-start matching** after submit.

## Documents

| Document | Scope |
|----------|--------|
| [implementation_004_dispatch_queue_redis_postgres.md](./implementation_004_dispatch_queue_redis_postgres.md) | Architecture, **transactional outbox**, Redis **LIST vs STREAM**, worker semantics, **idempotency** with current `advance_dispatch_round`, reconciliation, verification — **§12 step-by-step** |

## Entry index

[../implementation_004_dispatch_queue.md](../implementation_004_dispatch_queue.md)
