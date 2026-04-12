-- Implementation 003 §6.3: dispatch / approval / location / aggregates on plumber_profiles.

ALTER TABLE plumber_profiles RENAME COLUMN years_of_experience TO experience_years;

ALTER TABLE plumber_profiles
    ADD COLUMN bio TEXT NULL,
    ADD COLUMN avatar_url TEXT NULL,
    ADD COLUMN is_approved BOOLEAN NOT NULL DEFAULT false,
    ADD COLUMN approved_at TIMESTAMPTZ NULL,
    ADD COLUMN approved_by UUID NULL REFERENCES users (id) ON DELETE SET NULL,
    ADD COLUMN is_online BOOLEAN NOT NULL DEFAULT false,
    ADD COLUMN is_available BOOLEAN NOT NULL DEFAULT false,
    ADD COLUMN current_city_id UUID NULL REFERENCES cities (id) ON DELETE SET NULL,
    ADD COLUMN current_area_id UUID NULL REFERENCES areas (id) ON DELETE SET NULL,
    ADD COLUMN current_street_id UUID NULL REFERENCES streets (id) ON DELETE SET NULL,
    ADD COLUMN current_lat DOUBLE PRECISION NULL,
    ADD COLUMN current_lng DOUBLE PRECISION NULL,
    ADD COLUMN service_radius_km NUMERIC(8, 3) NOT NULL DEFAULT 5,
    ADD COLUMN last_location_updated_at TIMESTAMPTZ NULL,
    ADD COLUMN rating_avg NUMERIC(4, 3) NOT NULL DEFAULT 0,
    ADD COLUMN rating_count INTEGER NOT NULL DEFAULT 0,
    ADD COLUMN completed_orders_count INTEGER NOT NULL DEFAULT 0,
    ADD COLUMN cancelled_orders_count INTEGER NOT NULL DEFAULT 0,
    ADD COLUMN created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    ADD COLUMN updated_at TIMESTAMPTZ NOT NULL DEFAULT now();

ALTER TABLE plumber_profiles ADD CONSTRAINT plumber_profiles_service_radius_km_pos
    CHECK (service_radius_km > 0);

ALTER TABLE plumber_profiles ADD CONSTRAINT plumber_profiles_rating_avg_range
    CHECK (rating_avg >= 0 AND rating_avg <= 5);

CREATE INDEX idx_plumber_profiles_dispatch_ready_partial ON plumber_profiles (is_approved, is_online, is_available)
    WHERE is_approved = true AND is_online = true AND is_available = true;

CREATE INDEX idx_plumber_profiles_current_city_id ON plumber_profiles (current_city_id);
CREATE INDEX idx_plumber_profiles_current_area_id ON plumber_profiles (current_area_id);

CREATE INDEX idx_plumber_profiles_last_location_updated_at ON plumber_profiles (last_location_updated_at DESC);

CREATE INDEX idx_plumber_profiles_dispatch_city ON plumber_profiles (is_approved, is_online, is_available, current_city_id)
    WHERE is_approved = true AND is_online = true AND is_available = true;
