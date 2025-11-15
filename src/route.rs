use axum::{
    Router,
    routing::{get},
};
use sqlx::PgPool;

use crate::handler::{create_handler, create_user_handler, get_userid_handler};

pub fn create_route(pool: PgPool) -> Router {
    Router::new()
        .route("/users", get(create_handler).post(create_user_handler))
        .route("/users/:id", get(get_userid_handler))
        .with_state(pool)
}
