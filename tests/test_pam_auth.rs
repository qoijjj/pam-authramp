#[macro_use]
extern crate dotenv_codegen;
extern crate pam_client;

mod common;

// Authentication Integration Tests
#[cfg(test)]
mod tests {
    use super::common::utilities;
    use super::common::utilities::TestResult;

    const USER_PASSWD: &str = dotenv!("TEST_USER_PASSWD");
    const USER_NAME: &str = dotenv!("TEST_USER_NAME");

    #[test]
    fn test_system_auth_valid() -> TestResult {
        // Test that the system authentication is successful with valid credentials
        utilities::test_service("test-system-auth", USER_NAME, USER_PASSWD)?;
        Ok(())
    }

    #[test]
    fn test_system_auth_invalid() -> TestResult {
        // Test that the system authentication is unsuccessful with invalid credentials
        assert!(utilities::test_service("test-system-auth", "INVALID", "CREDS").is_err());
        Ok(())
    }

    #[test]
    fn test_rampdelay_preauth() -> TestResult {
        // Test that the rampdelay auth initializes properly and returns successful
        utilities::test_service("test-rampdelay-preauth", USER_NAME, USER_PASSWD)?;
        Ok(())
    }

    #[test]
    fn test_rampdelay_authfail() -> TestResult {
        // Test that the rampdelay authfail parameter unsuccessful even with valid credentials
        assert!(
            utilities::test_service("test-rampdelay-authfail", USER_NAME, USER_PASSWD).is_err()
        );
        Ok(())
    }
}
