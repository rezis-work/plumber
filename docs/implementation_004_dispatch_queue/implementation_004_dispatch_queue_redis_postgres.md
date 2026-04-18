# Implementation 004 — Dispatch work queue (Redis) with PostgreSQL as source of truth

**Purpose:** Specify how to **enqueue dispatch work** when a client creates an order so matching **starts automatically**—decoupled from `POST /orders` latency—while keeping **PostgreSQL** the **system of record** for orders, dispatches, and job durability.

**Prerequisites:**

- [implementation_003_domain_modeling.md](../implementation_003_domain_modeling.md) (schema).
- [implementation_003_orders_dispatch_tokens_redis.md](../implementation_003_domain_modeling/implementation_003_orders_dispatch_tokens_redis.md) — dispatch rounds, `advance_dispatch_round`, Redis lock/deadline helpers, `expire-due` cron.

**Non-goals (this doc):** WebSocket/SSE (see product “Phase 7” real-time); push notifications; matching v2 ML.

**Hands-on order:** Implement in the sequence in **[§12 — Step-by-step implementation guide](#12-step-by-step-implementation-guide)**; sections **§1–§11** are background and contracts.

---

## 1. Problem

Today, **creating an order** can finish while **first-round plumbers** are not yet assigned unless something calls **`advance_dispatch_round`** (internal advance route, reject/expire paths, etc.). For an **instant dispatch** product, the desired behavior is:

1. Client **`POST /orders`** → row in **`searching`** (and media) committed in PostgreSQL.
2. **Soon after**, the system runs the matcher and inserts **`order_dispatches`** for **round 1** without manual operator steps.

**Constraints:**

- **`POST /orders`** should stay **fast** and reliable (avoid long matcher work inline if it hurts UX or timeouts).
- **At-least-once** delivery from any queue means workers may see **duplicates**; processing must be **safe** against double execution.
- If **Redis** is empty or unavailable, work must still be **recoverable** from PostgreSQL.

---

## 2. Design principle: PostgreSQL owns truth

| Concern | PostgreSQL | Redis |
|---------|------------|--------|
| Order exists, status, geography, urgency | **Yes** | No |
| `order_dispatches` rows | **Yes** | No |
| “Should we run first-round advance for this order?” | **Yes** — outbox row or derivable state | Optional signal only |
| In-flight job visibility, wake-ups, burst buffering | Optional | **Yes** (LIST/STREAM) |
| Distributed lock during advance | Already optional in code | **Yes** — `order:dispatch:lock:{order_id}` (see 003) |

**Rule:** A message in Redis **must never** be the only proof that work exists. After a successful **`COMMIT`** on the order, either:

- **A)** insert a **durable job row** in PostgreSQL (recommended), *then* push to Redis for latency, **or**
- **B)** rely on a **reconciliation** job that scans PostgreSQL for “orphan” `searching` orders with **no** dispatches and enqueues them (Redis-only queue needs this safety net).

---

## 3. Recommended pattern: transactional outbox + Redis signal

### 3.1 Why outbox

- **Atomicity:** Order + job row commit **together**—no lost jobs if Redis `RPUSH` fails after commit (worker or reconciler still sees the row).
- **Auditing:** `attempt_count`, `last_error`, timestamps live in SQL.
- **Backpressure:** Workers drain Redis; DB shows backlog.

### 3.2 Minimal table (example name: `dispatch_outbox`)

Define in a **new migration** (names are illustrative—align with repo conventions):

| Column | Type | Notes |
|--------|------|--------|
| `id` | UUID PK | `gen_random_uuid()` |
| `order_id` | UUID NOT NULL | → `orders.id` **ON DELETE CASCADE** |
| `job_kind` | TEXT or ENUM NOT NULL | e.g. `bootstrap_first_round` |
| `status` | TEXT or ENUM NOT NULL | `pending` → `processing` → `done` / `failed` |
| `created_at` | TIMESTAMPTZ NOT NULL | default `now()` |
| `claimed_at` | TIMESTAMPTZ NULL | when a worker starts |
| `lease_expires_at` | TIMESTAMPTZ NULL | reclaim stuck rows |
| `processed_at` | TIMESTAMPTZ NULL | when terminal success |
| `attempt_count` | INTEGER NOT NULL DEFAULT 0 | |
| `last_error` | TEXT NULL | truncated stack or message |

**Constraints / indexes:**

- **UNIQUE (`order_id`, `job_kind`)** where `status = 'pending'` — partial unique index in PostgreSQL, *or* application rule: insert outbox only once per order for `bootstrap_first_round`.
- Index **`(status, created_at)`** for workers: `WHERE status = 'pending' ORDER BY created_at`.

### 3.3 API transaction flow (`create_order`)

1. Begin transaction.
2. Insert **`orders`** (+ **`order_media`** as today).
3. Insert **`dispatch_outbox`** row: `job_kind = bootstrap_first_round`, `status = pending`.
4. **Commit.**
5. **After** successful commit (same process): **`RPUSH dispatch:queue <order_id>`** (or `XADD` to a STREAM — see §5).

If step 5 fails, **no data loss**: row stays `pending`; **reconciler** or **worker polling PG** picks it up.

---

## 4. Worker: consume Redis, confirm in PostgreSQL

### 4.1 Bootstrap job semantics

The worker’s goal for **`bootstrap_first_round`** is: ensure **round 1** dispatches exist **once** when the order is still dispatchable and has **no** dispatches yet.

**Critical idempotency note (current `advance_dispatch_round` behavior):**  
In [`apps/api`](../../apps/api/), `advance_dispatch_round` computes **`next_round = max(offer_round) + 1`**. If **round 1 already exists**, a **duplicate** queue message would incorrectly attempt **round 2** immediately. Therefore the bootstrap worker **must** guard, for example:

- `SELECT COUNT(*) FROM order_dispatches WHERE order_id = $1` **== 0** (or `COALESCE(MAX(offer_round), 0) = 0` per your empty semantics), **then** call `advance_dispatch_round`, **or**
- Use a **single-row** outbox: transition `pending` → `processing` with **`FOR UPDATE SKIP LOCKED`**, and mark **`done`** only when `advance_dispatch_round` returns **`Success`** with `offer_round == 1` or when dispatches already exist (treat as success).

**Round 2+** should continue to be driven by **`expire-due`**, **reject path**, or explicit internal advance—**not** by repeating `bootstrap_first_round` unless you add a separate job kind.

### 4.2 Suggested worker loop

1. **BRPOP** `dispatch:queue` (blocking) **or** poll outbox `WHERE status = 'pending'` if Redis empty.
2. Load **`orders`** row; if not `searching`/`dispatched` or cancelled, mark outbox **`done`** (no-op) and ACK.
3. Acquire **DB row lock** on outbox (or order) — **`FOR UPDATE SKIP LOCKED`**.
4. Re-check “no dispatches yet” if bootstrap.
5. Call **`advance_dispatch_round`** (same pool, pass optional **`RedisDispatchHelper`** for lock + deadline keys per 003). The matcher inside that path must apply **§4.3** (strict match, then city-wide fallback) so a sparse area still gets offers when anyone exists in the city.
6. On success: outbox **`done`**, `processed_at = now()`.
7. On expected skip (`SkippedNoPlumbers`, `SkippedNotDispatchable`): only after **both** strict and city-wide attempts (§4.3)—then product policy: typically mark outbox **`done`** and let existing order lifecycle mark **`expired`** / alert ops if **zero** plumbers exist in the whole city.
8. On transient error: increment **`attempt_count`**, set **`lease_expires_at`**, leave **`pending`** for retry with backoff.

### 4.3 City-wide fallback when strict match finds no one

**Product rule:** While the order is **searching** for plumbers, run the **normal strict matcher** first (service category, distance / radius, area/street where applicable, online/available/approved, location staleness, exclusions—see [Implementation 003 §5](../implementation_003_domain_modeling/implementation_003_orders_dispatch_tokens_redis.md)). If that query returns **no eligible plumbers** (not even one), **extend** matching in the **same dispatch attempt** (same intended **`offer_round`**, usually round 1) to include **any eligible plumber tied to the order’s city** (`orders.city_id`).

**Suggested “city-wide” criteria (v1):**

- Still require **`plumber_profiles.is_approved`**, **`is_online`**, **`is_available`**, and **`plumber_services`** containing the order’s **`service_category_id`** (you usually still want someone who can do the job).
- **Geography:** restrict to plumbers **working in or located in** the order’s **`city_id`**—e.g. `plumber_service_areas` includes that city, and/or `current_city_id` matches `orders.city_id` (pick one explicit rule in SQL and document it; avoid returning plumbers with no city relationship).
- **Relax:** distance / `service_radius_km` check, fine-grained **area/street** filter, and (unless product forbids) other “nice to have” filters that caused the empty strict set.
- **Emergency orders:** keep **`token_balance ≥ emergency_min_token_balance`** on the fallback pass unless stakeholders explicitly allow widening (document the choice).
- **Exclusions unchanged:** plumbers already on **`order_dispatches`** for this **`order_id`** must never reappear.

**Implementation shape:**

- Inside **`advance_dispatch_round`** (or `match_plumbers`), **try strict SQL first**. If it returns at least one plumber, **use only that list**. If it returns **empty**, run the **second query** (city-wide) and use **that** list—do **not** union strict + fallback (avoids duplicates and odd ranking).
- **Ranking:** use the same scoring idea as strict mode where fields exist (tokens, rating); for fallback, distance may be missing or de-emphasized—keep sort **deterministic** (stable tie-break).
- **Telemetry:** log and optionally metric **`match_pass = strict | city_fallback`** and **`candidate_count`** so product can tune filters later.

**Interaction with round 2+:** Fallback applies to **finding candidates for the next `offer_round`** in general: if strict pass is empty for a **later** round, apply the same city-wide pass before returning **`SkippedNoPlumbers`**, so exhausted hyper-local pools still widen within the city before the order is marked **no plumbers**.

---

## 5. Redis data structures

### 5.1 Simple LIST (good for v1)

| Key | Type | Value | Producer | Consumer |
|-----|------|--------|----------|----------|
| `dispatch:queue` | LIST | JSON `{"order_id":"<uuid>","kind":"bootstrap_first_round"}` or plain `order_id` UUID string | **`RPUSH`** after PG commit | **`BRPOP`** worker |

**Pros:** trivial, works on Upstash.  
**Cons:** no built-in consumer groups; one message visible to one client at a time (`BRPOP`).

### 5.2 Redis Stream (scale-out)

| Key | Type | Notes |
|-----|------|--------|
| `dispatch:stream` | STREAM | `XADD` with auto ID; consumer group per service |

**Pros:** pending entries list, explicit ACK, replay.  
**Cons:** slightly more code; still need **PG outbox** or strict reconciliation for durability.

### 5.3 Do not use Redis alone

If you only **`RPUSH`** without a PG row, you **must** run a **reconciliation cron** (see §7) that finds **`searching`** orders with **zero** `order_dispatches` and re-enqueues—otherwise Redis flush or failed push loses work.

---

## 6. Interaction with existing 003 mechanisms

- **`POST /internal/dispatch/advance`:** keep for ops/debug; workers normally use **`advance_dispatch_round`** in-process.
- **`POST /internal/dispatch/expire-due`:** expires offers and advances rounds when appropriate, then runs **§12.6** `reconcile_stale_outbox` (lease reclaim + orphan nudge) at the end of each tick.
- **Redis order lock** inside `advance_dispatch_round:** still useful when **multiple workers** and **cron** can race; queue reduces duplicate *intent*, lock reduces duplicate *commit*.

---

## 7. Reconciliation (safety net)

Scheduled job (e.g. every 1–5 minutes):

```sql
-- Illustrative: orders stuck in searching with no dispatches and a pending outbox older than N minutes
SELECT o.id
FROM orders o
WHERE o.status = 'searching'
  AND NOT EXISTS (SELECT 1 FROM order_dispatches d WHERE d.order_id = o.id)
  AND EXISTS (
    SELECT 1 FROM dispatch_outbox x
    WHERE x.order_id = o.id AND x.job_kind = 'bootstrap_first_round' AND x.status = 'pending'
  );
```

For each row: **`RPUSH dispatch:queue`** again (cheap dedupe at worker), or call **`advance_dispatch_round`** directly from reconciler if you prefer DB-driven only.

Also reclaim **`processing`** outbox rows whose **`lease_expires_at < now()`** back to **`pending`** with incremented **`attempt_count`**.

---

## 8. Observability

- Structured logs: **`order_id`**, **`outbox_id`**, **`job_kind`**, outcome of `advance_dispatch_round`, Redis push failures.
- Metrics: queue depth (Redis `LLEN` or stream lag), outbox **`pending`** count, time from **`orders.requested_at`** to first dispatch insert (already partially instrumented in code for round 1).

---

## 9. Rollout phases

| Phase | Deliverable |
|-------|-------------|
| **DQ-0** | Migration: **`dispatch_outbox`** (or chosen name), indexes |
| **DQ-1** | `create_order`: insert outbox row in same txn; optional **`RPUSH`** after commit |
| **DQ-2** | Worker binary or background task: BRPOP + guard + `advance_dispatch_round` + outbox status updates |
| **DQ-2b** | Matcher: **strict** pass then **city-wide fallback** (§4.3) inside `advance_dispatch_round` / `match_plumbers`; tests for empty strict / non-empty fallback / still-empty city |
| **DQ-3** | Reconciliation + lease reclaim |
| **DQ-4** | Load test: duplicate deliveries, Redis down, worker crash mid-flight |

---

## 10. Verification checklist

- [ ] Create order → outbox row **`pending`** appears in same commit as order.
- [ ] Worker processes message → **`order_dispatches`** round 1 rows exist; order moves toward **`dispatched`** per existing rules.
- [ ] **Duplicate** Redis message does **not** create premature **round 2** (guard works).
- [ ] Redis push fails after commit → reconciler or PG-polled worker still completes job.
- [ ] Worker dies after **`processing`** → lease expires → retry succeeds or marks failed with alert.
- [ ] Strict matcher returns 0 plumbers in a small area → **city fallback** returns up to **`dispatch_batch_size`** from the order’s **city**; dispatches created in **round 1** (not skipped as `SkippedNoPlumbers`).
- [ ] Strict matcher returns ≥1 plumber → **city fallback is not used** (no widening when unnecessary).
- [ ] Both strict and city-wide return 0 → `SkippedNoPlumbers`; order/outbox behavior matches product (e.g. expire, ops alert).

---

## 11. Relation to other specs

- **Implementation 003** — schema and dispatch/token **behavior** this queue **drives**; when §4.3 is implemented, add a short cross-reference in [Implementation 003 §5](../implementation_003_domain_modeling/implementation_003_orders_dispatch_tokens_redis.md) so the “hard filters” narrative stays the single matcher spec.
- **Future (notifications / real-time)** — queue events can later **fan out** to WebSocket/SSE or push providers; keep payloads **order_id-centric** and read fresh state from PostgreSQL before emitting.

---

## 12. Step-by-step implementation guide

Follow these steps **in order** unless noted. Repo paths assume [`apps/api`](../../apps/api/) (**Rust**, **sqlx**, **Axum**).

### 12.0 Preconditions

- [ ] Core **`orders`**, **`order_media`**, **`order_dispatches`**, **`advance_dispatch_round`**, and internal **`/internal/dispatch/*`** routes exist (Implementation 003).
- [ ] **`MatcherConfig`** / `platform_settings` (or env defaults) readable where the worker will run.
- [ ] Redis client available to the API process (same as existing `RedisDispatchHelper` if you already use it for dispatch locks/deadlines).

### 12.1 Migration — `dispatch_outbox` (DQ-0)

1. Add a new pair under [`apps/api/migrations/`](../../apps/api/migrations/): `…_dispatch_outbox.up.sql` / `.down.sql`.
2. **`CREATE TABLE dispatch_outbox`** with columns from **§3.2** (`id`, `order_id` FK **`ON DELETE CASCADE`**, `job_kind`, `status`, timestamps, `attempt_count`, `last_error`).
3. Prefer **`job_kind`** and **`status`** as PostgreSQL **`ENUM`** types *or* `TEXT` + `CHECK` constraints—match how other enums are done in this repo.
4. Indexes:
   - **`(status, created_at)`** where you query pending work.
   - Optional **partial unique** on **`(order_id, job_kind)` WHERE status = 'pending'`** — if PostgreSQL version/pattern makes this awkward, enforce “one pending bootstrap per order” in application code on insert.
5. Run **`sqlx migrate run`** locally; confirm `sqlx-data.json` / offline query steps if your CI uses **`cargo sqlx prepare`**.

### 12.2 Rust model + repository

1. Add a small module (e.g. **`src/modules/dispatch_outbox/`**) with:
   - Struct **`DispatchOutbox`** (`FromRow`).
   - **`insert_pending_bootstrap_tx`** — called from `create_order`’s open transaction.
   - **`try_claim_next_pending`** — `UPDATE … RETURNING` using **`FOR UPDATE SKIP LOCKED`** (or `SELECT … FOR UPDATE SKIP LOCKED` then update), set **`status = processing`**, **`claimed_at`**, **`lease_expires_at = now() + interval '…'`**, increment **`attempt_count`** as appropriate.
   - **`mark_done`**, **`mark_failed`**, **`requeue_expired_leases`** (back to **`pending`** for §7).
2. Map DB errors; keep functions **`async`** with **`&mut Transaction`** where they participate in the create-order txn.

### 12.3 Wire `create_order` (DQ-1)

1. In **`orders::service::create_order`** (or equivalent), **inside the same transaction** as order + media insert, call **`insert_pending_bootstrap_tx`** for **`job_kind = bootstrap_first_round`**.
2. **`COMMIT`** the transaction.
3. **After** successful commit, **`RPUSH`** to **`dispatch:queue`** (value: **`order_id`** UUID string or small JSON per **§5.1**). Wrap in **`tracing::warn!`** on failure; **do not** fail the HTTP response if Redis is down—the outbox row remains **`pending`**.
4. Add integration test: create order → row in **`dispatch_outbox`** with **`pending`**; optional mock Redis assert **`RPUSH`** called once.

### 12.4 Matcher — strict then city fallback (DQ-2b)

1. Locate **`match_plumbers`** (e.g. **`dispatch_matcher/query.rs`**) and refactor into:
   - **`match_plumbers_strict`** — current behavior unchanged.
   - **`match_plumbers_city_fallback`** — **§4.3** rules; same batch cap and exclusion of **`order_dispatches`** for this order.
2. Orchestrator **`match_plumbers`**: run strict; if empty, run city fallback; return one list + metadata **`MatchPass { Strict, CityFallback }`** for logging/metrics.
3. Add unit/integration tests:
   - Strict returns plumbers → fallback query **not** executed (spy on call count or separate test DB fixtures).
   - Strict empty, city has plumbers → non-empty list, all tied to **`orders.city_id`** per your SQL rule.
   - Both empty → empty list.
4. Update [Implementation 003 §5](../implementation_003_domain_modeling/implementation_003_orders_dispatch_tokens_redis.md) with one paragraph + link back to **§4.3** here.

### 12.5 Dispatch worker (DQ-2)

Choose **one** deployment shape first; you can split binaries later.

**Option A — In-process loop (fastest to ship)**  
1. On API startup (behind env e.g. **`DISPATCH_QUEUE_WORKER_ENABLED=true`**), spawn a **`tokio::task`** that runs the consumer loop.  
2. Loop:
   - **`BRPOP dispatch:queue`** with timeout (e.g. 5 s) **or**, if Redis unavailable, **`sleep`** + **`try_claim_next_pending`** on PostgreSQL only.  
   - **Implementation note:** this repo uses **Upstash Redis REST**, which does not support blocking **`BRPOP`** cleanly over HTTP; the API worker uses **`LPOP dispatch:queue`** plus a **~5 s idle sleep** when the list is empty (same wake cadence, non-blocking requests).
   - For each **`order_id`**, load matching **`dispatch_outbox`** row(s); **claim** with **`SKIP LOCKED`**.
   - **Guard:** `COUNT(*) FROM order_dispatches WHERE order_id = ?` **== 0** (bootstrap) before calling **`advance_dispatch_round`**; if already has dispatches, mark outbox **`done`** (idempotent duplicate delivery).
   - Call **`advance_dispatch_round(pool, order_id, &config, redis_helper)`** — same as internal advance.
   - Map **`AdvanceDispatchOutcome`**: **`Success`** → **`mark_done`**; **`SkippedNoPlumbers`** / **`SkippedNotDispatchable`** after matcher tries both passes → **`mark_done`** or **`failed`** per product; **`SkippedLockNotAcquired`** → requeue / short sleep retry without burning attempts indefinitely.
3. Graceful shutdown: listen for shutdown signal and exit loop.

**Option B — Separate worker binary**  
1. New crate or `src/bin/dispatch_worker.rs` with same loop, shared library code from **`dispatch_writer`** / **`dispatch_matcher`**.  
2. Deploy as a second process; configure same **`DATABASE_URL`** and Redis.

### 12.6 Reconciliation + leases (DQ-3)

1. Implement a function **`reconcile_stale_outbox`** (callable from **`POST /internal/dispatch/reconcile-outbox`** or reuse **`expire-due`** cron with a second step):
   - **`requeue_expired_leases`** — **`processing`** where **`lease_expires_at < now()`** → **`pending`**, increment **`attempt_count`**, set **`last_error`** if useful.
   - **Orphan orders** — query like **§7**; for each **`order_id`**, **`RPUSH dispatch:queue`** (idempotent at worker) **or** call **`advance_dispatch_round`** directly if you want DB-only recovery.
2. Schedule in production (cron hitting your internal route with **`X-Internal-Secret`**, or external scheduler).
3. Test: force **`lease_expires_at`** in the past → row becomes **`pending`** again and is processed.

**v1 wiring:** The API implements **`reconcile_stale_outbox`** in the dispatch writer crate, exposes **`POST /internal/dispatch/reconcile-outbox`** (same auth as **`expire-due`**), and runs the same reconcile step automatically at the **end** of each **`POST /internal/dispatch/expire-due`** tick so a single cron can cover OD-5 expiry plus DQ-3 lease reclaim and orphan nudges.

### 12.7 Configuration and limits

| Setting | Purpose |
|---------|---------|
| `DISPATCH_OUTBOX_LEASE_SECS` | Lease seconds when a worker claims a row (**`processing`** until reclaim); default **120**. Legacy alias: **`DISPATCH_WORKER_LEASE_SECS`** if unset. |
| `DISPATCH_OUTBOX_MAX_ATTEMPTS` | After **N** lease-expired reclaim increments (`attempt_count`), row → **`failed`** (`max_attempts_exceeded`). **`0`** or unset = no cap. |
| `DISPATCH_QUEUE_REDIS_KEY` | Redis LIST name for **`RPUSH`/`LPOP`** (default **`dispatch:queue`**) |
| `DISPATCH_QUEUE_WORKER_CONCURRENCY` | In-process worker task count (**1..8**, default **1**); requires **`DISPATCH_QUEUE_WORKER_ENABLED`**. For scale-out beyond LIST semantics see **§5.2** (Streams). |

Defaults and comments: [`apps/api/.env.example`](../../apps/api/.env.example) (Dispatch writer section).

### 12.8 Observability (ties to §8)

1. Emit structured fields: **`order_id`**, **`outbox.id`**, **`match_pass`**, **`AdvanceDispatchOutcome`**, Redis errors.
2. **Implemented metrics** (Prometheus): **`dispatch_queue_rpush_failures_total`** (counter on create-order **`RPUSH dispatch:queue`** failure after DB commit), **`dispatch_outbox_pending`** (gauge: `COUNT(*)` of **`dispatch_outbox`** rows with **`status = 'pending'`**, refreshed at the end of **`reconcile_stale_outbox`**, including the step run after **`expire-due`**).
3. **Structured logs:** `advance_dispatch_round` emits **`tracing::info!(target = "dispatch", …)`** lines with message **`dispatch_advance_outcome`** and stable fields **`outcome`** / **`match_pass`** (`none` before matcher, **`strict`** / **`city_fallback`** after matcher). The dispatch worker emits **`dispatch_worker_job`** with **`outbox_id`**, **`job_kind`**, and the same **`outcome`** strings where applicable. Filter dispatch paths with **`target = "dispatch"`**.

### 12.9 Final verification

Run through **§10** manually or automated; add **`sqlx` tests** or HTTP tests for happy path + duplicate message + Redis down + lease reclaim.

### 12.10 Rollback / feature flag

- Ship with **`DISPATCH_QUEUE_WORKER_ENABLED=false`** first: outbox rows accumulate; run **manual** internal advance or enable worker in staging.
- Migration **`down`** drops **`dispatch_outbox`** only when no FK references from other new tables.

---

*When this guide and the running code diverge, update **both** in the same PR; migrations remain the schema contract.*
