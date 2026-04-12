-- Phase 2 foundations: enums for user lifecycle, orders, dispatch, plumber telemetry (Implementation 003 §5).
-- No tables reference these yet except user_status in the following migration.
-- Conventions for later tables: UUID PKs via gen_random_uuid(), TIMESTAMPTZ created_at/updated_at,
-- soft-delete via deleted_at where specified; money as NUMERIC(12,2) + CHAR(3) currency when added.

CREATE TYPE user_status AS ENUM ('active', 'blocked', 'pending');

CREATE TYPE order_urgency AS ENUM ('normal', 'urgent', 'emergency');

CREATE TYPE order_status AS ENUM (
    'searching',
    'dispatched',
    'accepted',
    'in_progress',
    'completed',
    'cancelled',
    'expired'
);

CREATE TYPE dispatch_status AS ENUM (
    'sent',
    'viewed',
    'accepted',
    'rejected',
    'expired',
    'lost_race'
);

CREATE TYPE plumber_status_type AS ENUM ('online', 'offline', 'available', 'unavailable');
