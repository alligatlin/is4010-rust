#![allow(dead_code)]
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum PasswordStrength {
    Weak,
    Medium,
    Strong,
    VeryStrong,
}

impl fmt::Display for PasswordStrength {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let label = match self {
            PasswordStrength::Weak => "Weak",
            PasswordStrength::Medium => "Medium",
            PasswordStrength::Strong => "Strong",
            PasswordStrength::VeryStrong => "Very strong",
        };
        write!(f, "{}", label)
    }
}

pub fn validate_strength(password: &str) -> PasswordStrength {
    let mut score = 0;
    let len = password.len();

    if len >= 8 {
        score += 1;
    }
    if len >= 12 {
        score += 1;
    }
    if len >= 16 {
        score += 1;
    }
    if password.chars().any(|c| c.is_lowercase()) {
        score += 1;
    }
    if password.chars().any(|c| c.is_uppercase()) {
        score += 1;
    }
    if password.chars().any(|c| c.is_numeric()) {
        score += 1;
    }
    if password.chars().any(|c| !c.is_alphanumeric()) {
        score += 1;
    }

    match score {
        0..=2 => PasswordStrength::Weak,
        3..=4 => PasswordStrength::Medium,
        5..=6 => PasswordStrength::Strong,
        _ => PasswordStrength::VeryStrong,
    }
}

pub fn check_common_patterns(password: &str) -> bool {
    let lower = password.to_lowercase();
    // Pattern 1: All same chars
    let first = password.chars().next();
    let all_same = !password.is_empty() && password.chars().all(|c| Some(c) == first);

    // Pattern 2: In common list
    let is_common = COMMON_PASSWORDS.iter().any(|&p| p == lower);

    all_same || is_common
}

pub fn calculate_entropy(password: &str) -> f64 {
    if password.is_empty() {
        return 0.0;
    }
    let mut charset_size = 26;
    if password.chars().any(|c| c.is_uppercase()) {
        charset_size = 52;
    }
    if password.chars().any(|c| c.is_numeric()) {
        charset_size = 62;
    }
    if password.chars().any(|c| !c.is_alphanumeric()) {
        charset_size = 94;
    }

    (charset_size as f64).log2() * password.len() as f64
}

pub const COMMON_PASSWORDS: &[&str] = &[
    "password",
    "123456",
    "password123",
    "qwerty",
    "letmein",
    "iloveyou",
    "admin",
    "welcome",
    "monkey",
    "dragon",
];
// ============================================================================
// TESTS — DO NOT MODIFY
// ============================================================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strength_weak_short() {
        assert_eq!(validate_strength("hi"), PasswordStrength::Weak);
    }

    #[test]
    fn test_strength_very_strong() {
        assert_eq!(
            validate_strength("MyStr0ng!Pass2024"),
            PasswordStrength::VeryStrong
        );
    }

    #[test]
    fn test_common_password_detected() {
        assert!(check_common_patterns("password"));
        assert!(check_common_patterns("123456"));
    }

    #[test]
    fn test_all_same_char_detected() {
        assert!(check_common_patterns("aaaa"));
    }

    #[test]
    fn test_entropy_alphanumeric() {
        let e = calculate_entropy("aB3d");
        assert!((e - 4.0 * f64::log2(62.0)).abs() < 1e-9);
    }
}
