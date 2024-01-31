use super::common_passwords::COMMON_PASSWORDS;

pub enum PasswordValidationError {
    TooShort,
    TooLong,
    InvalidCharacters,
    CommonPassword,
}

impl ToString for PasswordValidationError {
    fn to_string(&self) -> String {
        match self {
            Self::TooShort => "Password is too short. Must be 8 or more characters.",
            Self::TooLong => "Password is too long. Must be 64 or less characters",
            Self::InvalidCharacters => "Password contains invalid characters.",
            Self::CommonPassword => "Password is a commonly used and insecure.",
        }
        .to_string()
    }
}

pub enum PasswordStrength {
    VeryWeak,
    Weak,
    Medium,
    Strong,
    VeryStrong,
}

impl PasswordStrength {
    fn increment(self) -> Self {
        match self {
            Self::VeryWeak => Self::Weak,
            Self::Weak => Self::Medium,
            Self::Medium => Self::Strong,
            Self::Strong => Self::VeryStrong,
            Self::VeryStrong => Self::VeryStrong,
        }
    }
}

/// Returns the length of a password including strength based on length.
pub fn length_check(password: &str) -> Result<PasswordStrength, PasswordValidationError> {
    // Length Checks
    let len = password.len();

    match len {
        x if (0..8).contains(&x) => Err(PasswordValidationError::TooShort),
        8 => Ok(PasswordStrength::VeryWeak),
        x if (9..12).contains(&x) => Ok(PasswordStrength::Weak),
        x if (12..17).contains(&x) => Ok(PasswordStrength::Medium),
        x if (17..20).contains(&x) => Ok(PasswordStrength::Strong),
        x if (20..65).contains(&x) => Ok(PasswordStrength::VeryStrong),
        _ => Err(PasswordValidationError::TooLong),
    }
}

/// Returns true if the password exists in the common password database.
/// Any common password should be rejected for security reasons.
pub fn is_common_password(password: &str) -> bool {
    // TODO: Benchmark binary search, hashmap, or Vec lookup for common passwords
    COMMON_PASSWORDS.binary_search(&password).is_ok()
}

/// Check the strength of the password. Will return Ok with the strength,
/// otherwise returns an Err with the reason for rejecting.
pub fn get_password_strength(password: &str) -> Result<PasswordStrength, PasswordValidationError> {
    // First check length
    let mut strength = length_check(password)?;

    // Character Checks
    if !password.is_ascii() {
        return Err(PasswordValidationError::InvalidCharacters);
    }

    // Common Password Check
    if is_common_password(password) {
        return Err(PasswordValidationError::CommonPassword);
    }

    // Strength Increment
    let mut has_lower = false;
    let mut has_upper = false;
    let mut has_number = false;
    let mut has_special_character = false;

    for char in password.chars().into_iter() {
        if !has_lower && char.is_ascii_lowercase() {
            has_lower = true
        }

        if !has_upper && char.is_ascii_uppercase() {
            has_upper = true
        }

        if !has_number && char.is_ascii_digit() {
            has_number = true
        }

        if !has_special_character && char.is_ascii_punctuation() {
            has_special_character = true
        }

        if has_lower && has_upper && has_number && has_special_character {
            strength = strength.increment();
            break;
        }
    }

    Ok(strength)
}
