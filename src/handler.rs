use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use sqlx::PgPool;

use crate::model::{CreateUser, UpdateUser, UpdateUsers, User};
use crate::{error::AppError, model::GetUsers};

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

pub async fn update_handler_user(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateUser>,
) -> Result<Json<User>, AppError> {
    // 1️⃣ Fetch existing user (only required fields)
    let existing = sqlx::query_as::<_, User>(
        r#"SELECT id, user_name, email, phone_number
           FROM users WHERE id = $1"#,
    )
    .bind(id)
    .fetch_optional(&pool)
    .await?;

    let existing = existing.ok_or(AppError::NotFound("User not found".into()))?;

    // 2️⃣ Merge values safely
    let updated_user_name = payload.user_name.or(existing.user_name);
    let updated_email = payload.email.or(existing.email);
    let updated_phone = payload.phone_number.or(existing.phone_number);

    // 3️⃣ Update user (only return User fields)
    let updated = sqlx::query_as::<_, User>(
        r#"
        UPDATE users
        SET 
            user_name = $1,
            email = $2,
            phone_number = $3
        WHERE id = $4
        RETURNING id, user_name, email, phone_number
    "#,
    )
    .bind(updated_user_name)
    .bind(updated_email)
    .bind(updated_phone)
    .bind(id)
    .fetch_one(&pool)
    .await?;

    Ok(Json(updated))
}

pub async fn get_users(State(pool): State<PgPool>) -> Result<Json<Vec<GetUsers>>, AppError> {
    let users = sqlx::query_as::<_, GetUsers>("SELECT * FROM public.usertables")
        .fetch_all(&pool)
        .await?;
    Ok(Json(users))
}

pub async fn update_user(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateUsers>,
) -> Result<Json<GetUsers>, AppError> {

    let updated_user = sqlx::query_as::<_, GetUsers>(
        r#"
        UPDATE usertables 
        SET name = $1, age = $2
        WHERE id = $3
        RETURNING *
        "#
    )
    .bind(payload.name)  // $1
    .bind(payload.age)   // $2
    .bind(id)            // $3
    .fetch_one(&pool)
    .await?;

    Ok(Json(updated_user))
}

