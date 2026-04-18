-- =============================================================================
-- DEV ONLY — One city, one category, 10 dispatch-ready plumbers + 1 client.
-- =============================================================================
-- Use this when you want a minimal geography surface and guaranteed matcher
-- matches for POST /orders (see `dev_seed_simple_dispatch_order.json`).
--
-- Run (from apps/api):
--   psql "$DATABASE_URL" -v ON_ERROR_STOP=1 -f seeds/dev_seed_simple_dispatch_ten_plumbers.sql
--
-- Login (password for all: DevSeed!ChangeMe — same Argon2 as other dev seeds):
--   Client:   simple_dispatch_client@seed.local
--   Plumber:  simple_dispatch_plumber_01@seed.local … simple_dispatch_plumber_10@seed.local
--
-- Then: POST /auth/login (client) → POST /orders with the JSON in the footer.
--       Worker or POST /internal/dispatch/advance should find plumbers (not skipped_no_plumbers).
-- =============================================================================

BEGIN;

-- Same Argon2id hash as dev_seed_dispatch_matcher_demo.sql / comprehensive (password: DevSeed!ChangeMe)
-- $argon2id$v=19$m=19456,t=3,p=1$q3n66uplbcwM7cCIM/ed7Q$Hgnkr1nyUTgzXiAOTmRYU/+NvfYy9cs/SjHLedfAPuI

-- ---------------------------------------------------------------------------
-- One city, one area, one street, one service category (stable UUIDs)
-- ---------------------------------------------------------------------------
INSERT INTO cities (id, name, slug, is_active)
VALUES (
    'f1111111-1111-4111-8111-222222220001'::uuid,
    'Simple One City',
    'simple-one-city-dispatch',
    true
)
ON CONFLICT (slug) DO NOTHING;

INSERT INTO areas (id, city_id, name, slug, is_active)
VALUES (
    'f1111111-1111-4111-8111-222222220002'::uuid,
    'f1111111-1111-4111-8111-222222220001'::uuid,
    'Central',
    'simple-one-city-central',
    true
)
ON CONFLICT (city_id, slug) DO NOTHING;

INSERT INTO streets (id, city_id, area_id, name, slug, is_active)
VALUES (
    'f1111111-1111-4111-8111-222222220003'::uuid,
    'f1111111-1111-4111-8111-222222220001'::uuid,
    'f1111111-1111-4111-8111-222222220002'::uuid,
    'First Street',
    'simple-one-city-first-st',
    true
)
ON CONFLICT (city_id, area_id, slug) WHERE area_id IS NOT NULL DO NOTHING;

INSERT INTO service_categories (id, name, slug, description, sort_order, is_active)
VALUES (
    'f1111111-1111-4111-8111-222222220004'::uuid,
    'Simple dispatch plumbing',
    'simple-one-city-plumbing',
    'Seed category: one city + ten plumbers',
    0,
    true
)
ON CONFLICT (slug) DO NOTHING;

-- ---------------------------------------------------------------------------
-- Users: 10 plumbers + 1 client
-- ---------------------------------------------------------------------------
INSERT INTO users (id, email, password_hash, role, user_status, is_email_verified)
VALUES
    (
        'f1111111-1111-4111-8111-333333330001'::uuid,
        'simple_dispatch_plumber_01@seed.local'::citext,
        '$argon2id$v=19$m=19456,t=3,p=1$q3n66uplbcwM7cCIM/ed7Q$Hgnkr1nyUTgzXiAOTmRYU/+NvfYy9cs/SjHLedfAPuI',
        'plumber'::user_role,
        'active'::user_status,
        true
    ),
    (
        'f1111111-1111-4111-8111-333333330002'::uuid,
        'simple_dispatch_plumber_02@seed.local'::citext,
        '$argon2id$v=19$m=19456,t=3,p=1$q3n66uplbcwM7cCIM/ed7Q$Hgnkr1nyUTgzXiAOTmRYU/+NvfYy9cs/SjHLedfAPuI',
        'plumber'::user_role,
        'active'::user_status,
        true
    ),
    (
        'f1111111-1111-4111-8111-333333330003'::uuid,
        'simple_dispatch_plumber_03@seed.local'::citext,
        '$argon2id$v=19$m=19456,t=3,p=1$q3n66uplbcwM7cCIM/ed7Q$Hgnkr1nyUTgzXiAOTmRYU/+NvfYy9cs/SjHLedfAPuI',
        'plumber'::user_role,
        'active'::user_status,
        true
    ),
    (
        'f1111111-1111-4111-8111-333333330004'::uuid,
        'simple_dispatch_plumber_04@seed.local'::citext,
        '$argon2id$v=19$m=19456,t=3,p=1$q3n66uplbcwM7cCIM/ed7Q$Hgnkr1nyUTgzXiAOTmRYU/+NvfYy9cs/SjHLedfAPuI',
        'plumber'::user_role,
        'active'::user_status,
        true
    ),
    (
        'f1111111-1111-4111-8111-333333330005'::uuid,
        'simple_dispatch_plumber_05@seed.local'::citext,
        '$argon2id$v=19$m=19456,t=3,p=1$q3n66uplbcwM7cCIM/ed7Q$Hgnkr1nyUTgzXiAOTmRYU/+NvfYy9cs/SjHLedfAPuI',
        'plumber'::user_role,
        'active'::user_status,
        true
    ),
    (
        'f1111111-1111-4111-8111-333333330006'::uuid,
        'simple_dispatch_plumber_06@seed.local'::citext,
        '$argon2id$v=19$m=19456,t=3,p=1$q3n66uplbcwM7cCIM/ed7Q$Hgnkr1nyUTgzXiAOTmRYU/+NvfYy9cs/SjHLedfAPuI',
        'plumber'::user_role,
        'active'::user_status,
        true
    ),
    (
        'f1111111-1111-4111-8111-333333330007'::uuid,
        'simple_dispatch_plumber_07@seed.local'::citext,
        '$argon2id$v=19$m=19456,t=3,p=1$q3n66uplbcwM7cCIM/ed7Q$Hgnkr1nyUTgzXiAOTmRYU/+NvfYy9cs/SjHLedfAPuI',
        'plumber'::user_role,
        'active'::user_status,
        true
    ),
    (
        'f1111111-1111-4111-8111-333333330008'::uuid,
        'simple_dispatch_plumber_08@seed.local'::citext,
        '$argon2id$v=19$m=19456,t=3,p=1$q3n66uplbcwM7cCIM/ed7Q$Hgnkr1nyUTgzXiAOTmRYU/+NvfYy9cs/SjHLedfAPuI',
        'plumber'::user_role,
        'active'::user_status,
        true
    ),
    (
        'f1111111-1111-4111-8111-333333330009'::uuid,
        'simple_dispatch_plumber_09@seed.local'::citext,
        '$argon2id$v=19$m=19456,t=3,p=1$q3n66uplbcwM7cCIM/ed7Q$Hgnkr1nyUTgzXiAOTmRYU/+NvfYy9cs/SjHLedfAPuI',
        'plumber'::user_role,
        'active'::user_status,
        true
    ),
    (
        'f1111111-1111-4111-8111-33333333000a'::uuid,
        'simple_dispatch_plumber_10@seed.local'::citext,
        '$argon2id$v=19$m=19456,t=3,p=1$q3n66uplbcwM7cCIM/ed7Q$Hgnkr1nyUTgzXiAOTmRYU/+NvfYy9cs/SjHLedfAPuI',
        'plumber'::user_role,
        'active'::user_status,
        true
    ),
    (
        'f1111111-1111-4111-8111-3333333300c1'::uuid,
        'simple_dispatch_client@seed.local'::citext,
        '$argon2id$v=19$m=19456,t=3,p=1$q3n66uplbcwM7cCIM/ed7Q$Hgnkr1nyUTgzXiAOTmRYU/+NvfYy9cs/SjHLedfAPuI',
        'client'::user_role,
        'active'::user_status,
        true
    )
ON CONFLICT (email) DO NOTHING;

-- ---------------------------------------------------------------------------
-- Plumber profiles: online, available, GPS near order point (41.7155, 44.7855)
-- ---------------------------------------------------------------------------
INSERT INTO plumber_profiles (
    id,
    user_id,
    full_name,
    phone,
    experience_years,
    is_approved,
    is_online,
    is_available,
    current_city_id,
    current_area_id,
    current_street_id,
    current_lat,
    current_lng,
    last_location_updated_at,
    service_radius_km,
    token_balance,
    rating_avg
)
VALUES
    (
        'f1111111-1111-4111-8111-444444440001'::uuid,
        'f1111111-1111-4111-8111-333333330001'::uuid,
        'Simple Plumber 01',
        '+10005550001',
        4,
        true, true, true,
        'f1111111-1111-4111-8111-222222220001'::uuid,
        'f1111111-1111-4111-8111-222222220002'::uuid,
        'f1111111-1111-4111-8111-222222220003'::uuid,
        41.7150, 44.7850, now(), 25, 50, 4.20
    ),
    (
        'f1111111-1111-4111-8111-444444440002'::uuid,
        'f1111111-1111-4111-8111-333333330002'::uuid,
        'Simple Plumber 02',
        '+10005550002',
        5,
        true, true, true,
        'f1111111-1111-4111-8111-222222220001'::uuid,
        'f1111111-1111-4111-8111-222222220002'::uuid,
        'f1111111-1111-4111-8111-222222220003'::uuid,
        41.7152, 44.7852, now(), 25, 50, 4.25
    ),
    (
        'f1111111-1111-4111-8111-444444440003'::uuid,
        'f1111111-1111-4111-8111-333333330003'::uuid,
        'Simple Plumber 03',
        '+10005550003',
        6,
        true, true, true,
        'f1111111-1111-4111-8111-222222220001'::uuid,
        'f1111111-1111-4111-8111-222222220002'::uuid,
        'f1111111-1111-4111-8111-222222220003'::uuid,
        41.7154, 44.7854, now(), 25, 50, 4.30
    ),
    (
        'f1111111-1111-4111-8111-444444440004'::uuid,
        'f1111111-1111-4111-8111-333333330004'::uuid,
        'Simple Plumber 04',
        '+10005550004',
        7,
        true, true, true,
        'f1111111-1111-4111-8111-222222220001'::uuid,
        'f1111111-1111-4111-8111-222222220002'::uuid,
        'f1111111-1111-4111-8111-222222220003'::uuid,
        41.7156, 44.7856, now(), 25, 50, 4.35
    ),
    (
        'f1111111-1111-4111-8111-444444440005'::uuid,
        'f1111111-1111-4111-8111-333333330005'::uuid,
        'Simple Plumber 05',
        '+10005550005',
        8,
        true, true, true,
        'f1111111-1111-4111-8111-222222220001'::uuid,
        'f1111111-1111-4111-8111-222222220002'::uuid,
        'f1111111-1111-4111-8111-222222220003'::uuid,
        41.7158, 44.7858, now(), 25, 50, 4.40
    ),
    (
        'f1111111-1111-4111-8111-444444440006'::uuid,
        'f1111111-1111-4111-8111-333333330006'::uuid,
        'Simple Plumber 06',
        '+10005550006',
        9,
        true, true, true,
        'f1111111-1111-4111-8111-222222220001'::uuid,
        'f1111111-1111-4111-8111-222222220002'::uuid,
        'f1111111-1111-4111-8111-222222220003'::uuid,
        41.7160, 44.7860, now(), 25, 50, 4.45
    ),
    (
        'f1111111-1111-4111-8111-444444440007'::uuid,
        'f1111111-1111-4111-8111-333333330007'::uuid,
        'Simple Plumber 07',
        '+10005550007',
        10,
        true, true, true,
        'f1111111-1111-4111-8111-222222220001'::uuid,
        'f1111111-1111-4111-8111-222222220002'::uuid,
        'f1111111-1111-4111-8111-222222220003'::uuid,
        41.7162, 44.7862, now(), 25, 50, 4.50
    ),
    (
        'f1111111-1111-4111-8111-444444440008'::uuid,
        'f1111111-1111-4111-8111-333333330008'::uuid,
        'Simple Plumber 08',
        '+10005550008',
        11,
        true, true, true,
        'f1111111-1111-4111-8111-222222220001'::uuid,
        'f1111111-1111-4111-8111-222222220002'::uuid,
        'f1111111-1111-4111-8111-222222220003'::uuid,
        41.7164, 44.7864, now(), 25, 50, 4.55
    ),
    (
        'f1111111-1111-4111-8111-444444440009'::uuid,
        'f1111111-1111-4111-8111-333333330009'::uuid,
        'Simple Plumber 09',
        '+10005550009',
        12,
        true, true, true,
        'f1111111-1111-4111-8111-222222220001'::uuid,
        'f1111111-1111-4111-8111-222222220002'::uuid,
        'f1111111-1111-4111-8111-222222220003'::uuid,
        41.7166, 44.7866, now(), 25, 50, 4.60
    ),
    (
        'f1111111-1111-4111-8111-44444444000a'::uuid,
        'f1111111-1111-4111-8111-33333333000a'::uuid,
        'Simple Plumber 10',
        '+10005550010',
        13,
        true, true, true,
        'f1111111-1111-4111-8111-222222220001'::uuid,
        'f1111111-1111-4111-8111-222222220002'::uuid,
        'f1111111-1111-4111-8111-222222220003'::uuid,
        41.7168, 44.7868, now(), 25, 50, 4.65
    )
ON CONFLICT (user_id) DO NOTHING;

INSERT INTO client_profiles (
    user_id,
    full_name,
    phone,
    default_city_id,
    default_area_id,
    default_street_id
)
VALUES (
    'f1111111-1111-4111-8111-3333333300c1'::uuid,
    'Simple Dispatch Client',
    '+10005550999',
    'f1111111-1111-4111-8111-222222220001'::uuid,
    'f1111111-1111-4111-8111-222222220002'::uuid,
    'f1111111-1111-4111-8111-222222220003'::uuid
)
ON CONFLICT (user_id) DO NOTHING;

-- Category for all simple-dispatch plumbers; whole-city service area (NULL area_id)
INSERT INTO plumber_services (plumber_id, service_category_id)
SELECT p.id, 'f1111111-1111-4111-8111-222222220004'::uuid
FROM plumber_profiles p
WHERE p.user_id IN (
    SELECT id
    FROM users
    WHERE email::text LIKE 'simple_dispatch_plumber_%@seed.local'
)
ON CONFLICT (plumber_id, service_category_id) DO NOTHING;

INSERT INTO plumber_service_areas (plumber_id, city_id, area_id)
SELECT
    p.id,
    'f1111111-1111-4111-8111-222222220001'::uuid,
    NULL::uuid
FROM plumber_profiles p
WHERE p.user_id IN (
    SELECT id
    FROM users
    WHERE email::text LIKE 'simple_dispatch_plumber_%@seed.local'
)
ON CONFLICT (plumber_id, city_id) WHERE area_id IS NULL DO NOTHING;

COMMIT;

-- Optional — city-wide matcher fallback (different area than plumbers): after this seed,
-- run `seeds/dev_seed_city_fallback_second_area.sql` and use
-- `dev_seed_simple_dispatch_city_fallback_order.json` for POST /orders.
--
-- =============================================================================
-- POST /orders body (Bearer: simple_dispatch_client@seed.local / DevSeed!ChangeMe)
-- =============================================================================
-- {
--   "service_category_id": "f1111111-1111-4111-8111-222222220004",
--   "city_id": "f1111111-1111-4111-8111-222222220001",
--   "area_id": "f1111111-1111-4111-8111-222222220002",
--   "street_id": "f1111111-1111-4111-8111-222222220003",
--   "address_line": "10 Simple Street",
--   "lat": 41.7155,
--   "lng": 44.7855,
--   "description": "Leak — simple one-city seed.",
--   "urgency": "normal",
--   "media": []
-- }
-- =============================================================================
