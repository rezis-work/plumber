use serde_json::{json, Value};
use uuid::Uuid;

const DEFAULT_DISPATCH_QUEUE_LIST_KEY: &str = "dispatch:queue";

/// Optional Upstash Redis **REST** client (no TCP `redis` crate — avoids TLS feature matrix issues).
/// Keys follow spec §7: `order:dispatch:lock:{order_id}`, `dispatch:deadline:{dispatch_id}`.
#[derive(Clone)]
pub struct RedisDispatchHelper {
    client: reqwest::Client,
    /// e.g. `https://us1-xxx.upstash.io`
    endpoint: String,
    token: String,
    /// LIST key for `RPUSH` / `LPOP` (§12.7 `DISPATCH_QUEUE_REDIS_KEY`, default `dispatch:queue`).
    queue_list_key: String,
}

impl RedisDispatchHelper {
    /// Enable when both `UPSTASH_REDIS_REST_URL` and `UPSTASH_REDIS_REST_TOKEN` are set.
    pub fn from_env() -> Option<Self> {
        let endpoint = std::env::var("UPSTASH_REDIS_REST_URL").ok()?;
        let token = std::env::var("UPSTASH_REDIS_REST_TOKEN").ok()?;
        if endpoint.trim().is_empty() || token.trim().is_empty() {
            return None;
        }
        let queue_list_key = std::env::var("DISPATCH_QUEUE_REDIS_KEY")
            .ok()
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .unwrap_or_else(|| DEFAULT_DISPATCH_QUEUE_LIST_KEY.to_string());
        let client = reqwest::Client::builder()
            .use_rustls_tls()
            .build()
            .ok()?;
        Some(Self {
            client,
            endpoint: endpoint.trim_end_matches('/').to_string(),
            token,
            queue_list_key,
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

    /// `RPUSH dispatch:queue <order_id>` — best-effort after `orders` commit; workers `BRPOP` this list.
    pub async fn rpush_dispatch_queue(&self, order_id: Uuid) -> Result<(), DispatchRedisError> {
        let _ = self
            .command(json!([
                "RPUSH",
                &self.queue_list_key,
                order_id.to_string()
            ]))
            .await?;
        Ok(())
    }

    /// `LPOP` on the configured dispatch queue list (Upstash REST; §12.5 uses this + sleep instead of `BRPOP`).
    pub async fn lpop_dispatch_queue(&self) -> Result<Option<Uuid>, DispatchRedisError> {
        let v = self
            .command(json!(["LPOP", &self.queue_list_key]))
            .await?;
        parse_lpop_uuid(&v)
    }
}

fn parse_lpop_uuid(response: &Value) -> Result<Option<Uuid>, DispatchRedisError> {
    match response.get("result") {
        None | Some(Value::Null) => Ok(None),
        Some(Value::String(s)) if s.is_empty() => Ok(None),
        Some(Value::String(s)) => Uuid::parse_str(s)
            .map(Some)
            .map_err(|_| DispatchRedisError::InvalidQueuePayload(s.clone())),
        Some(other) => Err(DispatchRedisError::InvalidQueuePayload(other.to_string())),
    }
}

#[derive(Debug, thiserror::Error)]
pub enum DispatchRedisError {
    #[error("http error: {0}")]
    Http(#[from] reqwest::Error),
    #[error("upstash error {0}: {1}")]
    Upstream(reqwest::StatusCode, String),
    #[error("dispatch queue value is not a valid order UUID: {0}")]
    InvalidQueuePayload(String),
}

/// Upstash REST: `{"result":"OK"}` or `{"result":null}` for `SET ... NX`.
fn parse_set_nx_ok(response: &Value) -> bool {
    matches!(response.get("result"), Some(Value::String(s)) if s == "OK")
}
