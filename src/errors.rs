use actix_web::{HttpResponse, ResponseError};
use std::fmt;

#[derive(Debug)]
pub enum AppError {
    Db(sqlx::Error),
    Hash(argon2::password_hash::Error),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::Db(e) => write!(f, "Database error: {}", e),
            AppError::Hash(e) => write!(f, "Hashing error: {}", e),
        }
    }
}

// This tells Actix how to convert AppError into HTTP response
impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        match self {
            AppError::Db(_) => HttpResponse::InternalServerError().body(self.to_string()),
            AppError::Hash(_) => HttpResponse::InternalServerError().body(self.to_string()),
        }
    }
}

impl From<sqlx::Error> for AppError {
    fn from(e: sqlx::Error) -> Self {
        AppError::Db(e)
    }
}

impl From<argon2::password_hash::Error> for AppError {
    fn from(e: argon2::password_hash::Error) -> Self {
        AppError::Hash(e)
    }
}
