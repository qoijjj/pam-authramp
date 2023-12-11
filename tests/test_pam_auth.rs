#[macro_use]
extern crate dotenv_codegen;
extern crate pam_client;

mod common;

// Authentication Integration Tests
// Intended to run in containerized dev environment.
#[cfg(test)]
mod tests {
    use super::common;
    use super::common::TestResult;

    const USER_PASSWD: &str = dotenv!("TEST_USER_PASSWD");
    const USER_NAME: &str = dotenv!("TEST_USER_NAME");

    #[test]
    fn test_system_auth_valid() -> TestResult {
        let _ = common::test_service("test-system-auth", USER_NAME, USER_PASSWD);
        Ok(())
    }

    #[test]
    fn test_system_auth_invalid() -> TestResult {
        let _ = common::test_service("test-system-auth", "INVALID", "CREDS").unwrap_err();
        Ok(())
    }

    #[test]
    fn test_rampdelay_preauth() -> TestResult {
        let _ = common::test_service("test-rampdelay-preauth", USER_NAME, USER_PASSWD);
        Ok(())
    }

    #[test]
    fn test_rampdelay_authfail() -> TestResult {
        let _ = common::test_service("test-rampdelay-authfail", USER_NAME, USER_PASSWD).unwrap_err();
        Ok(())
    }
}
