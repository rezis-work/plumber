ALTER TABLE users
    ADD COLUMN is_email_verified BOOLEAN NOT NULL DEFAULT false,
    ADD COLUMN email_verification_token_hash TEXT NULL,
    ADD COLUMN email_verification_expires_at TIMESTAMPTZ NULL;

CREATE TABLE plumber_profiles (
    user_id UUID PRIMARY KEY REFERENCES users (id) ON DELETE CASCADE,
    full_name TEXT NOT NULL,
    phone TEXT NOT NULL,
    years_of_experience INTEGER NOT NULL CHECK (years_of_experience >= 0)
);
