-- Implementation 003 §6.8: service_price_guides (Phase 2 Step 3, second half).

CREATE TABLE service_price_guides (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    service_category_id UUID NOT NULL REFERENCES service_categories (id) ON DELETE CASCADE,
    city_id UUID NULL REFERENCES cities (id) ON DELETE CASCADE,
    area_id UUID NULL REFERENCES areas (id) ON DELETE CASCADE,
    min_price NUMERIC(12,2) NOT NULL,
    max_price NUMERIC(12,2) NOT NULL,
    currency CHAR(3) NOT NULL,
    estimated_duration_minutes INTEGER NOT NULL,
    is_emergency_supported BOOLEAN NOT NULL DEFAULT false,
    notes TEXT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT service_price_guides_min_lte_max CHECK (min_price <= max_price),
    CONSTRAINT service_price_guides_duration_positive CHECK (estimated_duration_minutes > 0),
    CONSTRAINT service_price_guides_area_requires_city CHECK (area_id IS NULL OR city_id IS NOT NULL)
);

CREATE INDEX idx_service_price_guides_service_category_id ON service_price_guides (service_category_id);
CREATE INDEX idx_service_price_guides_city_id ON service_price_guides (city_id);
CREATE INDEX idx_service_price_guides_area_id ON service_price_guides (area_id);
CREATE INDEX idx_service_price_guides_category_city_area ON service_price_guides (service_category_id, city_id, area_id);

CREATE UNIQUE INDEX service_price_guides_category_global_unique ON service_price_guides (service_category_id)
    WHERE city_id IS NULL AND area_id IS NULL;

CREATE UNIQUE INDEX service_price_guides_category_city_unique ON service_price_guides (service_category_id, city_id)
    WHERE city_id IS NOT NULL AND area_id IS NULL;

CREATE UNIQUE INDEX service_price_guides_category_city_area_unique ON service_price_guides (service_category_id, city_id, area_id)
    WHERE area_id IS NOT NULL;
