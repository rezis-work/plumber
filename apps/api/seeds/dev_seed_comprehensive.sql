-- =============================================================================
-- DEV ONLY — large FK-aligned seed for all migrated Phase 2 tables (no reviews).
-- =============================================================================
-- Prerequisites:
--   1. `sqlx migrate run`
--   2. `psql -f seeds/dev_truncate_all.sql` (or run truncate section) so tables are empty
--
-- Shared login for every seeded user (clients, plumbers, admins):
--   Password: DevSeed!ChangeMe
--   Argon2id PHC below matches default `PasswordConfig` in `src/modules/auth/passwords.rs`
--   (m_cost=19456, t_cost=3, p_cost=1). Regenerate with:
--     `cargo run --example print_dev_seed_password_hash` from `apps/api`.
--
-- Volume (tune by editing generate_series bounds / constants in comments):
--   3 Georgian cities (Tbilisi, Batumi, Kutaisi), 14 real districts/areas, 84 real streets;
--   50 plumbing service categories; 60 clients; 20 plumbers; 3 admins;
--   ~60% plumber×category pairs for plumber_services; whole-city + 2 extra areas per plumber;
--   18 status rows per plumber; 120 orders; up to 3 dispatches per non-searching order;
--   ~320 admin_audit rows + attributed slice.
-- Geography: real place names (Latin transliteration). Coordinates per city in plausible ranges.
-- =============================================================================

BEGIN;

-- ---------------------------------------------------------------------------
-- Geography — Georgia (Tbilisi-heavy): real cities, districts, street names
-- ---------------------------------------------------------------------------
INSERT INTO cities (name, slug, is_active)
VALUES
    ('Tbilisi', 'tbilisi', true),
    ('Batumi', 'batumi', true),
    ('Kutaisi', 'kutaisi', true);

INSERT INTO areas (city_id, name, slug, is_active)
SELECT c.id, v.name, v.slug, true
FROM cities AS c
INNER JOIN (
    VALUES
        ('tbilisi', 'Vake', 'vake'),
        ('tbilisi', 'Saburtalo', 'saburtalo'),
        ('tbilisi', 'Mtatsminda', 'mtatsminda'),
        ('tbilisi', 'Chughureti', 'chughureti'),
        ('tbilisi', 'Didube', 'didube'),
        ('tbilisi', 'Isani', 'isani'),
        ('tbilisi', 'Gldani', 'gldani'),
        ('tbilisi', 'Nadzaladevi', 'nadzaladevi'),
        ('batumi', 'Old Batumi', 'batumi-old-town'),
        ('batumi', 'Angisa', 'angisa'),
        ('batumi', 'Khelvachauri', 'khelvachauri'),
        ('kutaisi', 'Central Kutaisi', 'kutaisi-centre'),
        ('kutaisi', 'Bagrati', 'bagrati'),
        ('kutaisi', 'Gelati Road area', 'gelati-road')
) AS v(city_slug, name, slug) ON c.slug = v.city_slug;

INSERT INTO streets (city_id, area_id, name, slug, is_active)
SELECT c.id, a.id, x.street_name, x.street_slug, true
FROM (
    VALUES
        ('tbilisi', 'vake', 'Ilia Chavchavadze Avenue', 'chavchavadze-ave-vake'),
        ('tbilisi', 'vake', 'Abashidze Street', 'abashidze-st-vake'),
        ('tbilisi', 'vake', 'Viktor Dolidze Street', 'dolidze-st-vake'),
        ('tbilisi', 'vake', 'Niko Nikoladze Street', 'nikoladze-st-vake'),
        ('tbilisi', 'vake', 'Mosashvili Street', 'mosashvili-st-vake'),
        ('tbilisi', 'vake', 'Barnovi Street', 'barnovi-st-vake'),
        ('tbilisi', 'saburtalo', 'Vazha-Pshavela Avenue', 'vazha-pshavela-ave'),
        ('tbilisi', 'saburtalo', 'Pekini Avenue', 'pekini-ave'),
        ('tbilisi', 'saburtalo', 'Mikheil Tamarashvili Street', 'tamarashvili-st'),
        ('tbilisi', 'saburtalo', 'Merab Kostava Street', 'kostava-st-saburtalo'),
        ('tbilisi', 'saburtalo', 'Alexander Kazbegi Avenue', 'kazbegi-ave-saburtalo'),
        ('tbilisi', 'saburtalo', 'Belinski Street', 'belinski-st'),
        ('tbilisi', 'mtatsminda', 'Shota Rustaveli Avenue', 'rustaveli-ave-tbilisi'),
        ('tbilisi', 'mtatsminda', 'Lado Gudiashvili Street', 'gudiashvili-st'),
        ('tbilisi', 'mtatsminda', 'Nikoloz Baratashvili Street', 'baratashvili-st'),
        ('tbilisi', 'mtatsminda', 'Sioni Street', 'sioni-st'),
        ('tbilisi', 'mtatsminda', 'Erekle II Street', 'erekle-ii-st'),
        ('tbilisi', 'mtatsminda', 'Leonidze Street', 'leonidze-st'),
        ('tbilisi', 'chughureti', 'David Aghmashenebeli Avenue', 'aghmashenebeli-ave'),
        ('tbilisi', 'chughureti', 'Kote Marjanishvili Street', 'marjanishvili-st'),
        ('tbilisi', 'chughureti', 'Giorgi Akhvlediani Street', 'akhvlediani-st'),
        ('tbilisi', 'chughureti', 'Alexander Pushkin Street', 'pushkin-st-tbilisi'),
        ('tbilisi', 'chughureti', 'Shalva Dadiani Street', 'dadiani-st'),
        ('tbilisi', 'chughureti', 'Potskhverashvili Street', 'potskhverashvili-st'),
        ('tbilisi', 'didube', 'Akaki Tsereteli Avenue', 'tsereteli-ave'),
        ('tbilisi', 'didube', 'Aleksandre Gotsiridze Street', 'gotsiridze-st'),
        ('tbilisi', 'didube', 'Jikia Street', 'jikia-st'),
        ('tbilisi', 'didube', 'Tornike Eristavi Street', 'eristavi-st'),
        ('tbilisi', 'didube', 'Gotsiridze Rise', 'gotsiridze-rise'),
        ('tbilisi', 'didube', 'Didube Plateau Street', 'didube-plateau-st'),
        ('tbilisi', 'isani', 'George Bush Street', 'bush-st-tbilisi'),
        ('tbilisi', 'isani', 'Ketevan Dedofali Avenue', 'dedofali-ave'),
        ('tbilisi', 'isani', 'Ortachala Highway', 'ortachala-hwy'),
        ('tbilisi', 'isani', 'Akhmeteli Street', 'akhmeteli-st'),
        ('tbilisi', 'isani', 'Samgori Street', 'samgori-st'),
        ('tbilisi', 'isani', 'Navtlughi Street', 'navtlughi-st'),
        ('tbilisi', 'gldani', 'Gldani Highway', 'gldani-hwy'),
        ('tbilisi', 'gldani', 'Khizanishvili Street', 'khizanishvili-st'),
        ('tbilisi', 'gldani', 'Sasadilo Street', 'sasadilo-st'),
        ('tbilisi', 'gldani', 'David IV Street', 'david-iv-st-gldani'),
        ('tbilisi', 'gldani', 'Gldani 1st microdistrict', 'gldani-m1'),
        ('tbilisi', 'gldani', 'Gldani 2nd microdistrict', 'gldani-m2'),
        ('tbilisi', 'nadzaladevi', 'Petre Ianushevich Street', 'ianushevich-st'),
        ('tbilisi', 'nadzaladevi', 'Lomis Street', 'lomis-st'),
        ('tbilisi', 'nadzaladevi', 'Tsotne Dadiani Street', 'tsotne-dadiani-st'),
        ('tbilisi', 'nadzaladevi', 'Nadzaladevi Highway', 'nadzaladevi-hwy'),
        ('tbilisi', 'nadzaladevi', 'Khudadovi Street', 'khudadovi-st'),
        ('tbilisi', 'nadzaladevi', 'Delisi Street', 'delisi-st'),
        ('batumi', 'batumi-old-town', 'Batumi Boulevard', 'batumi-boulevard'),
        ('batumi', 'batumi-old-town', 'Rustaveli Avenue (Batumi)', 'rustaveli-ave-batumi'),
        ('batumi', 'batumi-old-town', 'Gorgasalis Street', 'gorgasalis-st'),
        ('batumi', 'batumi-old-town', 'Piazza Square', 'piazza-square'),
        ('batumi', 'batumi-old-town', 'Ninoshvili Street', 'ninoshvili-st'),
        ('batumi', 'batumi-old-town', 'Zubalashvili Street', 'zubalashvili-st'),
        ('batumi', 'angisa', 'Angisa Street', 'angisa-st'),
        ('batumi', 'angisa', 'Khimshiashvili Street', 'khimshiashvili-st-angisa'),
        ('batumi', 'angisa', 'Kobaladze Street', 'kobaladze-st'),
        ('batumi', 'angisa', 'Luka Asatiani Street', 'asatiani-st'),
        ('batumi', 'angisa', 'Chavchavadze Street (Batumi)', 'chavchavadze-st-batumi'),
        ('batumi', 'angisa', 'Tbel Abuseridze Street', 'abuseridze-st'),
        ('batumi', 'khelvachauri', 'Khelvachauri Road', 'khelvachauri-road'),
        ('batumi', 'khelvachauri', 'Makhinjauri Road', 'makhinjauri-road'),
        ('batumi', 'khelvachauri', 'Green Cape approach', 'green-cape-road'),
        ('batumi', 'khelvachauri', 'Chakvi Road', 'chakvi-road'),
        ('batumi', 'khelvachauri', 'Gonio Road', 'gonio-road'),
        ('batumi', 'khelvachauri', 'Kvariati Road', 'kvariati-road'),
        ('kutaisi', 'kutaisi-centre', 'Rustaveli Avenue (Kutaisi)', 'rustaveli-ave-kutaisi'),
        ('kutaisi', 'kutaisi-centre', 'Tsisperqantsela Street', 'tsisperqantsela-st'),
        ('kutaisi', 'kutaisi-centre', 'Niko Sulkhanishvili Street', 'sulkhanishvili-st'),
        ('kutaisi', 'kutaisi-centre', 'Paliashvili Street', 'paliashvili-st-kutaisi'),
        ('kutaisi', 'kutaisi-centre', 'Chavchavadze Street (Kutaisi)', 'chavchavadze-st-kutaisi'),
        ('kutaisi', 'kutaisi-centre', 'Besik Gabashvili Street', 'gabashvili-st'),
        ('kutaisi', 'bagrati', 'Bagrati Street', 'bagrati-st'),
        ('kutaisi', 'bagrati', 'Ukimerioni Street', 'ukimerioni-st'),
        ('kutaisi', 'bagrati', 'Gegeshidze Street', 'gegeshidze-st'),
        ('kutaisi', 'bagrati', 'Gogebashvili Street', 'gogebashvili-st'),
        ('kutaisi', 'bagrati', 'Balakhvari Street', 'balakhvari-st'),
        ('kutaisi', 'bagrati', 'Rioni embankment', 'rioni-embankment'),
        ('kutaisi', 'gelati-road', 'Gelati Monastery Road', 'gelati-monastery-road'),
        ('kutaisi', 'gelati-road', 'Kutaisi–Tskaltubo Road', 'kutaisi-tskaltubo-road'),
        ('kutaisi', 'gelati-road', 'Navykhino Street', 'navykhino-st'),
        ('kutaisi', 'gelati-road', 'Gelati village road', 'gelati-village-road'),
        ('kutaisi', 'gelati-road', 'Baghdati Highway', 'baghdati-hwy'),
        ('kutaisi', 'gelati-road', 'Rioni bridge approach', 'rioni-bridge-approach')
) AS x(city_slug, area_slug, street_name, street_slug)
INNER JOIN cities AS c ON c.slug = x.city_slug
INNER JOIN areas AS a ON a.city_id = c.id AND a.slug = x.area_slug;

-- ---------------------------------------------------------------------------
-- Catalog — realistic plumbing trade lines (GEL pricing in guides below)
-- ---------------------------------------------------------------------------
INSERT INTO service_categories (name, slug, description, sort_order, is_active)
SELECT name, slug, description, row_number() OVER (ORDER BY ord), true
FROM (
    VALUES
        (1, 'Emergency burst pipe', 'emergency-burst-pipe', 'Rupture or major leak; same-day stop and repair'),
        (2, 'Kitchen sink leak', 'kitchen-sink-leak', 'Trap, supply lines, cartridge and basket strainer'),
        (3, 'Bathroom sink & faucet', 'bathroom-sink-faucet', 'Washbasin taps, pop-up waste, supply hoses'),
        (4, 'Toilet repair & install', 'toilet-repair-install', 'Flush valve, fill valve, wax ring, close-couple install'),
        (5, 'Shower & bathtub plumbing', 'shower-bathtub-plumbing', 'Mixer, diverter, drain and overflow'),
        (6, 'Electric water heater', 'electric-water-heater', 'Install, element/thermostat service, safety group'),
        (7, 'Gas water heater / boiler hookup', 'gas-water-heater-boiler', 'Licensed gas line to appliance; flue checks'),
        (8, 'Drain cleaning (snake)', 'drain-cleaning-snake', 'Kitchen and bathroom branch lines'),
        (9, 'Main sewer line', 'main-sewer-line', 'Camera inspection, auger, external cleanout'),
        (10, 'Hydro-jetting', 'hydro-jetting', 'High-pressure cleaning for grease and scale'),
        (11, 'Radiator & heating circuit', 'radiator-heating-circuit', 'Bleed, valve replace, balancing'),
        (12, 'Underfloor heating manifold', 'underfloor-heating-manifold', 'Actuators, pumps, mixing valve service'),
        (13, 'Water meter replacement', 'water-meter-replacement', 'Utility-compliant meter swap and sealing'),
        (14, 'Pump (booster / well)', 'pump-booster-well', 'Pressure tank, controller, submersible service'),
        (15, 'Backflow preventer', 'backflow-preventer', 'RPZ install and annual test coordination'),
        (16, 'Whole-house re-pipe (copper)', 'repipe-copper', 'Full or partial copper replacement'),
        (17, 'PEX / multilayer re-pipe', 'repipe-pex', 'Manifold and home-run layouts'),
        (18, 'Pipe freeze protection', 'pipe-freeze-protection', 'Insulation, heat trace on exposed risers'),
        (19, 'Sump pump', 'sump-pump', 'Basement/cellar pump install and check valve'),
        (20, 'Washing machine boxes', 'washing-machine-boxes', 'Hot/cold/drain rough-in and valves'),
        (21, 'Dishwasher connection', 'dishwasher-connection', 'Supply, drain high loop, air gap'),
        (22, 'Garbage disposal', 'garbage-disposal', 'Install and jam/reset service'),
        (23, 'Outdoor hose bib / tap', 'outdoor-hose-bib', 'Freeze-proof sillcock and vacuum break'),
        (24, 'Water filter (under-sink)', 'water-filter-under-sink', 'Carbon block, remineralization, change-out'),
        (25, 'Whole-house filtration', 'whole-house-filtration', 'Sediment + carbon + UV pre-plumbing'),
        (26, 'Water softener', 'water-softener', 'Ion exchange brine tank and bypass'),
        (27, 'Leak detection (thermal / acoustic)', 'leak-detection', 'Hidden leak locate before opening walls'),
        (28, 'Ceiling stain investigation', 'ceiling-stain-investigation', 'Trace upstairs/downstairs plumbing'),
        (29, 'Low water pressure diagnosis', 'low-water-pressure-diagnosis', 'PRV, restriction, shared riser issues'),
        (30, 'Hammer / noise in pipes', 'water-hammer-noise', 'Arrestors, securing, pressure spikes'),
        (31, 'Septic tank inlet / outlet', 'septic-tank-io', 'Baffle repair coordination with civil'),
        (32, 'Grease trap (commercial)', 'grease-trap-commercial', 'Kitchen interceptor pump-out prep'),
        (33, 'Restaurant pre-rinse', 'restaurant-pre-rinse', 'High-flow faucet and mixing valve'),
        (34, 'Boiler annual service', 'boiler-annual-service', 'Burner, exchanger flush, safety'),
        (35, 'Expansion vessel', 'expansion-vessel', 'Heating and DHW pressure vessel sizing/replace'),
        (36, 'Thermostatic mixing valve (TMV)', 'thermostatic-mixing-valve', 'Anti-scald for public/bath'),
        (37, 'Bidet / washlet install', 'bidet-washlet-install', 'Seat, GFCI circuit coordination'),
        (38, 'Ice maker line', 'ice-maker-line', '1/4" stop and saddle or box'),
        (39, 'Coffee machine water line', 'coffee-machine-water-line', 'Filtered line to commercial espresso'),
        (40, 'Roof tank & downpipe', 'roof-tank-downpipe', 'Header tank, overflow, roof drains'),
        (41, 'Storm drain tie-in', 'storm-drain-tie-in', 'Yard drainage to municipal where allowed'),
        (42, 'CCTV drain survey', 'cctv-drain-survey', 'Recorded inspection for insurance/sale'),
        (43, 'Pipe locating', 'pipe-locating', 'Metal/plastic line trace before dig'),
        (44, 'Civil coordination (excavation)', 'civil-excavation-coord', 'Trench to main with permits'),
        (45, 'Polyethylene water service', 'polyethylene-water-service', 'PE pipe fusion to building entry'),
        (46, 'Cast iron stack repair', 'cast-iron-stack-repair', 'Spot repair or Fernco transitions'),
        (47, 'PVC / DWV replacement', 'pvc-dwv-replacement', 'Waste and vent reconfiguration'),
        (48, 'Commercial restroom rough-in', 'commercial-restroom-rough-in', 'Battery of WC and lavs'),
        (49, 'Medical / lab taps', 'medical-lab-taps', 'Deionized water points coordination'),
        (50, 'After-hours emergency callout', 'after-hours-emergency', 'Night/weekend premium; Tbilisi metro response')
) AS t(ord, name, slug, description);

-- ---------------------------------------------------------------------------
-- Identity (one password hash for all rows)
-- ---------------------------------------------------------------------------
INSERT INTO users (email, password_hash, role, user_status, is_email_verified)
SELECT
    ('seed_client_' || g || '@dev.local')::citext,
    '$argon2id$v=19$m=19456,t=3,p=1$q3n66uplbcwM7cCIM/ed7Q$Hgnkr1nyUTgzXiAOTmRYU/+NvfYy9cs/SjHLedfAPuI',
    'client'::user_role,
    'active'::user_status,
    true
FROM generate_series(1, 60) AS g;

INSERT INTO users (email, password_hash, role, user_status, is_email_verified)
SELECT
    ('seed_plumber_' || g || '@dev.local')::citext,
    '$argon2id$v=19$m=19456,t=3,p=1$q3n66uplbcwM7cCIM/ed7Q$Hgnkr1nyUTgzXiAOTmRYU/+NvfYy9cs/SjHLedfAPuI',
    'plumber'::user_role,
    'active'::user_status,
    true
FROM generate_series(1, 20) AS g;

INSERT INTO users (email, password_hash, role, user_status, is_email_verified)
SELECT
    ('seed_admin_' || g || '@dev.local')::citext,
    '$argon2id$v=19$m=19456,t=3,p=1$q3n66uplbcwM7cCIM/ed7Q$Hgnkr1nyUTgzXiAOTmRYU/+NvfYy9cs/SjHLedfAPuI',
    'admin'::user_role,
    'active'::user_status,
    true
FROM generate_series(1, 3) AS g;

-- ---------------------------------------------------------------------------
-- Plumber profiles (one per plumber user; geography aligned via streets)
-- ---------------------------------------------------------------------------
INSERT INTO plumber_profiles (
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
    service_radius_km
)
SELECT
    u.id,
    (ARRAY[
        'Giorgi', 'Nino', 'Luka', 'Tamar', 'Davit', 'Mariam', 'Nikoloz', 'Salome', 'Andro', 'Ana',
        'Irakli', 'Ketevan', 'Zurab', 'Eka', 'Levan', 'Sopho', 'Giga', 'Nana', 'Vakhtang', 'Tina',
        'Besarion', 'Natia', 'Shalva', 'Dea', 'Sandro', 'Lamara', 'Badri', 'Lia', 'Gocha', 'Manana'
    ])[1 + mod(u.pn - 1, 30)]
    || ' '
    || (ARRAY[
        'Berishvili', 'Kapanadze', 'Maisuradze', 'Gelashvili', 'Lomidze', 'Tsiklauri', 'Kvlividze', 'Janelidze'
    ])[1 + mod(u.pn - 1, 8)],
    '+9955'
    || (ARRAY['77', '91', '93', '95'])[1 + mod(u.pn - 1, 4)]
    || lpad(mod(u.pn * 7193, 1000000)::text, 6, '0'),
    LEAST(25, GREATEST(1, (u.pn % 15) + 1)),
    ((u.pn % 5) <> 0),
    ((u.pn % 3) = 0),
    ((u.pn % 2) = 0),
    g.city_id,
    g.area_id,
    g.street_id,
    CASE g.city_slug
        WHEN 'tbilisi' THEN 41.62 + (mod(abs(hashtext(g.street_id::text || u.id::text)), 155)::double precision) / 1000.0
        WHEN 'batumi' THEN 41.615 + (mod(abs(hashtext(g.street_id::text || u.id::text)), 45)::double precision) / 1000.0
        WHEN 'kutaisi' THEN 42.255 + (mod(abs(hashtext(g.street_id::text || u.id::text)), 55)::double precision) / 1000.0
        ELSE 41.69
    END,
    CASE g.city_slug
        WHEN 'tbilisi' THEN 44.72 + (mod(abs(hashtext(g.street_id::text || u.id::text || 'lng')), 148)::double precision) / 1000.0
        WHEN 'batumi' THEN 41.635 + (mod(abs(hashtext(g.street_id::text || u.id::text || 'lng')), 38)::double precision) / 1000.0
        WHEN 'kutaisi' THEN 42.68 + (mod(abs(hashtext(g.street_id::text || u.id::text || 'lng')), 42)::double precision) / 1000.0
        ELSE 44.8
    END,
    (5 + (u.pn % 15))::numeric
FROM (
    SELECT id, row_number() OVER (ORDER BY email) AS pn
    FROM users
    WHERE role = 'plumber'::user_role
) AS u
INNER JOIN LATERAL (
    SELECT s.city_id, s.area_id, s.id AS street_id, ci.slug AS city_slug
    FROM streets AS s
    INNER JOIN cities AS ci ON ci.id = s.city_id
    WHERE s.area_id IS NOT NULL
    ORDER BY s.city_id, s.area_id, s.slug
    LIMIT 1
    OFFSET LEAST(
        ((u.pn - 1) % GREATEST((SELECT COUNT(*)::int FROM streets WHERE area_id IS NOT NULL), 1)),
        GREATEST((SELECT COUNT(*)::int FROM streets WHERE area_id IS NOT NULL), 1) - 1
    )
) AS g ON TRUE;

UPDATE plumber_profiles AS pp
SET
    approved_by = a.id,
    approved_at = now() - interval '1 day'
FROM (
    SELECT id
    FROM users
    WHERE role = 'admin'::user_role
    ORDER BY created_at
    LIMIT 1
) AS a
WHERE pp.is_approved = true;

-- ---------------------------------------------------------------------------
-- Client profiles
-- ---------------------------------------------------------------------------
INSERT INTO client_profiles (
    user_id,
    full_name,
    phone,
    default_city_id,
    default_area_id,
    default_street_id
)
SELECT
    u.id,
    (ARRAY[
        'Nino', 'Giorgi', 'Mariam', 'Luka', 'Ana', 'Davit', 'Salome', 'Irakli', 'Tamar', 'Nikoloz',
        'Ketevan', 'Levan', 'Sopho', 'Zurab', 'Andro', 'Eka', 'Giga', 'Natia', 'Vakhtang', 'Tina',
        'Besarion', 'Dea', 'Shalva', 'Lamara', 'Sandro', 'Badri', 'Lia', 'Gocha', 'Manana', 'Teona'
    ])[1 + mod(u.cn - 1, 30)]
    || ' '
    || (ARRAY[
        'Tsereteli', 'Shengelia', 'Chkheidze', 'Kobakhidze', 'Dvali', 'Shaverdashvili', 'Abuladze', 'Gvaramia'
    ])[1 + mod(u.cn - 1, 8)],
    '+9955'
    || (ARRAY['98', '99', '55', '57'])[1 + mod(u.cn - 1, 4)]
    || lpad(mod(u.cn * 5183, 1000000)::text, 6, '0'),
    g.city_id,
    g.area_id,
    g.street_id
FROM (
    SELECT id, row_number() OVER (ORDER BY email) AS cn
    FROM users
    WHERE role = 'client'::user_role
) AS u
INNER JOIN LATERAL (
    SELECT s.city_id, s.area_id, s.id AS street_id
    FROM streets AS s
    WHERE s.area_id IS NOT NULL
    ORDER BY s.city_id, s.area_id, s.slug
    LIMIT 1
    OFFSET LEAST(
        ((u.cn - 1) % GREATEST((SELECT COUNT(*)::int FROM streets WHERE area_id IS NOT NULL), 1)),
        GREATEST((SELECT COUNT(*)::int FROM streets WHERE area_id IS NOT NULL), 1) - 1
    )
) AS g ON TRUE;

-- ---------------------------------------------------------------------------
-- Price guides: global (category-only) + city + sparse area rows
-- ---------------------------------------------------------------------------
INSERT INTO service_price_guides (
    service_category_id,
    city_id,
    area_id,
    min_price,
    max_price,
    currency,
    estimated_duration_minutes,
    is_emergency_supported
)
SELECT
    id,
    NULL,
    NULL,
    25::numeric,
    180::numeric,
    'GEL',
    90,
    false
FROM service_categories;

INSERT INTO service_price_guides (
    service_category_id,
    city_id,
    area_id,
    min_price,
    max_price,
    currency,
    estimated_duration_minutes,
    is_emergency_supported
)
SELECT
    sc.id,
    c.id,
    NULL,
    LEAST(v.a, v.b),
    GREATEST(v.a, v.b) + 15::numeric,
    'GEL',
    30 + mod(abs(hashtext(sc.id::text || c.id::text || 'd')), 90),
    mod(abs(hashtext(sc.id::text || c.id::text || 'e')), 7) = 0
FROM service_categories AS sc
CROSS JOIN cities AS c
CROSS JOIN LATERAL (
    SELECT
        (40 + mod(abs(hashtext(sc.id::text || c.id::text)), 80))::numeric AS a,
        (90 + mod(abs(hashtext(c.id::text || sc.id::text)), 100))::numeric AS b
) AS v;

INSERT INTO service_price_guides (
    service_category_id,
    city_id,
    area_id,
    min_price,
    max_price,
    currency,
    estimated_duration_minutes,
    is_emergency_supported
)
SELECT
    sc.id,
    a.city_id,
    a.id,
    LEAST(v.x, v.y),
    GREATEST(v.x, v.y) + 20::numeric,
    'GEL',
    40 + mod(abs(hashtext(sc.id::text || a.id::text)), 100),
    false
FROM service_categories AS sc
INNER JOIN areas AS a ON mod(abs(hashtext(sc.id::text || a.id::text)), 11) = 0
CROSS JOIN LATERAL (
    SELECT
        (55 + mod(abs(hashtext(sc.id::text)), 70))::numeric AS x,
        (120 + mod(abs(hashtext(a.id::text)), 90))::numeric AS y
) AS v;

-- ---------------------------------------------------------------------------
-- Capabilities
-- ---------------------------------------------------------------------------
INSERT INTO plumber_services (plumber_id, service_category_id)
SELECT p.id, c.id
FROM plumber_profiles AS p
CROSS JOIN service_categories AS c
WHERE mod(abs(hashtext(p.id::text || c.id::text)), 10) < 6;

INSERT INTO plumber_service_areas (plumber_id, city_id, area_id)
SELECT id, current_city_id, NULL
FROM plumber_profiles
WHERE current_city_id IS NOT NULL;

INSERT INTO plumber_service_areas (plumber_id, city_id, area_id)
SELECT
    p.id,
    la.city_id,
    la.area_id
FROM plumber_profiles AS p
INNER JOIN LATERAL (
    SELECT a.city_id, a.id AS area_id
    FROM areas AS a
    WHERE a.city_id = p.current_city_id
      AND a.id IS DISTINCT FROM p.current_area_id
    ORDER BY a.slug
    LIMIT 2
) AS la ON TRUE
WHERE p.current_city_id IS NOT NULL;

-- ---------------------------------------------------------------------------
-- Plumber status history
-- ---------------------------------------------------------------------------
INSERT INTO plumber_status_history (plumber_id, status_type, meta, created_at)
SELECT
    p.id,
    (ARRAY['online', 'offline', 'available', 'unavailable']::plumber_status_type[])[1 + mod(g, 4)],
    jsonb_build_object(
        'seed_row', g,
        'source', 'dev_seed_comprehensive',
        'region', 'Georgia',
        'note', 'Synthetic telemetry for Tbilisi/Batumi/Kutaisi dispatch testing'
    ),
    now() - (g * interval '90 minutes')
FROM plumber_profiles AS p
CROSS JOIN generate_series(1, 18) AS g;

-- ---------------------------------------------------------------------------
-- Orders (clients only; geo from streets; plumbers by plumber_profiles.id)
-- ---------------------------------------------------------------------------
INSERT INTO orders (
    client_id,
    assigned_plumber_id,
    service_category_id,
    city_id,
    area_id,
    street_id,
    address_line,
    lat,
    lng,
    description,
    urgency,
    status,
    estimated_price_min,
    estimated_price_max,
    requested_at
)
SELECT
    cl.id,
    CASE
        WHEN mod(s.g, 10) >= 6 THEN (
            SELECT id
            FROM plumber_profiles
            ORDER BY id
            LIMIT 1
            OFFSET LEAST(
                mod(s.g * 3, GREATEST((SELECT COUNT(*)::int FROM plumber_profiles), 1)),
                GREATEST((SELECT COUNT(*)::int FROM plumber_profiles), 1) - 1
            )
        )
        ELSE NULL
    END,
    cat.id,
    st.city_id,
    st.area_id,
    st.street_id,
    st.city_nm || ' — ' || st.street_nm || ', № ' || (1 + mod(s.g, 220))::text,
    CASE st.city_slug
        WHEN 'tbilisi' THEN 41.62 + (mod(abs(hashtext(st.street_id::text || s.g::text)), 155)::double precision) / 1000.0
        WHEN 'batumi' THEN 41.615 + (mod(abs(hashtext(st.street_id::text || s.g::text)), 45)::double precision) / 1000.0
        WHEN 'kutaisi' THEN 42.255 + (mod(abs(hashtext(st.street_id::text || s.g::text)), 55)::double precision) / 1000.0
        ELSE 41.69
    END,
    CASE st.city_slug
        WHEN 'tbilisi' THEN 44.72 + (mod(abs(hashtext(st.street_id::text || 'o' || s.g::text)), 148)::double precision) / 1000.0
        WHEN 'batumi' THEN 41.62 + (mod(abs(hashtext(st.street_id::text || 'o' || s.g::text)), 50)::double precision) / 1000.0
        WHEN 'kutaisi' THEN 42.68 + (mod(abs(hashtext(st.street_id::text || 'o' || s.g::text)), 42)::double precision) / 1000.0
        ELSE 44.8
    END,
    (ARRAY[
        'სამზარეულოში წყლის ლაქა; სასწრაფოდ სჭირდება ოსტატი',
        'აბაზანაში მილის გაცივება — ტემპერატურა ვარდება',
        'უნიტაზის გამოძახება უწყვეტად; სავარაუდოდ flush valve',
        'სარდაფში აუზი; საჭიროა pump და check valve',
        'Kitchen sink trap leak; need same-day visit in apartment',
        'Electric boiler shows E1 error; no hot water',
        'Main shutoff stiff; replace valve before renovation',
        'Shower diverter stuck; only tub spout works',
        'Low pressure on cold line only; rest of flat OK',
        'Washing machine install — need hot/cold boxes',
        'Restaurant grease line slow drain after service',
        'Office WC battery — three cisterns running',
        'Roof tank overflow pipe dripping',
        'Hydro test after re-pipe; small weep at joint',
        'Batumi rental — guest reported dripping ceiling'
    ])[1 + mod(s.g - 1, 15)],
    (ARRAY['normal', 'urgent', 'emergency']::order_urgency[])[1 + mod(s.g, 3)],
    CASE mod(s.g, 10)
        WHEN 0 THEN 'searching'::order_status
        WHEN 1 THEN 'searching'::order_status
        WHEN 2 THEN 'searching'::order_status
        WHEN 3 THEN 'searching'::order_status
        WHEN 4 THEN 'dispatched'::order_status
        WHEN 5 THEN 'dispatched'::order_status
        WHEN 6 THEN 'accepted'::order_status
        WHEN 7 THEN 'accepted'::order_status
        WHEN 8 THEN 'in_progress'::order_status
        ELSE 'completed'::order_status
    END,
    LEAST(pr.lo, pr.hi),
    GREATEST(pr.lo, pr.hi),
    now() - (s.g * interval '45 minutes')
FROM generate_series(1, 120) AS s(g)
INNER JOIN (
    SELECT
        id,
        row_number() OVER (ORDER BY created_at) AS rn
    FROM users
    WHERE role = 'client'::user_role
) AS cl ON cl.rn = 1 + mod((s.g - 1) * 11, (SELECT COUNT(*)::int FROM users WHERE role = 'client'::user_role))
INNER JOIN LATERAL (
    SELECT id
    FROM service_categories
    ORDER BY slug
    LIMIT 1
    OFFSET LEAST(
        mod(s.g * 5, (SELECT COUNT(*)::int FROM service_categories)),
        (SELECT COUNT(*)::int FROM service_categories) - 1
    )
) AS cat ON TRUE
INNER JOIN LATERAL (
    SELECT
        str.city_id,
        str.area_id,
        str.id AS street_id,
        str.name AS street_nm,
        ci.name AS city_nm,
        ci.slug AS city_slug
    FROM streets AS str
    INNER JOIN cities AS ci ON ci.id = str.city_id
    WHERE str.area_id IS NOT NULL
    ORDER BY str.city_id, str.area_id, str.slug
    LIMIT 1
    OFFSET LEAST(
        mod(s.g * 13, GREATEST((SELECT COUNT(*)::int FROM streets WHERE area_id IS NOT NULL), 1)),
        GREATEST((SELECT COUNT(*)::int FROM streets WHERE area_id IS NOT NULL), 1) - 1
    )
) AS st ON TRUE
CROSS JOIN LATERAL (
    SELECT
        (30 + mod(s.g * 13, 200))::numeric AS lo,
        (50 + mod(s.g * 17, 250))::numeric AS hi
) AS pr;

-- ---------------------------------------------------------------------------
-- Dispatches (non-searching orders; distinct plumbers per rank)
-- ---------------------------------------------------------------------------
INSERT INTO order_dispatches (order_id, plumber_id, dispatch_rank, status, sent_at, responded_at)
SELECT
    o.id,
    np.id,
    rnk.rnk::smallint,
    CASE
        WHEN rnk.rnk = 1
            AND o.status IN (
                'accepted'::order_status,
                'in_progress'::order_status,
                'completed'::order_status
            )
            THEN 'accepted'::dispatch_status
        WHEN rnk.rnk = 1 AND o.status = 'dispatched'::order_status THEN 'viewed'::dispatch_status
        ELSE (
            ARRAY['sent', 'rejected', 'expired']::dispatch_status[]
        )[1 + mod(abs(hashtext(o.id::text || rnk.rnk::text)), 3)]
    END,
    o.requested_at + (rnk.rnk * interval '2 minutes'),
    CASE
        WHEN rnk.rnk <= 2 THEN o.requested_at + ((rnk.rnk + 5) * interval '1 minute')
        ELSE NULL
    END
FROM orders AS o
CROSS JOIN generate_series(1, 3) AS rnk(rnk)
INNER JOIN (
    SELECT id, row_number() OVER (ORDER BY user_id) AS rn
    FROM plumber_profiles
) AS np ON np.rn = rnk.rnk
WHERE o.status IN (
    'dispatched'::order_status,
    'accepted'::order_status,
    'in_progress'::order_status,
    'completed'::order_status
);

-- ---------------------------------------------------------------------------
-- Admin audit (system rows + optional rows attributed to first admin)
-- ---------------------------------------------------------------------------
INSERT INTO admin_audit_logs (admin_id, action, entity_type, entity_id, meta, created_at)
SELECT
    NULL,
    (ARRAY[
        'plumber.approved',
        'plumber.blocked',
        'plumber.unblocked',
        'city.created',
        'city.updated',
        'service_category.updated',
        'service_category.created',
        'order.cancelled',
        'user.role_changed',
        'system.seed',
        'auth.impersonation_started',
        'geography.area_deactivated',
        'dispatch.config_changed',
        'review.flagged',
        'price_guide.adjusted',
        'client_profile.viewed'
    ])[1 + ((g - 1) % 15)],
    (ARRAY[
        'plumber_profile',
        'city',
        'service_category',
        'order',
        'user',
        'area',
        'street',
        'dispatch_config',
        'review',
        'service_price_guide',
        'client_profile',
        'session',
        'api_key',
        'feature_flag',
        'bulk_job'
    ])[1 + ((g - 1) % 14)],
    CASE
        WHEN g % 4 = 0 THEN 'slug-entity-' || g::text
        WHEN g % 4 = 1 THEN gen_random_uuid()::text
        WHEN g % 4 = 2 THEN 'composite:' || ((g - 1) % 999)::text || ':v2'
        ELSE 'natural-key-' || ((g * 17) % 100000)::text
    END,
    jsonb_build_object(
        'source', (ARRAY['admin_ui', 'api', 'system'])[1 + ((g - 1) % 3)],
        'ip',
        '192.168.'
        || ((((g - 1) / 100) % 255) + 1)::text
        || '.'
        || (((g - 1) % 254) + 1)::text,
        'user_agent', 'TbilisiDevBot/' || (1 + (g - 1) % 5)::text || '.0 (GE comprehensive seed)',
        'request_id', 'req_' || md5(g::text || 'comp'),
        'correlation_id', 'cor_' || substr(md5(((g * 7919))::text), 1, 12),
        'diff', jsonb_build_object(
            'field',
            (ARRAY['is_active', 'sort_order', 'slug', 'rating_avg', 'address_line'])[1 + ((g - 1) % 5)],
            'before', to_jsonb('prev_'::text || g::text),
            'after', to_jsonb('next_'::text || (g + 7)::text)
        ),
        'note', 'Comprehensive dev seed (Georgia geography) row ' || g::text
    ),
    now() - (g * interval '2 hours')
FROM generate_series(1, 320) AS g;

INSERT INTO admin_audit_logs (admin_id, action, entity_type, entity_id, meta, created_at)
SELECT
    u.id,
    'admin.manual_override',
    'plumber_profile',
    gen_random_uuid()::text,
    jsonb_build_object(
        'source', 'admin_ui',
        'ip', '10.0.0.' || (50 + (g % 50))::text,
        'reason', 'Comprehensive seed attributed entry',
        'batch', g
    ),
    now() - (g * interval '6 hours')
FROM generate_series(1, 25) AS g
CROSS JOIN (
    SELECT id
    FROM users
    WHERE role = 'admin'::user_role
    ORDER BY created_at
    LIMIT 1
) AS u
WHERE EXISTS (SELECT 1 FROM users WHERE role = 'admin'::user_role);

COMMIT;
