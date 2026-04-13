//! PostgreSQL enum mirrors for Implementation 003 §5 (`20260210120000_phase2_domain_enums`).
//! `orders` — `20260210120011_orders`; `order_dispatches` — `20260210120012_order_dispatches`. `plumber_status_type` — `plumber_status_history` (`20260210120010_plumber_status_history`).

#![allow(dead_code)]

use serde::{Deserialize, Serialize};
use sqlx::Type;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Type, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
#[sqlx(type_name = "order_urgency", rename_all = "lowercase")]
pub enum OrderUrgency {
    Normal,
    Urgent,
    Emergency,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Type, Serialize)]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "order_status", rename_all = "snake_case")]
pub enum OrderStatus {
    Searching,
    Dispatched,
    Accepted,
    InProgress,
    Completed,
    Cancelled,
    Expired,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Type, Serialize)]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "dispatch_status", rename_all = "snake_case")]
pub enum DispatchStatus {
    Sent,
    Viewed,
    Accepted,
    Rejected,
    Expired,
    LostRace,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Type, Serialize)]
#[serde(rename_all = "lowercase")]
#[sqlx(type_name = "plumber_status_type", rename_all = "lowercase")]
pub enum PlumberStatusType {
    Online,
    Offline,
    Available,
    Unavailable,
}
