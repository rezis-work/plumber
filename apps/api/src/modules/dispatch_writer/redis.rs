use serde_json::{json, Value};
use uuid::Uuid;

/// Optional Upstash Redis **REST** client (no TCP `redis` crate — avoids TLS feature matrix issues).
/// Keys follow spec §7: `order:dispatch:lock:{order_id}`, `dispatch:deadline:{dispatch_id}`.
#[derive(Clone)]
pub struct RedisDispatchHelper {
    client: reqwest::Client,
    /// e.g. `https://us1-xxx.upstash.io`
    endpoint: String,
    token: String,
}

impl RedisDispatchHelper {
    /// Enable when both `UPSTASH_REDIS_REST_URL` and `UPSTASH_REDIS_REST_TOKEN` are set.
    pub fn from_env() -> Option<Self> {
        let endpoint = std::env::var("UPSTASH_REDIS_REST_URL").ok()?;
        let token = std::env::var("UPSTASH_REDIS_REST_TOKEN").ok()?;
        if endpoint.trim().is_empty() || token.trim().is_empty() {
            return None;
        }
        let client = reqwest::Client::builder()
            .use_rustls_tls()
            .build()
            .ok()?;
        Some(Self {
            client,
            endpoint: endpoint.trim_end_matches('/').to_string(),
            token,
        })
    }

    async fn command(&self, parts: Value) -> Result<Value, DispatchRedisError> {
        let res = self
            .client
            .post(&self.endpoint)
            .bearer_auth(&self.token)
            .json(&parts)
            .send()
            .await
            .map_err(DispatchRedisError::Http)?;
        if !res.status().is_success() {
            let status = res.status();
            let text = res.text().await.unwrap_or_default();
            return Err(DispatchRedisError::Upstream(status, text));
        }
        res.json::<Value>()
            .await
            .map_err(DispatchRedisError::Http)
    }

    /// `SET key 1 NX EX 30` — returns `true` if lock acquired.
    pub async fn try_acquire_order_lock(&self, order_id: Uuid) -> Result<bool, DispatchRedisError> {
        let key = format!("order:dispatch:lock:{order_id}");
        let v = self
            .command(json!(["SET", key, "1", "NX", "EX", 30]))
            .await?;
        Ok(parse_set_nx_ok(&v))
    }

    pub async fn release_order_lock(&self, order_id: Uuid) -> Result<(), DispatchRedisError> {
        let key = format!("order:dispatch:lock:{order_id}");
        let _ = self.command(json!(["DEL", key])).await?;
        Ok(())
    }

    pub async fn set_dispatch_deadline(
        &self,
        dispatch_id: Uuid,
        ttl_secs: u64,
    ) -> Result<(), DispatchRedisError> {
        let key = format!("dispatch:deadline:{dispatch_id}");
        let _ = self
            .command(json!(["SET", key, "1", "EX", ttl_secs]))
            .await?;
        Ok(())
    }

    /// Spec §7: optional read-through cache for `token_balance`; invalidate after ledger writes.
    pub async fn invalidate_plumber_token_cache(
        &self,
        plumber_id: Uuid,
    ) -> Result<(), DispatchRedisError> {
        let key = format!("plumber:tokens:{plumber_id}");
        let _ = self.command(json!(["DEL", key])).await?;
        Ok(())
    }
}

#[derive(Debug, thiserror::Error)]
pub enum DispatchRedisError {
    #[error("http error: {0}")]
    Http(#[from] reqwest::Error),
    #[error("upstash error {0}: {1}")]
    Upstream(reqwest::StatusCode, String),
}

/// Upstash REST: `{"result":"OK"}` or `{"result":null}` for `SET ... NX`.
fn parse_set_nx_ok(response: &Value) -> bool {
    matches!(response.get("result"), Some(Value::String(s)) if s == "OK")
}
