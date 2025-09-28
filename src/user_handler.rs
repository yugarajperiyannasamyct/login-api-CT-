use actix_web::{web, HttpResponse, Responder};
use sqlx::PgPool;
use crate::user::NewUser;
use crate::repositary::{create_user, find_user_by_email};
use crate::security::verify_password;
use serde::Deserialize;



fn validate_password(password: &str) -> bool {
    let len_ok = password.len() >= 8;
    let has_lower = password.chars().any(|c| c.is_lowercase());
    let has_upper = password.chars().any(|c| c.is_uppercase());
    let has_digit = password.chars().any(|c| c.is_ascii_digit());
    let has_special = password.chars().any(|c| "!@#$%^&*()_+-=[]{}|;:',.<>/?".contains(c));

    len_ok && has_lower && has_upper && has_digit && has_special
}

#[derive(Deserialize)]
pub struct LoginInput {
    pub email: String,
    pub password: String,
}

pub async fn register_user(
    pool: web::Data<PgPool>,
    new_user: web::Json<NewUser>,
) -> impl Responder {

    let user = new_user.into_inner();

        if !validate_password(&user.password) {
        return HttpResponse::BadRequest().body(
            "Password must be at least 8 characters, include uppercase, lowercase, digit, and special character"
        );
    }

    // Password hashing is already done inside create_user
    match create_user(pool.get_ref(), user).await {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e)),
    }
}

pub async fn login_user(
    pool: web::Data<PgPool>,
    login: web::Json<LoginInput>,
) -> impl Responder {
    match find_user_by_email(pool.get_ref(), &login.email).await {
        Ok(user) => {
            if verify_password(&user.password_hash, &login.password) {
                HttpResponse::Ok().json(user) // in real apps, return JWT instead
            } else {
                HttpResponse::Unauthorized().body("Invalid credentials")
            }
        }
        Err(_) => HttpResponse::Unauthorized().body("Invalid credentials"),
    }
}
