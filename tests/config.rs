use std::env;

/// Test configuration for the todoist-api library
pub struct TestConfig {
    /// Whether to run integration tests that require API access
    pub run_integration_tests: bool,
    /// API token for testing (if available)
    pub api_token: Option<String>,
    /// Base URL for testing (can be overridden for mock testing)
    pub base_url: String,
    /// Timeout for test requests
    pub timeout_seconds: u64,
}

impl Default for TestConfig {
    fn default() -> Self {
        Self {
            run_integration_tests: env::var("RUN_INTEGRATION_TESTS").is_ok(),
            api_token: env::var("TODOIST_API_TOKEN").ok(),
            base_url: env::var("TODOIST_TEST_BASE_URL")
                .unwrap_or_else(|_| "https://api.todoist.com/api/v1".to_string()),
            timeout_seconds: env::var("TODOIST_TEST_TIMEOUT")
                .unwrap_or_else(|_| "30".to_string())
                .parse()
                .unwrap_or(30),
        }
    }
}

impl TestConfig {
    /// Create a new test configuration
    pub fn new() -> Self {
        Self::default()
    }

    /// Check if integration tests should be run
    pub fn should_run_integration_tests(&self) -> bool {
        self.run_integration_tests && self.api_token.is_some()
    }

    /// Get the API token for testing
    pub fn get_api_token(&self) -> String {
        self.api_token.clone().unwrap_or_else(|| "test-token-12345".to_string())
    }

    /// Check if we have a real API token
    pub fn has_real_api_token(&self) -> bool {
        self.api_token.is_some() && !self.api_token.as_ref().unwrap().starts_with("test-token")
    }

    /// Get the base URL for testing
    pub fn get_base_url(&self) -> &str {
        &self.base_url
    }

    /// Get the timeout for test requests
    pub fn get_timeout_seconds(&self) -> u64 {
        self.timeout_seconds
    }
}

/// Test environment utilities
pub mod test_env {
    use std::env;

    /// Set up test environment variables
    pub fn setup_test_env() {
        // Set default test values if not already set
        if env::var("TODOIST_API_TOKEN").is_err() {
            env::set_var("TODOIST_API_TOKEN", "test-token-12345");
        }

        if env::var("RUN_INTEGRATION_TESTS").is_err() {
            env::set_var("RUN_INTEGRATION_TESTS", "false");
        }

        if env::var("TODOIST_TEST_TIMEOUT").is_err() {
            env::set_var("TODOIST_TEST_TIMEOUT", "30");
        }
    }

    /// Clean up test environment variables
    pub fn cleanup_test_env() {
        // Remove test-specific environment variables
        env::remove_var("TODOIST_API_TOKEN");
        env::remove_var("RUN_INTEGRATION_TESTS");
        env::remove_var("TODOIST_TEST_TIMEOUT");
        env::remove_var("TODOIST_TEST_BASE_URL");
    }

    /// Check if we're running in a CI environment
    pub fn is_ci() -> bool {
        env::var("CI").is_ok() || env::var("GITHUB_ACTIONS").is_ok()
    }

    /// Check if we're running in a test environment
    pub fn is_test() -> bool {
        env::var("CARGO_PKG_NAME").is_ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_test_config_default() {
        let config = TestConfig::new();
        assert!(!config.run_integration_tests);
        // API token might or might not be set in test environment
        // assert!(config.api_token.is_some());
        assert_eq!(config.base_url, "https://api.todoist.com/api/v1");
        assert_eq!(config.timeout_seconds, 30);
    }

    #[test]
    fn test_test_config_methods() {
        let config = TestConfig::new();

        assert!(!config.should_run_integration_tests());
        // Skip assertion on has_real_api_token() - it depends on environment
        // In development, a real token might be set; in CI, it won't be
        assert_eq!(config.get_base_url(), "https://api.todoist.com/api/v1");
        assert_eq!(config.get_timeout_seconds(), 30);
    }

    #[test]
    fn test_env_utilities() {
        assert!(test_env::is_test());
        // Note: is_ci() might be true or false depending on the environment
    }
}
