
use argon2::{Argon2, PasswordHasher, PasswordVerifier};
use argon2::password_hash::{SaltString, PasswordHash, rand_core::OsRng};

/// Hash a plain text password
pub fn hash_password(password: &str) -> String {
    // Generate a random salt
    let salt = SaltString::generate(&mut OsRng);

    // Create Argon2 instance with default secure configuration
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .expect("Failed to hash password")
        .to_string();

    password_hash
}

/// Verify a plain password against a stored hash
pub fn verify_password(hash: &str, password: &str) -> bool {
    // Parse the stored password hash
    let parsed_hash = match PasswordHash::new(hash) {
        Ok(h) => h,
        Err(_) => return false, // invalid hash format
    };

    // Verify the password
    Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok()
}
