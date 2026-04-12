-- Implementation 003 §3–§6.1: user_status replaces is_active; audit timestamps; admin indexes.
-- Path A (single migration): backfill user_status, drop is_active.

ALTER TABLE users ADD COLUMN user_status user_status NOT NULL DEFAULT 'active';

UPDATE users
SET user_status = CASE
    WHEN is_active THEN 'active'::user_status
    ELSE 'blocked'::user_status
END;

ALTER TABLE users DROP COLUMN is_active;

ALTER TABLE users ADD COLUMN last_login_at TIMESTAMPTZ NULL;
ALTER TABLE users ADD COLUMN blocked_at TIMESTAMPTZ NULL;
ALTER TABLE users ADD COLUMN deleted_at TIMESTAMPTZ NULL;

UPDATE users SET blocked_at = NOW() WHERE user_status = 'blocked';

CREATE INDEX idx_users_role_status_created ON users (role, user_status, created_at DESC);
CREATE INDEX idx_users_status_created ON users (user_status, created_at DESC);
