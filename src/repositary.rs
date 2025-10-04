use crate::user::{User, NewUser};
use sqlx::PgPool;
use chrono::Utc;
use crate::security::hash_password;
use crate::errors::AppError; // your AppError enum

pub async fn create_user(pool: &PgPool, new_user: NewUser) -> Result<User, AppError> {
    // Hash password, propagate error with ?
    let password_hash = hash_password(&new_user.password)?;

    let user = sqlx::query_as::<_, User>(
        "INSERT INTO users (username, email, password_hash, role, created_at, updated_at)
         VALUES ($1, $2, $3, $4, $5, $6)
         RETURNING *"
    )
    .bind(&new_user.username)
    .bind(&new_user.email)
    .bind(&password_hash)
    .bind("user") // default role
    .bind(Utc::now())
    .bind(Utc::now())
    .fetch_one(pool)
    .await?;

    Ok(user)
}

pub async fn find_user_by_email(pool: &PgPool, email: &str) -> Result<User, AppError> {
    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE email = $1")
        .bind(email)
        .fetch_one(pool)
        .await?;
    Ok(user)
}
