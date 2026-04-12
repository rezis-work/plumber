-- Implementation 003 §6.10: plumber_service_areas (Phase 2 Step 7).

CREATE TABLE plumber_service_areas (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    plumber_id UUID NOT NULL REFERENCES plumber_profiles (id) ON DELETE CASCADE,
    city_id UUID NOT NULL REFERENCES cities (id) ON DELETE CASCADE,
    area_id UUID NULL REFERENCES areas (id) ON DELETE CASCADE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE UNIQUE INDEX plumber_service_areas_plumber_city_whole_unique ON plumber_service_areas (plumber_id, city_id)
    WHERE area_id IS NULL;

CREATE UNIQUE INDEX plumber_service_areas_plumber_city_area_unique ON plumber_service_areas (plumber_id, city_id, area_id)
    WHERE area_id IS NOT NULL;

CREATE INDEX idx_plumber_service_areas_plumber_id ON plumber_service_areas (plumber_id);
CREATE INDEX idx_plumber_service_areas_city_area ON plumber_service_areas (city_id, area_id);
