use validator::{Validate, ValidationError};

// Custom validators
pub fn validate_lowercase(password: &str) -> Result<(), ValidationError> {
    if password.chars().any(|c| c.is_lowercase()) {
        Ok(())
    } else {
        Err(ValidationError::new("missing_lowercase"))
    }
}

pub fn validate_uppercase(password: &str) -> Result<(), ValidationError> {
    if password.chars().any(|c| c.is_uppercase()) {
        Ok(())
    } else {
        Err(ValidationError::new("missing_uppercase"))
    }
}

pub fn validate_digit(password: &str) -> Result<(), ValidationError> {
    if password.chars().any(|c| c.is_ascii_digit()) {
        Ok(())
    } else {
        Err(ValidationError::new("missing_digit"))
    }
}

pub fn validate_special(password: &str) -> Result<(), ValidationError> {
    let special_chars = "!@#$%^&*()_+-=[]{}|;:',.<>/?";
    if password.chars().any(|c| special_chars.contains(c)) {
        Ok(())
    } else {
        Err(ValidationError::new("missing_special"))
    }
}
