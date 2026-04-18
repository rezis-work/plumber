//! OD-7: structured lifecycle logs (`target = "order_lifecycle"`) and Prometheus metrics.

use std::sync::{Once, OnceLock};

use metrics_exporter_prometheus::PrometheusBuilder;
use tracing_subscriber::EnvFilter;
use uuid::Uuid;

static TRACING_INIT: Once = Once::new();
static METRICS_INIT: Once = Once::new();
static PROMETHEUS: OnceLock<metrics_exporter_prometheus::PrometheusHandle> = OnceLock::new();

/// Default histogram buckets (seconds for time metrics; also used for round counts as coarse values).
const HISTOGRAM_BUCKETS: &[f64] = &[0.5, 1.0, 5.0, 15.0, 60.0, 300.0, 900.0, 3600.0];

/// Install `tracing_subscriber` once. Honors `RUST_LOG`; set `LOG_JSON=1` for JSON lines.
pub fn init_tracing() {
    TRACING_INIT.call_once(|| {
        let filter = EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| EnvFilter::new("info,api=info"));

        let json_logs = std::env::var("LOG_JSON").ok().as_deref() == Some("1");

        if json_logs {
            let _ = tracing_subscriber::fmt()
                .with_env_filter(filter)
                .json()
                .try_init();
        } else {
            let _ = tracing_subscriber::fmt()
                .with_env_filter(filter)
                .try_init();
        }
    });
}

/// Install Prometheus recorder and scrape handle once.
pub fn init_metrics() {
    METRICS_INIT.call_once(|| {
        let handle = PrometheusBuilder::new()
            .set_buckets(HISTOGRAM_BUCKETS)
            .expect("prometheus set_buckets")
            .install_recorder()
            .expect("install Prometheus metrics recorder");
        let _ = PROMETHEUS.set(handle);
    });
}

/// Prometheus exposition body (text format).
pub fn metrics_render() -> String {
    init_metrics();
    PROMETHEUS
        .get()
        .map(|h| h.render())
        .unwrap_or_else(|| "# metrics handle missing\n".to_string())
}

/// Structured lifecycle event: `order_id`, `transition`, optional `offer_round` / `plumber_id`.
pub fn log_order_transition(
    order_id: Uuid,
    transition: &'static str,
    offer_round: Option<i16>,
    plumber_id: Option<Uuid>,
) {
    tracing::info!(
        target = "order_lifecycle",
        %order_id,
        transition,
        offer_round = ?offer_round,
        plumber_id = ?plumber_id,
        "order_transition"
    );
}

pub fn log_expire_tick(expired_dispatch_rows: usize) {
    if expired_dispatch_rows == 0 {
        return;
    }
    tracing::info!(
        target = "order_lifecycle",
        transition = "dispatch_expired_batch",
        expired_dispatch_rows,
        "order_transition"
    );
}

pub fn record_time_to_first_offer_seconds(seconds: f64) {
    init_metrics();
    metrics::histogram!("plumber_order_time_to_first_offer_seconds").record(seconds.max(0.0));
}

pub fn record_time_to_accept_seconds(seconds: f64) {
    init_metrics();
    metrics::histogram!("plumber_order_time_to_accept_seconds").record(seconds.max(0.0));
}

pub fn record_dispatch_rounds_on_complete(max_offer_round: i16) {
    init_metrics();
    metrics::histogram!("plumber_order_dispatch_rounds").record(f64::from(max_offer_round));
}

pub fn record_token_grants(tokens: i32) {
    if tokens <= 0 {
        return;
    }
    init_metrics();
    metrics::counter!("plumber_token_grants_total").increment(tokens as u64);
}

/// Implementation 004 §12.8 — `POST /orders` RPUSH to `dispatch:queue` failed after DB commit.
pub fn record_dispatch_queue_rpush_failure() {
    init_metrics();
    metrics::counter!("dispatch_queue_rpush_failures_total").increment(1);
}

/// Implementation 004 §12.8 — snapshot of `dispatch_outbox` rows in **`pending`** (refreshed after reconcile).
pub fn set_dispatch_outbox_pending(count: u64) {
    init_metrics();
    metrics::gauge!("dispatch_outbox_pending").set(count as f64);
}

#[cfg(test)]
mod tests {
    #[test]
    fn metrics_init_and_record_does_not_panic() {
        super::init_metrics();
        super::record_token_grants(2);
        super::record_time_to_first_offer_seconds(1.5);
        super::record_dispatch_rounds_on_complete(3);
        assert!(!super::metrics_render().is_empty());
    }

    #[test]
    fn dispatch_metrics_record_and_render() {
        super::init_metrics();
        super::record_dispatch_queue_rpush_failure();
        super::set_dispatch_outbox_pending(7);
        let body = super::metrics_render();
        assert!(
            body.contains("dispatch_queue_rpush_failures_total"),
            "{body}"
        );
        assert!(body.contains("dispatch_outbox_pending"), "{body}");
    }
}
