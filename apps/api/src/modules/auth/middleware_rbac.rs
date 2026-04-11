//! Role checks after [`super::middleware_access::require_access_token`] (Step 8).

use axum::extract::Request;
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};

use crate::modules::users::Role;

use super::auth_context::AuthContext;
use super::auth_forbidden::AuthForbidden;
use super::auth_unauthorized::AuthUnauthorized;

pub(crate) async fn check_exact_role(required: Role, req: Request, next: Next) -> Response {
    match req.extensions().get::<AuthContext>().copied() {
        None => AuthUnauthorized.into_response(),
        Some(ctx) if ctx.role != required => AuthForbidden.into_response(),
        Some(_) => next.run(req).await,
    }
}

pub(crate) async fn check_any_role(
    roles: &'static [Role],
    req: Request,
    next: Next,
) -> Response {
    match req.extensions().get::<AuthContext>().copied() {
        None => AuthUnauthorized.into_response(),
        Some(ctx) if !roles.contains(&ctx.role) => AuthForbidden.into_response(),
        Some(_) => next.run(req).await,
    }
}

/// Axum middleware layer: require exactly one [`Role`] after access-token middleware.
///
/// Use as `crate::require_role!(Role::Plumber)` (this crate) or `api::require_role!(...)` from binaries.
#[macro_export]
macro_rules! require_role {
    ($role:expr) => {{
        ::axum::middleware::from_fn::<_, (::axum::extract::Request,)>({
            let required = $role;
            move |req: ::axum::extract::Request, next: ::axum::middleware::Next| {
                $crate::modules::auth::middleware_rbac::check_exact_role(required, req, next)
            }
        })
    }};
}

/// Axum middleware layer: require any of `roles` (e.g. `&[Role::Plumber, Role::Admin]`).
#[macro_export]
macro_rules! require_any_role {
    ($roles:expr) => {{
        ::axum::middleware::from_fn::<_, (::axum::extract::Request,)>({
            let roles: &'static [$crate::modules::users::Role] = $roles;
            move |req: ::axum::extract::Request, next: ::axum::middleware::Next| {
                $crate::modules::auth::middleware_rbac::check_any_role(roles, req, next)
            }
        })
    }};
}
