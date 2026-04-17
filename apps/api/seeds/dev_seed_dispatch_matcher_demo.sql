-- =============================================================================
-- DEV ONLY — 10 dispatch-ready plumbers + 1 client for matcher / advance testing.
-- =============================================================================
-- Prerequisites: `sqlx migrate run` (or equivalent) on an empty or existing DB.
--
-- Run (from apps/api):
--   psql "$DATABASE_URL" -v ON_ERROR_STOP=1 -f seeds/dev_seed_dispatch_matcher_demo.sql
--
-- Fixed UUIDs match `seeds/dev_seed_dispatch_matcher_demo_order.json` and the footer below.
-- Login client: dispatch_demo_client@dev.local  /  DevSeed!ChangeMe
-- (same Argon2 hash as dev_seed_comprehensive.sql)
--
-- After seed: POST /orders (Bearer client) with the JSON below, then
-- POST /internal/dispatch/advance with {"order_id":"<returned id>"}.
-- =============================================================================

BEGIN;

-- ---------------------------------------------------------------------------
-- Geography + category (stable IDs for Postman)
-- ---------------------------------------------------------------------------
INSERT INTO cities (id, name, slug, is_active)
VALUES (
    'a0000001-0000-4000-8000-000000000001'::uuid,
    'Dispatch Matcher Demo City',
    'dispatch-matcher-demo-city',
    true
)
ON CONFLICT (slug) DO NOTHING;

INSERT INTO areas (id, city_id, name, slug, is_active)
VALUES (
    'a0000001-0000-4000-8000-000000000002'::uuid,
    'a0000001-0000-4000-8000-000000000001'::uuid,
    'Demo District',
    'dispatch-matcher-demo-district',
    true
)
ON CONFLICT (city_id, slug) DO NOTHING;

INSERT INTO streets (id, city_id, area_id, name, slug, is_active)
VALUES (
    'a0000001-0000-4000-8000-000000000003'::uuid,
    'a0000001-0000-4000-8000-000000000001'::uuid,
    'a0000001-0000-4000-8000-000000000002'::uuid,
    'Demo Street',
    'dispatch-matcher-demo-street',
    true
)
ON CONFLICT (city_id, area_id, slug) WHERE area_id IS NOT NULL DO NOTHING;

INSERT INTO service_categories (id, name, slug, description, sort_order, is_active)
VALUES (
    'a0000001-0000-4000-8000-000000000004'::uuid,
    'Dispatch demo — drain cleaning',
    'dispatch-matcher-drain-cleaning',
    'Seed category for dispatch matcher demo',
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
        'b0000001-0000-4000-8000-000000000001'::uuid,
        'dispatch_demo_plumber_01@dev.local'::citext,
        '$argon2id$v=19$m=19456,t=3,p=1$q3n66uplbcwM7cCIM/ed7Q$Hgnkr1nyUTgzXiAOTmRYU/+NvfYy9cs/SjHLedfAPuI',
        'plumber'::user_role,
        'active'::user_status,
        true
    ),
    (
        'b0000001-0000-4000-8000-000000000002'::uuid,
        'dispatch_demo_plumber_02@dev.local'::citext,
        '$argon2id$v=19$m=19456,t=3,p=1$q3n66uplbcwM7cCIM/ed7Q$Hgnkr1nyUTgzXiAOTmRYU/+NvfYy9cs/SjHLedfAPuI',
        'plumber'::user_role,
        'active'::user_status,
        true
    ),
    (
        'b0000001-0000-4000-8000-000000000003'::uuid,
        'dispatch_demo_plumber_03@dev.local'::citext,
        '$argon2id$v=19$m=19456,t=3,p=1$q3n66uplbcwM7cCIM/ed7Q$Hgnkr1nyUTgzXiAOTmRYU/+NvfYy9cs/SjHLedfAPuI',
        'plumber'::user_role,
        'active'::user_status,
        true
    ),
    (
        'b0000001-0000-4000-8000-000000000004'::uuid,
        'dispatch_demo_plumber_04@dev.local'::citext,
        '$argon2id$v=19$m=19456,t=3,p=1$q3n66uplbcwM7cCIM/ed7Q$Hgnkr1nyUTgzXiAOTmRYU/+NvfYy9cs/SjHLedfAPuI',
        'plumber'::user_role,
        'active'::user_status,
        true
    ),
    (
        'b0000001-0000-4000-8000-000000000005'::uuid,
        'dispatch_demo_plumber_05@dev.local'::citext,
        '$argon2id$v=19$m=19456,t=3,p=1$q3n66uplbcwM7cCIM/ed7Q$Hgnkr1nyUTgzXiAOTmRYU/+NvfYy9cs/SjHLedfAPuI',
        'plumber'::user_role,
        'active'::user_status,
        true
    ),
    (
        'b0000001-0000-4000-8000-000000000006'::uuid,
        'dispatch_demo_plumber_06@dev.local'::citext,
        '$argon2id$v=19$m=19456,t=3,p=1$q3n66uplbcwM7cCIM/ed7Q$Hgnkr1nyUTgzXiAOTmRYU/+NvfYy9cs/SjHLedfAPuI',
        'plumber'::user_role,
        'active'::user_status,
        true
    ),
    (
        'b0000001-0000-4000-8000-000000000007'::uuid,
        'dispatch_demo_plumber_07@dev.local'::citext,
        '$argon2id$v=19$m=19456,t=3,p=1$q3n66uplbcwM7cCIM/ed7Q$Hgnkr1nyUTgzXiAOTmRYU/+NvfYy9cs/SjHLedfAPuI',
        'plumber'::user_role,
        'active'::user_status,
        true
    ),
    (
        'b0000001-0000-4000-8000-000000000008'::uuid,
        'dispatch_demo_plumber_08@dev.local'::citext,
        '$argon2id$v=19$m=19456,t=3,p=1$q3n66uplbcwM7cCIM/ed7Q$Hgnkr1nyUTgzXiAOTmRYU/+NvfYy9cs/SjHLedfAPuI',
        'plumber'::user_role,
        'active'::user_status,
        true
    ),
    (
        'b0000001-0000-4000-8000-000000000009'::uuid,
        'dispatch_demo_plumber_09@dev.local'::citext,
        '$argon2id$v=19$m=19456,t=3,p=1$q3n66uplbcwM7cCIM/ed7Q$Hgnkr1nyUTgzXiAOTmRYU/+NvfYy9cs/SjHLedfAPuI',
        'plumber'::user_role,
        'active'::user_status,
        true
    ),
    (
        'b0000001-0000-4000-8000-00000000000a'::uuid,
        'dispatch_demo_plumber_10@dev.local'::citext,
        '$argon2id$v=19$m=19456,t=3,p=1$q3n66uplbcwM7cCIM/ed7Q$Hgnkr1nyUTgzXiAOTmRYU/+NvfYy9cs/SjHLedfAPuI',
        'plumber'::user_role,
        'active'::user_status,
        true
    ),
    (
        'b0000001-0000-4000-8000-00000000000c'::uuid,
        'dispatch_demo_client@dev.local'::citext,
        '$argon2id$v=19$m=19456,t=3,p=1$q3n66uplbcwM7cCIM/ed7Q$Hgnkr1nyUTgzXiAOTmRYU/+NvfYy9cs/SjHLedfAPuI',
        'client'::user_role,
        'active'::user_status,
        true
    )
ON CONFLICT (email) DO NOTHING;

-- ---------------------------------------------------------------------------
-- Plumber profiles: approved, online, available, fresh GPS, wide radius
-- (matches dispatch_matcher/query.rs filters)
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
        'c0000001-0000-4000-8000-000000000001'::uuid,
        'b0000001-0000-4000-8000-000000000001'::uuid,
        'Demo Plumber 01',
        '+995555000001',
        5,
        true,
        true,
        true,
        'a0000001-0000-4000-8000-000000000001'::uuid,
        'a0000001-0000-4000-8000-000000000002'::uuid,
        'a0000001-0000-4000-8000-000000000003'::uuid,
        41.7150,
        44.7850,
        now(),
        25,
        50,
        4.20
    ),
    (
        'c0000001-0000-4000-8000-000000000002'::uuid,
        'b0000001-0000-4000-8000-000000000002'::uuid,
        'Demo Plumber 02',
        '+995555000002',
        6,
        true,
        true,
        true,
        'a0000001-0000-4000-8000-000000000001'::uuid,
        'a0000001-0000-4000-8000-000000000002'::uuid,
        'a0000001-0000-4000-8000-000000000003'::uuid,
        41.7152,
        44.7852,
        now(),
        25,
        50,
        4.25
    ),
    (
        'c0000001-0000-4000-8000-000000000003'::uuid,
        'b0000001-0000-4000-8000-000000000003'::uuid,
        'Demo Plumber 03',
        '+995555000003',
        7,
        true,
        true,
        true,
        'a0000001-0000-4000-8000-000000000001'::uuid,
        'a0000001-0000-4000-8000-000000000002'::uuid,
        'a0000001-0000-4000-8000-000000000003'::uuid,
        41.7154,
        44.7854,
        now(),
        25,
        50,
        4.30
    ),
    (
        'c0000001-0000-4000-8000-000000000004'::uuid,
        'b0000001-0000-4000-8000-000000000004'::uuid,
        'Demo Plumber 04',
        '+995555000004',
        8,
        true,
        true,
        true,
        'a0000001-0000-4000-8000-000000000001'::uuid,
        'a0000001-0000-4000-8000-000000000002'::uuid,
        'a0000001-0000-4000-8000-000000000003'::uuid,
        41.7156,
        44.7856,
        now(),
        25,
        50,
        4.35
    ),
    (
        'c0000001-0000-4000-8000-000000000005'::uuid,
        'b0000001-0000-4000-8000-000000000005'::uuid,
        'Demo Plumber 05',
        '+995555000005',
        9,
        true,
        true,
        true,
        'a0000001-0000-4000-8000-000000000001'::uuid,
        'a0000001-0000-4000-8000-000000000002'::uuid,
        'a0000001-0000-4000-8000-000000000003'::uuid,
        41.7158,
        44.7858,
        now(),
        25,
        50,
        4.40
    ),
    (
        'c0000001-0000-4000-8000-000000000006'::uuid,
        'b0000001-0000-4000-8000-000000000006'::uuid,
        'Demo Plumber 06',
        '+995555000006',
        10,
        true,
        true,
        true,
        'a0000001-0000-4000-8000-000000000001'::uuid,
        'a0000001-0000-4000-8000-000000000002'::uuid,
        'a0000001-0000-4000-8000-000000000003'::uuid,
        41.7160,
        44.7860,
        now(),
        25,
        50,
        4.45
    ),
    (
        'c0000001-0000-4000-8000-000000000007'::uuid,
        'b0000001-0000-4000-8000-000000000007'::uuid,
        'Demo Plumber 07',
        '+995555000007',
        11,
        true,
        true,
        true,
        'a0000001-0000-4000-8000-000000000001'::uuid,
        'a0000001-0000-4000-8000-000000000002'::uuid,
        'a0000001-0000-4000-8000-000000000003'::uuid,
        41.7162,
        44.7862,
        now(),
        25,
        50,
        4.50
    ),
    (
        'c0000001-0000-4000-8000-000000000008'::uuid,
        'b0000001-0000-4000-8000-000000000008'::uuid,
        'Demo Plumber 08',
        '+995555000008',
        12,
        true,
        true,
        true,
        'a0000001-0000-4000-8000-000000000001'::uuid,
        'a0000001-0000-4000-8000-000000000002'::uuid,
        'a0000001-0000-4000-8000-000000000003'::uuid,
        41.7164,
        44.7864,
        now(),
        25,
        50,
        4.55
    ),
    (
        'c0000001-0000-4000-8000-000000000009'::uuid,
        'b0000001-0000-4000-8000-000000000009'::uuid,
        'Demo Plumber 09',
        '+995555000009',
        13,
        true,
        true,
        true,
        'a0000001-0000-4000-8000-000000000001'::uuid,
        'a0000001-0000-4000-8000-000000000002'::uuid,
        'a0000001-0000-4000-8000-000000000003'::uuid,
        41.7166,
        44.7866,
        now(),
        25,
        50,
        4.60
    ),
    (
        'c0000001-0000-4000-8000-00000000000a'::uuid,
        'b0000001-0000-4000-8000-00000000000a'::uuid,
        'Demo Plumber 10',
        '+995555000010',
        14,
        true,
        true,
        true,
        'a0000001-0000-4000-8000-000000000001'::uuid,
        'a0000001-0000-4000-8000-000000000002'::uuid,
        'a0000001-0000-4000-8000-000000000003'::uuid,
        41.7168,
        44.7868,
        now(),
        25,
        50,
        4.65
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
    'b0000001-0000-4000-8000-00000000000c'::uuid,
    'Dispatch Demo Client',
    '+995555000099',
    'a0000001-0000-4000-8000-000000000001'::uuid,
    'a0000001-0000-4000-8000-000000000002'::uuid,
    'a0000001-0000-4000-8000-000000000003'::uuid
)
ON CONFLICT (user_id) DO NOTHING;

-- ---------------------------------------------------------------------------
-- Services + whole-city service areas (covers any area in the demo city)
-- ---------------------------------------------------------------------------
INSERT INTO plumber_services (plumber_id, service_category_id)
SELECT p.id, 'a0000001-0000-4000-8000-000000000004'::uuid
FROM plumber_profiles p
WHERE p.user_id IN (
    SELECT id
    FROM users
    WHERE email::text LIKE 'dispatch_demo_plumber_%@dev.local'
)
ON CONFLICT (plumber_id, service_category_id) DO NOTHING;

INSERT INTO plumber_service_areas (plumber_id, city_id, area_id)
SELECT
    p.id,
    'a0000001-0000-4000-8000-000000000001'::uuid,
    NULL::uuid
FROM plumber_profiles p
WHERE p.user_id IN (
    SELECT id
    FROM users
    WHERE email::text LIKE 'dispatch_demo_plumber_%@dev.local'
)
ON CONFLICT (plumber_id, city_id) WHERE area_id IS NULL DO NOTHING;

COMMIT;

-- =============================================================================
-- Sample POST /orders body (Bearer: dispatch_demo_client@dev.local)
-- Coordinates align with seeded plumber locations (Tbilisi-scale test point).
-- =============================================================================
-- {
--   "service_category_id": "a0000001-0000-4000-8000-000000000004",
--   "city_id": "a0000001-0000-4000-8000-000000000001",
--   "area_id": "a0000001-0000-4000-8000-000000000002",
--   "street_id": "a0000001-0000-4000-8000-000000000003",
--   "address_line": "1 Demo Street (dispatch matcher seed)",
--   "lat": 41.7155,
--   "lng": 44.7855,
--   "description": "Leak under kitchen sink — dispatch demo order.",
--   "urgency": "normal",
--   "media": []
-- }
-- =============================================================================
