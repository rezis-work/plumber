-- =============================================================================
-- DEV ONLY — DESTRUCTIVE. Never run against production or shared staging.
-- =============================================================================
-- Wipes Phase 2 domain data in FK-safe order (children before parents).
-- Also truncates refresh_tokens: all dev sessions are invalidated after this.
-- Prerequisites: schema applied (`sqlx migrate run`). Run BEFORE comprehensive seed.
-- =============================================================================

BEGIN;

TRUNCATE TABLE
    admin_audit_logs,
    plumber_token_ledger,
    order_media,
    order_dispatches,
    orders,
    plumber_status_history,
    plumber_service_areas,
    plumber_services,
    service_price_guides,
    client_profiles,
    refresh_tokens,
    plumber_profiles,
    users,
    streets,
    areas,
    cities,
    service_categories
RESTART IDENTITY;

COMMIT;
