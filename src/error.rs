use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use sqlx::Error as SqlxErr;

#[derive(Debug)]
pub enum AppError {
    SqlxError(SqlxErr),
}

// Convert sqlx::Error → AppError
impl From<SqlxErr> for AppError {
    fn from(err: SqlxErr) -> Self {
        AppError::SqlxError(err)
    }
}

// Convert AppError → HTTP Response
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            AppError::SqlxError(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Database error",
            )
                .into_response(),
        }
    }
}
