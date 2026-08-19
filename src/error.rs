use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("no autorizado")]
    Unauthorized,

    #[error("prohibido")]
    Forbidden,

    #[error("no encontrado")]
    NotFound,

    #[error("conflicto: {0}")]
    Conflict(String),

    #[error("solicitud inválida: {0}")]
    BadRequest(String),

    #[error("error de validación: {0}")]
    Validation(String),

    #[error("error de base de datos: {0}")]
    SqlxError(#[from] sqlx::Error),

    #[error("error interno: {0}")]
    InternalServerError(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::Unauthorized => (StatusCode::UNAUTHORIZED, "no autorizado".to_string()),
            AppError::Forbidden => (StatusCode::FORBIDDEN, "prohibido".to_string()),
            AppError::NotFound => (StatusCode::NOT_FOUND, "no encontrado".to_string()),
            AppError::Conflict(msg) => (StatusCode::CONFLICT, msg),
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
            AppError::Validation(msg) => (StatusCode::UNPROCESSABLE_ENTITY, msg),
            AppError::SqlxError(sqlx::Error::RowNotFound) => {
                (StatusCode::NOT_FOUND, "recurso no encontrado".to_string())
            }
            AppError::SqlxError(e) => {
                tracing::error!(error = ?e, "error de base de datos");
                (StatusCode::INTERNAL_SERVER_ERROR, "error interno".to_string())
            }
            AppError::InternalServerError(msg) => {
                tracing::error!(error = %msg, "error interno");
                (StatusCode::INTERNAL_SERVER_ERROR, "error interno".to_string())
            }
        };

        (status, Json(serde_json::json!({ "error": message }))).into_response()
    }
}