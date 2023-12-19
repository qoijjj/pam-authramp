#[macro_use]
extern crate dotenv_codegen;

mod common;

#[cfg(test)]
mod test_pam_auth {
    use super::common::utils;
    use pam_client::{conv_mock::Conversation, Context, Flag};

    const USER_NAME: &str = dotenv!("TEST_USER_NAME");
    const USER_PWD: &str = dotenv!("TEST_USER_PWD");

    #[test]
    fn test_pam_auth_valid_credentials() {
        utils::create_pam_service_file();
        let mut ctx = Context::new(
            utils::PAM_SRV, // Service name
            None,
            Conversation::with_credentials(USER_NAME, USER_PWD),
        )
        .expect("Failed to create PAM context");

        ctx.authenticate(Flag::NONE).expect("Authentication failed");
        ctx.acct_mgmt(Flag::NONE)
            .expect("Account management failed");
        utils::remove_pam_service_file();
    }

    #[test]
    fn test_pam_auth_invalid_credentials() {
        utils::create_pam_service_file();
        let mut ctx = Context::new(
            utils::PAM_SRV, // Service name
            None,
            Conversation::with_credentials("INVALID", "CREDENTIALS"),
        )
        .expect("Failed to create PAM context");

        // Expect an error during authentication (invalid credentials)
        let _ = ctx.authenticate(Flag::NONE).unwrap_err();

        // Additional assertions or test steps
        ctx.acct_mgmt(Flag::NONE)
            .expect("Account management failed");
        utils::remove_pam_service_file();
    }
}
