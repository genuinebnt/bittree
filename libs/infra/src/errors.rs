use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::{Deserialize, Serialize};

pub enum ApiError {
    NotFound(String),
    Unauthorized,
    UnprocessableEntity(String),
    Validation(String),
    Conflict(String),
    Internal,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error: String,
    pub message: Option<String>,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            ApiError::NotFound(e) => (StatusCode::NOT_FOUND, Some(e.to_string())),
            ApiError::Validation(e) => (StatusCode::BAD_REQUEST, Some(e.to_string())),
            ApiError::Conflict(e) => (StatusCode::CONFLICT, Some(e.to_string())),
            ApiError::Unauthorized => (StatusCode::UNAUTHORIZED, None),
            ApiError::UnprocessableEntity(e) => {
                (StatusCode::UNPROCESSABLE_ENTITY, Some(e.to_string()))
            }
            _ => (StatusCode::INTERNAL_SERVER_ERROR, None),
        };

        let body = Json(ErrorResponse {
            error: status.to_string(),
            message,
        });

        (status, body).into_response()
    }
}
