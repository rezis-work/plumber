DROP TABLE IF EXISTS platform_settings;

ALTER TABLE plumber_profiles DROP CONSTRAINT IF EXISTS plumber_profiles_token_balance_non_negative;
ALTER TABLE plumber_profiles DROP COLUMN IF EXISTS token_balance;

DROP INDEX IF EXISTS idx_plumber_token_ledger_order_id_partial;
DROP INDEX IF EXISTS idx_plumber_token_ledger_plumber_created_at;
DROP TABLE IF EXISTS plumber_token_ledger;

DROP INDEX IF EXISTS idx_order_media_order_id_sort_order;
DROP TABLE IF EXISTS order_media;

DROP INDEX IF EXISTS idx_order_dispatches_order_id_offer_round;
ALTER TABLE order_dispatches DROP CONSTRAINT IF EXISTS order_dispatches_offer_round_positive;
ALTER TABLE order_dispatches DROP COLUMN IF EXISTS offer_round;

DROP TYPE IF EXISTS token_ledger_reason;
