use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};

#[allow(dead_code)]
#[derive(Debug)]
pub enum AppError {
    Unauthorized,
    Forbidden,
    NotFound,
    Conflict,
    BadRequest,
    SqlxError(sqlx::Error),
    InternalServerError(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::Unauthorized => (StatusCode::UNAUTHORIZED, "No autorizado".to_string()),
            AppError::Forbidden    => (StatusCode::FORBIDDEN, "Prohibido".to_string()),
            AppError::NotFound     => (StatusCode::NOT_FOUND, "No encontrado".to_string()),
            AppError::Conflict     => (StatusCode::CONFLICT, "Ya existe".to_string()),
            AppError::BadRequest   => (StatusCode::BAD_REQUEST, "Solicitud inválida".to_string()),
            AppError::SqlxError(e) => {
                tracing::error!("Error de base de datos: {e}");
                (StatusCode::INTERNAL_SERVER_ERROR, "Error interno".to_string())
            }
            AppError::InternalServerError(msg) => {
                tracing::error!("Error interno: {msg}");
                (StatusCode::INTERNAL_SERVER_ERROR, "Error interno".to_string())
            }
        };
        (status, Json(serde_json::json!({ "error": message }))).into_response()
    }
}

impl From<sqlx::Error> for AppError {
    fn from(e: sqlx::Error) -> Self {
        AppError::SqlxError(e)
    }
}