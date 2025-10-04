use chrono::{Utc, Duration};
use serde::{Serialize, Deserialize};
use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey, TokenData};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: i32,   // user id
    pub exp: usize, // expiration timestamp
}

/// Generate JWT token
pub fn generate_jwt(user_id: i32, secret: &str) -> String {
    let expiration = Utc::now() + Duration::hours(24); // expires in 24h
    let claims = Claims {
        sub: user_id,
        exp: expiration.timestamp() as usize,
    };
    encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_ref()))
        .expect("Failed to generate JWT")
}

/// Decode JWT token
pub fn decode_jwt(token: &str, secret: &str) -> Result<TokenData<Claims>, jsonwebtoken::errors::Error> {
    decode::<Claims>(token, &DecodingKey::from_secret(secret.as_ref()), &Validation::default())
}
