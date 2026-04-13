use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;

#[derive(Debug)]
pub enum CreateOrderError {
    Validation { message: String },
    InvalidCategory,
    CategoryInactive,
    InvalidCity,
    CityInactive,
    InvalidArea,
    AreaNotInCity,
    AreaInactive,
    InvalidStreet,
    StreetNotInCity,
    StreetAreaMismatch,
    StreetInactive,
    Internal,
}

#[derive(Serialize)]
struct ErrorBody {
    error: &'static str,
    message: String,
}

impl IntoResponse for CreateOrderError {
    fn into_response(self) -> Response {
        match self {
            CreateOrderError::Validation { message } => (
                StatusCode::BAD_REQUEST,
                Json(ErrorBody {
                    error: "validation_error",
                    message,
                }),
            )
                .into_response(),
            CreateOrderError::InvalidCategory => (
                StatusCode::BAD_REQUEST,
                Json(ErrorBody {
                    error: "invalid_category",
                    message: "service category not found".to_string(),
                }),
            )
                .into_response(),
            CreateOrderError::CategoryInactive => (
                StatusCode::BAD_REQUEST,
                Json(ErrorBody {
                    error: "category_inactive",
                    message: "service category is not active".to_string(),
                }),
            )
                .into_response(),
            CreateOrderError::InvalidCity => (
                StatusCode::BAD_REQUEST,
                Json(ErrorBody {
                    error: "invalid_city",
                    message: "city not found".to_string(),
                }),
            )
                .into_response(),
            CreateOrderError::CityInactive => (
                StatusCode::BAD_REQUEST,
                Json(ErrorBody {
                    error: "city_inactive",
                    message: "city is not active".to_string(),
                }),
            )
                .into_response(),
            CreateOrderError::InvalidArea => (
                StatusCode::BAD_REQUEST,
                Json(ErrorBody {
                    error: "invalid_area",
                    message: "area not found".to_string(),
                }),
            )
                .into_response(),
            CreateOrderError::AreaNotInCity => (
                StatusCode::BAD_REQUEST,
                Json(ErrorBody {
                    error: "area_city_mismatch",
                    message: "area does not belong to the given city".to_string(),
                }),
            )
                .into_response(),
            CreateOrderError::AreaInactive => (
                StatusCode::BAD_REQUEST,
                Json(ErrorBody {
                    error: "area_inactive",
                    message: "area is not active".to_string(),
                }),
            )
                .into_response(),
            CreateOrderError::InvalidStreet => (
                StatusCode::BAD_REQUEST,
                Json(ErrorBody {
                    error: "invalid_street",
                    message: "street not found".to_string(),
                }),
            )
                .into_response(),
            CreateOrderError::StreetNotInCity => (
                StatusCode::BAD_REQUEST,
                Json(ErrorBody {
                    error: "street_city_mismatch",
                    message: "street does not belong to the given city".to_string(),
                }),
            )
                .into_response(),
            CreateOrderError::StreetAreaMismatch => (
                StatusCode::BAD_REQUEST,
                Json(ErrorBody {
                    error: "street_area_mismatch",
                    message: "street is not in the selected area".to_string(),
                }),
            )
                .into_response(),
            CreateOrderError::StreetInactive => (
                StatusCode::BAD_REQUEST,
                Json(ErrorBody {
                    error: "street_inactive",
                    message: "street is not active".to_string(),
                }),
            )
                .into_response(),
            CreateOrderError::Internal => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorBody {
                    error: "internal_error",
                    message: "something went wrong".to_string(),
                }),
            )
                .into_response(),
        }
    }
}

impl From<sqlx::Error> for CreateOrderError {
    fn from(e: sqlx::Error) -> Self {
        tracing::error!("create_order db error: {e}");
        CreateOrderError::Internal
    }
}
