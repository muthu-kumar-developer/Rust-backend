use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use axum::Json;
use sqlx::Error as SqlxErr;

#[derive(Debug)]
pub enum AppError {
    SqlxError(SqlxErr),
    NotFound(String),
    BadRequest(String),
}

impl From<SqlxErr> for AppError {
    fn from(err: SqlxErr) -> Self {
        AppError::SqlxError(err)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            // 404
            AppError::NotFound(msg) => (
                StatusCode::NOT_FOUND,
                Json(json!({ "error": msg })),
            )
                .into_response(),

            // 400
            AppError::BadRequest(msg) => (
                StatusCode::BAD_REQUEST,
                Json(json!({ "error": msg })),
            )
                .into_response(),

            // SQLx error → 500
            AppError::SqlxError(err) => {
                eprintln!("Database error: {:?}", err);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({ "error": "Database error" })),
                )
                    .into_response()
            }
        }
    }
}
