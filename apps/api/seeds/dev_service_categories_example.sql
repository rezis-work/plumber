-- Dev-only service category scaffold (Implementation 003 §11). Run manually after migrations.
-- Idempotent: ON CONFLICT (slug) DO NOTHING.

INSERT INTO service_categories (name, slug, description, sort_order, is_active)
VALUES
    ('Leak repair', 'leak-repair', 'Pipes, fittings, and visible leaks', 10, true),
    ('Drain cleaning', 'drain-cleaning', 'Clogs and drain line clearing', 20, true),
    ('Water heater', 'water-heater', 'Installation and repair', 30, true),
    ('Emergency callout', 'emergency-callout', 'Urgent plumbing response', 40, true)
ON CONFLICT (slug) DO NOTHING;
