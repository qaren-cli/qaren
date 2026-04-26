//! Secret masking for terminal output.
//!
//! Values whose keys contain security-related keywords are replaced with
//! `***MASKED***` to prevent accidental exposure during screen sharing or
//! terminal recordings. Masking is applied only to terminal output and
//! can be disabled via the `--show-secrets` flag.

/// Keywords that trigger automatic value masking.
///
/// Matches are case-insensitive.
const MASK_KEYWORDS: &[&str] = &[
    "secret",
    "password",
    "token",
    "key",
    "auth",
    "credential",
    "private",
    "cert",
    "signing",
    "connection_string",
    // Infrastructure credentials (from feedback)
    "db",
    "database",
    "redis",
    "rabbit",
    "mq",
    "dsn",
];

/// Check if a key should have its value masked.
pub fn should_mask(key: &str) -> bool {
    let key_lower = key.to_lowercase();
    MASK_KEYWORDS.iter().any(|&k| key_lower.contains(k))
}

/// Mask a value if the key is sensitive and masking is enabled.
pub fn mask_value(key: &str, value: &str, show_secrets: bool) -> String {
    if !show_secrets && should_mask(key) {
        "***MASKED***".to_string()
    } else {
        value.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_should_mask_exact() {
        assert!(should_mask("PASSWORD"));
        assert!(should_mask("db_password"));
        assert!(should_mask("API_KEY"));
    }

    #[test]
    fn test_should_mask_substring() {
        assert!(should_mask("super_secret_value"));
        assert!(should_mask("github_token"));
    }

    #[test]
    fn test_should_not_mask_safe() {
        assert!(!should_mask("LOG_LEVEL"));
        assert!(!should_mask("USER_NAME"));
    }

    #[test]
    fn test_case_insensitive_matching() {
        assert!(should_mask("password"));
        assert!(should_mask("Password"));
        assert!(should_mask("pAsSwOrD"));
    }

    #[test]
    fn test_mask_value_masked() {
        assert_eq!(
            mask_value("DB_PASSWORD", "super-secret-123", false),
            "***MASKED***"
        );
    }

    #[test]
    fn test_mask_value_shown_with_flag() {
        assert_eq!(
            mask_value("DB_PASSWORD", "super-secret-123", true),
            "super-secret-123"
        );
    }

    #[test]
    fn test_mask_value_safe_key_not_masked() {
        assert_eq!(
            mask_value("LOG_LEVEL", "info", false),
            "info"
        );
    }

    #[test]
    fn test_mask_infrastructure_keywords() {
        assert!(should_mask("DATABASE_URL"));
        assert!(should_mask("REDIS_HOST"));
        assert!(should_mask("RABBITMQ_PASS"));
    }
}
