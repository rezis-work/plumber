-- Reverse order: no table columns reference these types yet (user_status column dropped in prior down of next migration).

DROP TYPE IF EXISTS plumber_status_type;
DROP TYPE IF EXISTS dispatch_status;
DROP TYPE IF EXISTS order_status;
DROP TYPE IF EXISTS order_urgency;
DROP TYPE IF EXISTS user_status;
