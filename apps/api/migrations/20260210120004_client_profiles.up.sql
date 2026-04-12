-- Implementation 003 §6.2: client_profiles (1:1 with users; role = client enforced in app).

CREATE TABLE client_profiles (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL UNIQUE REFERENCES users (id) ON DELETE CASCADE,
    full_name TEXT NOT NULL,
    phone TEXT NOT NULL,
    default_city_id UUID NULL REFERENCES cities (id) ON DELETE SET NULL,
    default_area_id UUID NULL REFERENCES areas (id) ON DELETE SET NULL,
    default_street_id UUID NULL REFERENCES streets (id) ON DELETE SET NULL,
    address_line TEXT NULL,
    lat DOUBLE PRECISION NULL,
    lng DOUBLE PRECISION NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_client_profiles_default_city_id ON client_profiles (default_city_id);
