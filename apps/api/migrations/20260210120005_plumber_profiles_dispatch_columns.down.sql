-- Best-effort rollback of §6.3 columns (not for production data that depends on new fields).

DROP INDEX IF EXISTS idx_plumber_profiles_dispatch_city;
DROP INDEX IF EXISTS idx_plumber_profiles_last_location_updated_at;
DROP INDEX IF EXISTS idx_plumber_profiles_current_area_id;
DROP INDEX IF EXISTS idx_plumber_profiles_current_city_id;
DROP INDEX IF EXISTS idx_plumber_profiles_dispatch_ready_partial;

ALTER TABLE plumber_profiles DROP CONSTRAINT IF EXISTS plumber_profiles_service_radius_km_pos;
ALTER TABLE plumber_profiles DROP CONSTRAINT IF EXISTS plumber_profiles_rating_avg_range;

ALTER TABLE plumber_profiles
    DROP COLUMN IF EXISTS updated_at,
    DROP COLUMN IF EXISTS created_at,
    DROP COLUMN IF EXISTS cancelled_orders_count,
    DROP COLUMN IF EXISTS completed_orders_count,
    DROP COLUMN IF EXISTS rating_count,
    DROP COLUMN IF EXISTS rating_avg,
    DROP COLUMN IF EXISTS last_location_updated_at,
    DROP COLUMN IF EXISTS service_radius_km,
    DROP COLUMN IF EXISTS current_lng,
    DROP COLUMN IF EXISTS current_lat,
    DROP COLUMN IF EXISTS current_street_id,
    DROP COLUMN IF EXISTS current_area_id,
    DROP COLUMN IF EXISTS current_city_id,
    DROP COLUMN IF EXISTS is_available,
    DROP COLUMN IF EXISTS is_online,
    DROP COLUMN IF EXISTS approved_by,
    DROP COLUMN IF EXISTS approved_at,
    DROP COLUMN IF EXISTS is_approved,
    DROP COLUMN IF EXISTS avatar_url,
    DROP COLUMN IF EXISTS bio;

ALTER TABLE plumber_profiles RENAME COLUMN experience_years TO years_of_experience;
