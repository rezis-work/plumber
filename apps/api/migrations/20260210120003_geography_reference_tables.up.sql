-- Implementation 003 §6.4–§6.6: geography reference tables (Phase 2 Step 2).

CREATE TABLE cities (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name TEXT NOT NULL,
    slug TEXT NOT NULL UNIQUE,
    is_active BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_cities_is_active ON cities (is_active);

CREATE TABLE areas (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    city_id UUID NOT NULL REFERENCES cities (id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    slug TEXT NOT NULL,
    is_active BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    UNIQUE (city_id, slug)
);

CREATE INDEX idx_areas_city_id ON areas (city_id);
CREATE INDEX idx_areas_city_id_is_active ON areas (city_id, is_active);

CREATE TABLE streets (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    city_id UUID NOT NULL REFERENCES cities (id) ON DELETE CASCADE,
    area_id UUID NULL REFERENCES areas (id) ON DELETE SET NULL,
    name TEXT NOT NULL,
    slug TEXT NOT NULL,
    is_active BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE UNIQUE INDEX streets_city_area_slug_unique ON streets (city_id, area_id, slug)
    WHERE area_id IS NOT NULL;

CREATE UNIQUE INDEX streets_city_slug_null_area_unique ON streets (city_id, slug)
    WHERE area_id IS NULL;

CREATE INDEX idx_streets_city_id ON streets (city_id);
CREATE INDEX idx_streets_area_id ON streets (area_id);
CREATE INDEX idx_streets_city_id_area_id ON streets (city_id, area_id);
