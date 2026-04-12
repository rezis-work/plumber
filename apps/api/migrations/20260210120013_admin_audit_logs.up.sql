-- Implementation 003 §6.15: admin_audit_logs (Phase 2 Step 11).
-- admin_id NULL = system / batch actions; ON DELETE SET NULL keeps history if admin user removed.

CREATE TABLE admin_audit_logs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    admin_id UUID NULL REFERENCES users (id) ON DELETE SET NULL,
    action TEXT NOT NULL,
    entity_type TEXT NOT NULL,
    entity_id TEXT NOT NULL,
    meta JSONB NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_admin_audit_logs_admin_created_at ON admin_audit_logs (admin_id, created_at DESC);
CREATE INDEX idx_admin_audit_logs_entity ON admin_audit_logs (entity_type, entity_id);
CREATE INDEX idx_admin_audit_logs_created_at ON admin_audit_logs (created_at DESC);
