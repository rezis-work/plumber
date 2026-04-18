-- Implementation 004 §12.1 / §3.2 — dispatch_outbox (transactional outbox for dispatch queue).
-- See docs/implementation_004_dispatch_queue/implementation_004_dispatch_queue_redis_postgres.md

CREATE TYPE dispatch_outbox_job_kind AS ENUM (
    'bootstrap_first_round'
);

CREATE TYPE dispatch_outbox_status AS ENUM (
    'pending',
    'processing',
    'done',
    'failed'
);

CREATE TABLE dispatch_outbox (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    order_id UUID NOT NULL REFERENCES orders (id) ON DELETE CASCADE,
    job_kind dispatch_outbox_job_kind NOT NULL,
    status dispatch_outbox_status NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    claimed_at TIMESTAMPTZ NULL,
    lease_expires_at TIMESTAMPTZ NULL,
    processed_at TIMESTAMPTZ NULL,
    attempt_count INTEGER NOT NULL DEFAULT 0,
    last_error TEXT NULL,
    CONSTRAINT dispatch_outbox_attempt_count_non_negative CHECK (attempt_count >= 0)
);

CREATE INDEX idx_dispatch_outbox_status_created_at
    ON dispatch_outbox (status, created_at);

CREATE UNIQUE INDEX dispatch_outbox_order_job_pending_unique
    ON dispatch_outbox (order_id, job_kind)
    WHERE status = 'pending';
