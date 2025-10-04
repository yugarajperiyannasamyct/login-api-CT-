use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use validator::Validate;
use crate::validation::{validate_lowercase, validate_uppercase, validate_digit, validate_special};




#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub role: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewUser {
    pub username: String,
    pub email: String,
    pub password: String, // plain password (not stored in DB)
}



#[derive(Debug, Validate, Deserialize)] 
pub struct RegisterInput {
    #[validate(length(min = 8))]
    #[validate(custom = "validate_lowercase")]
    #[validate(custom = "validate_uppercase")]
    #[validate(custom = "validate_digit")]
    #[validate(custom = "validate_special")]
    pub password: String,
    pub email: String,
    pub username: String,
}
