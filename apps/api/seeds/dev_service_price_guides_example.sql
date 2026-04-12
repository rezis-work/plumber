-- Dev-only price guide examples (Implementation 003 §11).
-- Run after geography + service_categories seeds (tbilisi, vake, category slugs).
-- Idempotent: NOT EXISTS on scope + category.

INSERT INTO service_price_guides (
    service_category_id,
    city_id,
    area_id,
    min_price,
    max_price,
    currency,
    estimated_duration_minutes,
    is_emergency_supported,
    notes
)
SELECT
    sc.id,
    c.id,
    NULL,
    80.00,
    150.00,
    'GEL',
    90,
    true,
    'City-wide estimate for Tbilisi'
FROM service_categories sc
CROSS JOIN cities c
WHERE sc.slug = 'leak-repair'
  AND c.slug = 'tbilisi'
  AND NOT EXISTS (
    SELECT 1
    FROM service_price_guides g
    WHERE g.service_category_id = sc.id
      AND g.city_id = c.id
      AND g.area_id IS NULL
  );

INSERT INTO service_price_guides (
    service_category_id,
    city_id,
    area_id,
    min_price,
    max_price,
    currency,
    estimated_duration_minutes,
    is_emergency_supported,
    notes
)
SELECT
    sc.id,
    a.city_id,
    a.id,
    100.00,
    200.00,
    'GEL',
    120,
    true,
    'Vake area band'
FROM service_categories sc
JOIN areas a ON a.slug = 'vake'
JOIN cities c ON c.id = a.city_id AND c.slug = 'tbilisi'
WHERE sc.slug = 'drain-cleaning'
  AND NOT EXISTS (
    SELECT 1
    FROM service_price_guides g
    WHERE g.service_category_id = sc.id
      AND g.city_id = a.city_id
      AND g.area_id = a.id
  );
