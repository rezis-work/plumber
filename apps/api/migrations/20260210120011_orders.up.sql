-- Implementation 003 §6.12: orders (Phase 2 Step 8).
-- Enums order_urgency, order_status: 20260210120000_phase2_domain_enums.

CREATE TABLE orders (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    client_id UUID NOT NULL REFERENCES users (id) ON DELETE RESTRICT,
    assigned_plumber_id UUID NULL REFERENCES plumber_profiles (id) ON DELETE SET NULL,
    service_category_id UUID NOT NULL REFERENCES service_categories (id) ON DELETE RESTRICT,
    city_id UUID NOT NULL REFERENCES cities (id) ON DELETE RESTRICT,
    area_id UUID NULL REFERENCES areas (id) ON DELETE SET NULL,
    street_id UUID NULL REFERENCES streets (id) ON DELETE SET NULL,
    address_line TEXT NOT NULL,
    lat DOUBLE PRECISION NOT NULL,
    lng DOUBLE PRECISION NOT NULL,
    description TEXT NULL,
    urgency order_urgency NOT NULL DEFAULT 'normal',
    status order_status NOT NULL DEFAULT 'searching',
    estimated_price_min NUMERIC(12,2) NULL,
    estimated_price_max NUMERIC(12,2) NULL,
    final_price NUMERIC(12,2) NULL,
    requested_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    accepted_at TIMESTAMPTZ NULL,
    started_at TIMESTAMPTZ NULL,
    completed_at TIMESTAMPTZ NULL,
    cancelled_at TIMESTAMPTZ NULL,
    cancel_reason TEXT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT orders_estimated_price_min_lte_max CHECK (
        estimated_price_min IS NULL
        OR estimated_price_max IS NULL
        OR estimated_price_min <= estimated_price_max
    )
);

CREATE INDEX idx_orders_status_requested_at ON orders (status, requested_at DESC);
CREATE INDEX idx_orders_client_requested_at ON orders (client_id, requested_at DESC);
CREATE INDEX idx_orders_assigned_plumber_id ON orders (assigned_plumber_id)
    WHERE assigned_plumber_id IS NOT NULL;
CREATE INDEX idx_orders_service_category_id ON orders (service_category_id);
CREATE INDEX idx_orders_city_id ON orders (city_id);
CREATE INDEX idx_orders_area_id ON orders (area_id);
CREATE INDEX idx_orders_requested_at ON orders (requested_at DESC);
CREATE INDEX idx_orders_dispatch_queue ON orders (requested_at DESC)
    WHERE status IN ('searching'::order_status, 'dispatched'::order_status);
