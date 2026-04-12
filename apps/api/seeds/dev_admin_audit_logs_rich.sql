-- Dev-only: large admin_audit_logs dataset for dashboards, pagination, and time-range filters.
-- Run after migrations (and optionally after dev geography/category seeds).
-- Fake data only — no real PII. Re-run anytime: TRUNCATE wipes and reloads.

TRUNCATE admin_audit_logs;

INSERT INTO admin_audit_logs (admin_id, action, entity_type, entity_id, meta, created_at)
SELECT
  NULL,
  (ARRAY[
    'plumber.approved','plumber.blocked','plumber.unblocked','city.created','city.updated',
    'service_category.updated','service_category.created','order.cancelled','user.role_changed',
    'system.seed','auth.impersonation_started','geography.area_deactivated','dispatch.config_changed',
    'review.flagged','price_guide.adjusted','client_profile.viewed'
  ])[1 + ((g - 1) % 15)],
  (ARRAY[
    'plumber_profile','city','service_category','order','user','area','street','dispatch_config',
    'review','service_price_guide','client_profile','session','api_key','feature_flag','bulk_job'
  ])[1 + ((g - 1) % 14)],
  CASE
    WHEN g % 4 = 0 THEN 'slug-entity-' || g::TEXT
    WHEN g % 4 = 1 THEN gen_random_uuid()::TEXT
    WHEN g % 4 = 2 THEN 'composite:' || ((g - 1) % 999)::TEXT || ':v2'
    ELSE 'natural-key-' || ((g * 17) % 100000)::TEXT
  END,
  jsonb_build_object(
    'source', (ARRAY['admin_ui','api','system'])[1 + ((g - 1) % 3)],
    'ip',
      '192.168.'
      || ((((g - 1) / 100) % 255) + 1)::TEXT
      || '.'
      || (((g - 1) % 254) + 1)::TEXT,
    'user_agent', 'PlumberDevBot/' || (1 + (g - 1) % 5)::TEXT || '.0 (FakeShell)',
    'request_id', 'req_' || md5(g::TEXT || 'salt'),
    'correlation_id', 'cor_' || substr(md5(((g * 7919))::TEXT), 1, 12),
    'diff', jsonb_build_object(
      'field',
        (ARRAY['is_active','sort_order','slug','rating_avg','address_line'])[1 + ((g - 1) % 5)],
      'before', to_jsonb('prev_'::TEXT || g::TEXT),
      'after', to_jsonb('next_'::TEXT || (g + 7)::TEXT)
    ),
    'note', 'Synthetic dev seed row ' || g::TEXT
  ),
  now() - (g * interval '2 hours')
FROM generate_series(1, 420) AS g;

-- Attributed rows (only if at least one admin user exists in DB).
INSERT INTO admin_audit_logs (admin_id, action, entity_type, entity_id, meta, created_at)
SELECT
  u.id,
  'admin.manual_override',
  'plumber_profile',
  gen_random_uuid()::TEXT,
  jsonb_build_object(
    'source', 'admin_ui',
    'ip', '10.0.0.' || (50 + (g % 50))::TEXT,
    'reason', 'Dev seed attributed entry',
    'batch', g
  ),
  now() - (g * interval '6 hours')
FROM generate_series(1, 30) AS g
CROSS JOIN (
  SELECT id FROM users WHERE role = 'admin' ORDER BY created_at ASC LIMIT 1
) u
WHERE EXISTS (SELECT 1 FROM users WHERE role = 'admin');
