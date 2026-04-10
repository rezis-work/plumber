DROP TABLE IF EXISTS plumber_profiles;

ALTER TABLE users
    DROP COLUMN IF EXISTS email_verification_expires_at,
    DROP COLUMN IF EXISTS email_verification_token_hash,
    DROP COLUMN IF EXISTS is_email_verified;
