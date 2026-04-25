//! Secret masking for terminal output.
//!
//! Values whose keys contain security-related keywords are replaced with
//! `***MASKED***` to prevent accidental exposure during screen sharing or
//! terminal recordings. Masking is a **presentation-layer** concern — patch
//! files always contain the actual values.

/// Keywords that trigger secret masking (checked case-insensitively).
///
/// Includes the core PRD keywords plus defense-in-depth additions
/// from the chaos audit (Finding 4).
const SECRET_KEYWORDS: &[&str] = &[
    // PRD-mandated keywords
    "key", "password", "secret", "token", "auth",
    // Defense-in-depth additions (chaos audit Finding 4)
    "credential", "cert", "private", "signing",
    // Infrastructure and DB connection strings (v0.1 feedback)
    "connection_string", "conn_str", "url", "dsn", "redis", "rabbit", "amqp", "postgres", "mongo", "db"
];

/// Check if a key should have its value masked.
///
/// Returns `true` if the key contains any of the secret keywords
/// (case-insensitive substring match).
pub fn should_mask(key: &str) -> bool {
    let key_lower = key.to_lowercase();
    SECRET_KEYWORDS
        .iter()
        .any(|keyword| key_lower.contains(keyword))
}

/// Return the display value for a key — either the actual value or `***MASKED***`.
///
/// - If `show_secrets` is `true`, the actual value is always returned.
/// - Otherwise, masking is applied based on keyword detection via [`should_mask`].
pub fn mask_value(key: &str, value: &str, show_secrets: bool) -> String {
    if show_secrets || !should_mask(key) {
        value.to_string()
    } else {
        "***MASKED***".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ── should_mask tests ───────────────────────────────────────────

    #[test]
    fn test_mask_key_with_key() {
        assert!(should_mask("API_KEY"));
        assert!(should_mask("api_key"));
        assert!(should_mask("SECRET_KEY"));
    }

    #[test]
    fn test_mask_key_with_password() {
        assert!(should_mask("PASSWORD"));
        assert!(should_mask("DB_PASSWORD"));
        assert!(should_mask("password"));
    }

    #[test]
    fn test_mask_key_with_secret() {
        assert!(should_mask("CLIENT_SECRET"));
        assert!(should_mask("secret"));
        assert!(should_mask("APP_SECRET_VALUE"));
    }

    #[test]
    fn test_mask_key_with_token() {
        assert!(should_mask("AUTH_TOKEN"));
        assert!(should_mask("ACCESS_TOKEN"));
        assert!(should_mask("token"));
    }

    #[test]
    fn test_mask_key_with_auth() {
        assert!(should_mask("AUTH_HEADER"));
        assert!(should_mask("BASIC_AUTH"));
        assert!(should_mask("auth"));
    }

    #[test]
    fn test_case_insensitive_matching() {
        assert!(should_mask("Api_Key"));
        assert!(should_mask("API_KEY"));
        assert!(should_mask("api_key"));
        assert!(should_mask("ApI_kEy"));
    }

    #[test]
    fn test_no_mask_for_safe_keys() {
        assert!(!should_mask("PORT"));
        assert!(!should_mask("APP_NAME"));
        assert!(!should_mask("LOG_LEVEL"));
        assert!(!should_mask("DEBUG"));
        assert!(!should_mask("TIMEOUT"));
    }

    // ── Finding 4: expanded keyword coverage ────────────────────────

    #[test]
    fn test_mask_key_with_credential() {
        assert!(should_mask("AWS_CREDENTIAL"));
        assert!(should_mask("CREDENTIAL_FILE"));
    }

    #[test]
    fn test_mask_key_with_cert() {
        assert!(should_mask("SSL_CERT"));
        assert!(should_mask("TLS_CERT_PATH"));
    }

    #[test]
    fn test_mask_key_with_private() {
        assert!(should_mask("PRIVATE_KEY_PATH"));
        assert!(should_mask("SSH_PRIVATE"));
    }

    #[test]
    fn test_mask_key_with_signing() {
        assert!(should_mask("SIGNING_KEY_ID"));
        assert!(should_mask("JWT_SIGNING"));
    }

    #[test]
    fn test_mask_key_with_connection_string() {
        assert!(should_mask("CONNECTION_STRING"));
        assert!(should_mask("DB_CONN_STR"));
    }

    #[test]
    fn test_mask_infrastructure_keywords() {
        assert!(should_mask("DATABASE_URL"));
        assert!(should_mask("REDIS_HOST"));
        assert!(should_mask("MONGODB_URI"));
        assert!(should_mask("RABBITMQ_URL"));
        assert!(should_mask("POSTGRES_DB"));
        assert!(should_mask("AMQP_DSN"));
    }

    // ── mask_value tests ────────────────────────────────────────────

    #[test]
    fn test_mask_value_masked() {
        assert_eq!(
            mask_value("API_KEY", "super-secret-123", false),
            "***MASKED***"
        );
    }

    #[test]
    fn test_mask_value_shown_with_flag() {
        assert_eq!(
            mask_value("API_KEY", "super-secret-123", true),
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
}
