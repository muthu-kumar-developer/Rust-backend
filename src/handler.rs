use std::ops::BitAnd;

use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use sqlx::PgPool;

use crate::error::AppError;
use crate::model::{CreateUser, User};

pub async fn create_handler(State(pool): State<PgPool>) -> Result<Json<Vec<User>>, AppError> {
    let users = sqlx::query_as::<_, User>("SELECT * FROM users")
        .fetch_all(&pool)
        .await?;

    Ok(Json(users))
}

pub async fn create_user_handler(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateUser>,
) -> Result<(StatusCode, Json<User>), AppError> {
    let new_user = sqlx::query_as::<_, User>(
        r#"
        INSERT INTO users (user_name, email, phone_number)
        VALUES ($1, $2, $3)
        RETURNING id, user_name, email, phone_number
        "#,
    )
    .bind(payload.user_name)
    .bind(payload.email)
    .bind(payload.phone_number)
    .fetch_one(&pool)
    .await?;

    Ok((StatusCode::CREATED, Json(new_user)))
}

pub async fn get_userid_handler(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
) -> Result<Json<User>, AppError> {
    let getuser = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id=$1")
        .bind(id)
        .fetch_one(&pool)
        .await?;
    Ok(Json(getuser))
}
