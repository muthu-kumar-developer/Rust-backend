use axum::{
    Router,
    routing::{get, put},
};
use sqlx::PgPool;

use crate::handler::{create_handler, create_user_handler, get_userid_handler, update_handler_user};

pub fn create_route(pool: PgPool) -> Router {
    Router::new()
        .route("/users", get(create_handler).post(create_user_handler))
        .route(
            "/users/:id",
            get(get_userid_handler).put(update_handler_user)
        )
        .with_state(pool)
}

