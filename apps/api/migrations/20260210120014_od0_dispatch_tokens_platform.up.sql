-- OD-0: order media, dispatch offer rounds, plumber token ledger, platform_settings.
-- See docs/implementation_003_domain_modeling/implementation_003_orders_dispatch_tokens_redis.md §2.

CREATE TYPE token_ledger_reason AS ENUM (
    'order_completed',
    'speed_bonus',
    'admin_adjustment'
);

ALTER TABLE order_dispatches
    ADD COLUMN offer_round SMALLINT NOT NULL DEFAULT 1;

ALTER TABLE order_dispatches
    ADD CONSTRAINT order_dispatches_offer_round_positive CHECK (offer_round >= 1);

CREATE INDEX idx_order_dispatches_order_id_offer_round ON order_dispatches (order_id, offer_round);

CREATE TABLE order_media (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    order_id UUID NOT NULL REFERENCES orders (id) ON DELETE CASCADE,
    storage_key TEXT NOT NULL,
    content_type TEXT NOT NULL,
    byte_size INTEGER NOT NULL,
    sort_order SMALLINT NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT order_media_byte_size_non_negative CHECK (byte_size >= 0)
);

CREATE INDEX idx_order_media_order_id_sort_order ON order_media (order_id, sort_order);

CREATE TABLE plumber_token_ledger (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    plumber_id UUID NOT NULL REFERENCES plumber_profiles (id) ON DELETE CASCADE,
    delta INTEGER NOT NULL,
    reason token_ledger_reason NOT NULL,
    order_id UUID NULL REFERENCES orders (id) ON DELETE SET NULL,
    idempotency_key TEXT NULL,
    meta JSONB NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT plumber_token_ledger_idempotency_key_unique UNIQUE (idempotency_key)
);

CREATE INDEX idx_plumber_token_ledger_plumber_created_at
    ON plumber_token_ledger (plumber_id, created_at DESC);

CREATE INDEX idx_plumber_token_ledger_order_id_partial
    ON plumber_token_ledger (order_id)
    WHERE order_id IS NOT NULL;

-- token_balance: NOT NULL DEFAULT 0 backfills existing plumber_profiles rows.
ALTER TABLE plumber_profiles
    ADD COLUMN token_balance INTEGER NOT NULL DEFAULT 0;

ALTER TABLE plumber_profiles
    ADD CONSTRAINT plumber_profiles_token_balance_non_negative CHECK (token_balance >= 0);

CREATE TABLE platform_settings (
    key TEXT PRIMARY KEY,
    value JSONB NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

INSERT INTO platform_settings (key, value)
VALUES
    ('dispatch_offer_ttl_minutes_normal', '30'::jsonb),
    ('dispatch_offer_ttl_minutes_emergency', '10'::jsonb),
    ('dispatch_batch_size', '3'::jsonb),
    ('emergency_min_token_balance', '10'::jsonb),
    ('speed_bonus_window_minutes', '30'::jsonb);
