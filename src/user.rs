use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;


#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub role: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewUser {
    pub username: String,
    pub email: String,
    pub password: String, // plain password (not stored in DB)
}
