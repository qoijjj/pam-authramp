#[macro_use]
extern crate dotenv_codegen;
extern crate pam_client;

mod common;

// Authentication Integration Tests
// Intended to run in containerized dev environment.
#[cfg(test)]
mod tests {

    use pam_client::conv_mock::Conversation;
    use pam_client::{Context, Flag};

    use super::common;

    type TestResult = Result<(), Box<dyn std::error::Error>>;

    const USER_PASSWD: &str = dotenv!("TEST_USER_PASSWD");
    const USER_NAME: &str = dotenv!("TEST_USER_NAME");

    #[test]
    fn valid_credentials() -> TestResult {
        common::setup();
        let mut context = Context::new(
            "rampdelay-auth", // Service name
            None,
            Conversation::with_credentials(USER_NAME, USER_PASSWD),
        )
        .expect("Failed to initialize PAM context");

        // Authenticate the user
        context
            .authenticate(Flag::NONE)
            .expect("Authentication failed");

        // Validate the account
        context
            .acct_mgmt(Flag::NONE)
            .expect("Account validation failed");
        common::clean();
        Ok(())
    }

    #[test]
    fn invalid_credentials() -> TestResult {
        common::setup();
        let mut context = Context::new(
            "rampdelay-auth", // Service name
            None,
            Conversation::with_credentials("invalid", "creds"),
        )
        .expect("Failed to initialize PAM context");

        // Authenticate the user
        let res = context.authenticate(Flag::NONE);
        common::clean();
        match res {
            Ok(_) => panic!("Authenticated with invalid credentials!"),
            Err(_) => Ok(()),
        }
    }
}
