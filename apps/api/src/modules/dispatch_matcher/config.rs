use chrono::Duration;

/// Defaults align with `platform_settings` seed in migration `20260210120014_od0_dispatch_tokens_platform.up.sql`.
#[derive(Debug, Clone)]
pub struct MatcherConfig {
    pub batch_size: usize,
    pub emergency_min_token_balance: i32,
    pub location_max_age: Duration,
    /// Offer TTL for Redis `dispatch:deadline:*` (minutes); matches `platform_settings` seed.
    pub dispatch_offer_ttl_minutes_normal: u64,
    pub dispatch_offer_ttl_minutes_emergency: u64,
    pub w_token: f64,
    pub w_rating: f64,
    pub w_distance: f64,
}

impl Default for MatcherConfig {
    fn default() -> Self {
        Self {
            batch_size: 3,
            emergency_min_token_balance: 10,
            location_max_age: Duration::minutes(15),
            dispatch_offer_ttl_minutes_normal: 30,
            dispatch_offer_ttl_minutes_emergency: 10,
            w_token: 1.0,
            w_rating: 1.0,
            w_distance: 1.0,
        }
    }
}
