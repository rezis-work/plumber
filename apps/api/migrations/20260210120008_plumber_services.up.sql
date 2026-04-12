-- Implementation 003 §6.9: plumber_services (Phase 2 Step 7, first table).

CREATE TABLE plumber_services (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    plumber_id UUID NOT NULL REFERENCES plumber_profiles (id) ON DELETE CASCADE,
    service_category_id UUID NOT NULL REFERENCES service_categories (id) ON DELETE CASCADE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT plumber_services_plumber_category_unique UNIQUE (plumber_id, service_category_id)
);

CREATE INDEX idx_plumber_services_service_category_id ON plumber_services (service_category_id);
CREATE INDEX idx_plumber_services_plumber_id ON plumber_services (plumber_id);
