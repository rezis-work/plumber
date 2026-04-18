# Implementation 004 — §12.0 Preconditions sign-off

**Date:** 2026-04-18  
**Scope:** Readiness audit only (main guide **§12.0 Preconditions**). No `dispatch_outbox` migration or queue code in this sign-off.

| # | Precondition | Result | Evidence |
|---|--------------|--------|----------|
| 1 | Core `orders`, `order_media`, `order_dispatches`, `advance_dispatch_round`, `/internal/dispatch/*` | **Pass** | Migrations: [`20260210120011_orders.up.sql`](../../apps/api/migrations/20260210120011_orders.up.sql), [`20260210120012_order_dispatches.up.sql`](../../apps/api/migrations/20260210120012_order_dispatches.up.sql), [`20260210120014_od0_dispatch_tokens_platform.up.sql`](../../apps/api/migrations/20260210120014_od0_dispatch_tokens_platform.up.sql) (`order_media`, `platform_settings`). Code: [`advance_dispatch_round`](../../apps/api/src/modules/dispatch_writer/service.rs), routes in [`main.rs`](../../apps/api/src/main.rs) + [`dispatch_writer/mod.rs`](../../apps/api/src/modules/dispatch_writer/mod.rs). **Tests:** `cargo test dispatch_writer` — **8 passed** (advance + expire job against real DB from `.env`). |
| 2 | `MatcherConfig` / `platform_settings` readable where worker will run | **Pass (with note)** | [`MatcherConfig::default()`](../../apps/api/src/modules/dispatch_matcher/config.rs) matches seeded `platform_settings` numerically. Internal HTTP handlers use **`MatcherConfig::default()`** today ([`handler.rs`](../../apps/api/src/modules/dispatch_writer/handler.rs)), not a DB load per request. **§12.1+:** optional hardening = load TTL/batch from `platform_settings` once at worker startup or cache. |
| 3 | Redis client available to API process | **Pass (optional)** | [`RedisDispatchHelper::from_env()`](../../apps/api/src/modules/dispatch_writer/redis.rs) — `UPSTASH_REDIS_REST_URL` + `UPSTASH_REDIS_REST_TOKEN`; **`None`** if unset; dispatch still works DB-only. **Forward-looking (§12.3/12.5):** LIST queue needs **`RPUSH`/`BRPOP`**; current helper is REST for locks/deadlines — extend or add a small Redis command helper for the queue key. |

## HTTP smoke (local dev, debug secret)

Server: `cargo run --bin api` with `.env` (`DATABASE_URL`). Debug build: `X-Internal-Secret: your-local-dev-secret` when `DISPATCH_INTERNAL_SECRET` unset ([`main.rs`](../../apps/api/src/main.rs)).

| Request | HTTP | Body (summary) |
|---------|------|------------------|
| `GET /health` | 200 | `ok` |
| `POST /internal/dispatch/expire-due` + secret | 200 | `expired_count`, `rounds_checked`, … |
| `POST /internal/dispatch/advance` + secret + unknown `order_id` | 404 | `outcome: skipped_order_not_found` (expected per [`AdvanceResponse`](../../apps/api/src/modules/dispatch_writer/handler.rs)) |

## Verdict

**§12.0 complete — safe to proceed to §12.1** (`dispatch_outbox` migration) unless product blocks on “load `MatcherConfig` from DB before any worker” (not required by §12.0 text).
