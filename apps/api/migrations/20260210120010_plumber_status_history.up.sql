-- Implementation 003 §6.11: plumber_status_history (Phase 2 Step 7).
-- Enum plumber_status_type: 20260210120000_phase2_domain_enums.

CREATE TABLE plumber_status_history (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    plumber_id UUID NOT NULL REFERENCES plumber_profiles (id) ON DELETE CASCADE,
    status_type plumber_status_type NOT NULL,
    meta JSONB NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_plumber_status_history_plumber_created_at
    ON plumber_status_history (plumber_id, created_at DESC);

CREATE INDEX idx_plumber_status_history_status_created_at
    ON plumber_status_history (status_type, created_at DESC);
