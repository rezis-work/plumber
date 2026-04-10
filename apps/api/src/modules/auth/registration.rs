//! Extra field validation for plumber registration (Step 3).

use super::register_error::RegisterError;

const FULL_NAME_MAX: usize = 200;
const PHONE_MIN: usize = 8;
const PHONE_MAX: usize = 32;
const YEARS_MAX: i32 = 80;

/// Trim, require non-empty, max length.
pub fn validate_full_name(raw: &str) -> Result<String, RegisterError> {
    let s = raw.trim();
    if s.is_empty() {
        return Err(RegisterError::Validation {
            message: "full name is required".to_string(),
        });
    }
    if s.len() > FULL_NAME_MAX {
        return Err(RegisterError::Validation {
            message: "full name is too long".to_string(),
        });
    }
    Ok(s.to_string())
}

/// Collapse whitespace, length bounds, require at least 8 digits for a minimal sanity check.
pub fn normalize_and_validate_phone(raw: &str) -> Result<String, RegisterError> {
    let collapsed: String = raw.chars().filter(|c| !c.is_whitespace()).collect();
    let len = collapsed.len();
    if len < PHONE_MIN || len > PHONE_MAX {
        return Err(RegisterError::Validation {
            message: "invalid phone number".to_string(),
        });
    }
    let digit_count = collapsed.chars().filter(|c| c.is_ascii_digit()).count();
    if digit_count < 8 {
        return Err(RegisterError::Validation {
            message: "invalid phone number".to_string(),
        });
    }
    Ok(collapsed)
}

pub fn validate_years_of_experience(years: i32) -> Result<(), RegisterError> {
    if years < 0 {
        return Err(RegisterError::Validation {
            message: "years of experience must be non-negative".to_string(),
        });
    }
    if years > YEARS_MAX {
        return Err(RegisterError::Validation {
            message: "years of experience is too large".to_string(),
        });
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn full_name_rejects_empty() {
        assert!(validate_full_name("   ").is_err());
    }

    #[test]
    fn phone_normalizes_spaces() {
        assert_eq!(
            normalize_and_validate_phone("  +1 234 567 8901  ").unwrap(),
            "+12345678901"
        );
    }

    #[test]
    fn years_bounds() {
        assert!(validate_years_of_experience(-1).is_err());
        assert!(validate_years_of_experience(81).is_err());
        assert!(validate_years_of_experience(0).is_ok());
    }
}
