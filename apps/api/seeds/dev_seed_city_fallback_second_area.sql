-- =============================================================================
-- DEV ONLY — City-wide fallback (Implementation 004 §4.3) on top of simple seed
-- =============================================================================
-- **Prerequisite:** `dev_seed_simple_dispatch_ten_plumbers.sql` has been applied
-- (same city `f1111111-…220001`, plumbers, client, category).
--
-- This seed:
--   1. Adds a **second area** ("East Side") in that city + a street there.
--   2. Replaces plumbers’ **city-wide** `plumber_service_areas` (area_id NULL)
--      with **Central-only** rows (`area_id` = existing Central area).
--
-- Effect: an order whose **`area_id` is the new East area** does **not** match
-- any plumber under **strict** area rules, but **does** match under **city
-- fallback** (same city, service category, ranking without strict radius cap
-- per fallback query).
--
-- Run (from apps/api):
--   psql "$DATABASE_URL" -v ON_ERROR_STOP=1 -f seeds/dev_seed_city_fallback_second_area.sql
--
-- Then POST /orders using `dev_seed_simple_dispatch_city_fallback_order.json`
-- and advance / worker — success logs should show `outcome=success` and
-- `match_pass=city_fallback`. (If you see `skipped_no_plumbers` with
-- `match_pass=city_fallback`, strict was empty but fallback also returned no
-- rows — often stale GPS; this script refreshes `last_location_updated_at`.)
-- =============================================================================

BEGIN;

-- Second area in **Simple One City** (same city_id as simple dispatch seed)
INSERT INTO areas (id, city_id, name, slug, is_active)
VALUES (
    'f1111111-1111-4111-8111-222222220007'::uuid,
    'f1111111-1111-4111-8111-222222220001'::uuid,
    'East Side',
    'simple-one-city-east-side',
    true
)
ON CONFLICT (city_id, slug) DO NOTHING;

-- Street in East Side (for POST /orders FK)
INSERT INTO streets (id, city_id, area_id, name, slug, is_active)
VALUES (
    'f1111111-1111-4111-8111-222222220008'::uuid,
    'f1111111-1111-4111-8111-222222220001'::uuid,
    'f1111111-1111-4111-8111-222222220007'::uuid,
    'East Wharf Street',
    'simple-one-city-east-wharf',
    true
)
ON CONFLICT (city_id, area_id, slug) WHERE area_id IS NOT NULL DO NOTHING;

-- Remove city-wide coverage so strict no longer matches orders in other areas
DELETE FROM plumber_service_areas psa
USING plumber_profiles p
JOIN users u ON u.id = p.user_id
WHERE psa.plumber_id = p.id
  AND u.email::text LIKE 'simple_dispatch_plumber_%@seed.local'
  AND psa.area_id IS NULL;

-- Central-only service area (strict requires Central OR NULL OR order area; East order → miss)
INSERT INTO plumber_service_areas (plumber_id, city_id, area_id)
SELECT
    p.id,
    'f1111111-1111-4111-8111-222222220001'::uuid,
    'f1111111-1111-4111-8111-222222220002'::uuid
FROM plumber_profiles p
JOIN users u ON u.id = p.user_id
WHERE u.email::text LIKE 'simple_dispatch_plumber_%@seed.local'
ON CONFLICT (plumber_id, city_id, area_id) WHERE area_id IS NOT NULL DO NOTHING;

-- Matcher `location_max_age` default is **15 minutes** (`MatcherConfig`). Stale
-- `last_location_updated_at` removes plumbers from **both** strict and city fallback.
UPDATE plumber_profiles p
SET last_location_updated_at = NOW()
FROM users u
WHERE u.id = p.user_id
  AND u.email::text LIKE 'simple_dispatch_plumber_%@seed.local';

COMMIT;

-- =============================================================================
-- Order JSON: see `dev_seed_simple_dispatch_city_fallback_order.json`
-- =============================================================================
