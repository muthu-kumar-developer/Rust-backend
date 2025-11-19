use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: i32,
    pub user_name: Option<String>,
    pub email: Option<String>,
    pub phone_number: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUser {
    pub user_name: Option<String>,
    pub email: Option<String>,
    pub phone_number: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateUser{
    pub user_name:Option<String>,
    pub email:Option<String>,
    pub phone_number:Option<i64>
}
