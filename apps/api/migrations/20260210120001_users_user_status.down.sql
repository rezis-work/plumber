DROP INDEX IF EXISTS idx_users_status_created;
DROP INDEX IF EXISTS idx_users_role_status_created;

ALTER TABLE users DROP COLUMN IF EXISTS deleted_at;
ALTER TABLE users DROP COLUMN IF EXISTS blocked_at;
ALTER TABLE users DROP COLUMN IF EXISTS last_login_at;

ALTER TABLE users ADD COLUMN is_active BOOLEAN NOT NULL DEFAULT true;

UPDATE users SET is_active = (user_status = 'active');

ALTER TABLE users DROP COLUMN user_status;
