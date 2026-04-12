-- Dev-only geography scaffold (Implementation 003 §11). Run manually after migrations.
-- Idempotent-ish: safe to re-run; skips rows that already exist by slug / composite keys.

INSERT INTO cities (name, slug, is_active)
VALUES ('Tbilisi', 'tbilisi', true)
ON CONFLICT (slug) DO NOTHING;

INSERT INTO areas (city_id, name, slug, is_active)
SELECT c.id, 'Vake', 'vake', true
FROM cities c
WHERE c.slug = 'tbilisi'
ON CONFLICT (city_id, slug) DO NOTHING;

INSERT INTO streets (city_id, area_id, name, slug, is_active)
SELECT c.id, a.id, 'Example Street', 'example-street', true
FROM cities c
JOIN areas a ON a.city_id = c.id AND a.slug = 'vake'
WHERE c.slug = 'tbilisi'
  AND NOT EXISTS (
    SELECT 1
    FROM streets s
    WHERE s.city_id = c.id
      AND s.area_id = a.id
      AND s.slug = 'example-street'
  );
