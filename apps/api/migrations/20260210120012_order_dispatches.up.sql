-- Implementation 003 §6.13: order_dispatches (Phase 2 Step 9).
-- Enum dispatch_status: 20260210120000_phase2_domain_enums.

CREATE TABLE order_dispatches (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    order_id UUID NOT NULL REFERENCES orders (id) ON DELETE CASCADE,
    plumber_id UUID NOT NULL REFERENCES plumber_profiles (id) ON DELETE CASCADE,
    dispatch_rank SMALLINT NOT NULL,
    status dispatch_status NOT NULL,
    sent_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    responded_at TIMESTAMPTZ NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT order_dispatches_order_plumber_unique UNIQUE (order_id, plumber_id),
    CONSTRAINT order_dispatches_dispatch_rank_positive CHECK (dispatch_rank >= 1)
);

CREATE INDEX idx_order_dispatches_order_id ON order_dispatches (order_id);
CREATE INDEX idx_order_dispatches_plumber_id ON order_dispatches (plumber_id);
CREATE INDEX idx_order_dispatches_status_sent_at ON order_dispatches (status, sent_at DESC);
CREATE INDEX idx_order_dispatches_plumber_status_sent_at ON order_dispatches (plumber_id, status, sent_at DESC);
