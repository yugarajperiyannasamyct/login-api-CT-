use actix_web::{web, HttpResponse, Responder};
use sqlx::PgPool;
use crate::user::{NewUser, RegisterInput};
use crate::repositary::{create_user, find_user_by_email};
use crate::security::{hash_password, verify_password};
use crate::jwt::generate_jwt;
use dotenvy::dotenv;
use std::env;
use crate::errors::AppError;
use validator::Validate;

#[derive(serde::Serialize)]
struct AuthResponse {
    access_token: String,
}

/// Register user and return JWT
pub async fn register_user(
    pool: web::Data<PgPool>,
    new_user: web::Json<RegisterInput>,
) -> Result<HttpResponse, AppError> {
    let input = new_user.into_inner();

    input.validate().map_err(|e| {
        AppError::Hash(argon2::password_hash::Error::Password) // or create a custom error
    })?;

    let new_user_data = NewUser {
        username: input.username,
        email: input.email,
        password: input.password,
    };

    let user = create_user(pool.get_ref(), new_user_data).await?; // ? converts sqlx::Error or hash error to AppError
    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let token = crate::jwt::generate_jwt(user.id, &secret);

    Ok(HttpResponse::Ok().json(AuthResponse { access_token: token }))
}


/// Login user and return JWT
#[derive(serde::Deserialize)]
pub struct LoginInput {
    pub email: String,
    pub password: String,
}

pub async fn login_user(
    pool: web::Data<PgPool>,
    login: web::Json<LoginInput>,
) -> Result<HttpResponse, AppError> {
    dotenvy::dotenv().ok();
    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");

    let user = find_user_by_email(pool.get_ref(), &login.email).await?;

    if user.password_hash.is_empty() {
        return Ok(HttpResponse::InternalServerError().body("Password not set for user"));
    }

    let is_valid = verify_password(&user.password_hash, &login.password)?;
    if is_valid {
        let token = generate_jwt(user.id, &secret);
        Ok(HttpResponse::Ok().json(AuthResponse { access_token: token }))
    } else {
        Ok(HttpResponse::Unauthorized().body("Invalid credentials"))
    }
}

