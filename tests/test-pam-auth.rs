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
    fn test_valid_creds_success() {
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
    fn test_invalid_creds_creates_tally() {
        utils::create_pam_service_file();
        let mut ctx = Context::new(
            utils::PAM_SRV, // Service name
            None,
            Conversation::with_credentials("INVALID", "CREDENTIALS"),
        )
        .expect("Failed to create PAM context");

        // Expect an error during authentication (invalid credentials)
        let auth_result = ctx.authenticate(Flag::NONE);
        assert!(auth_result.is_err(), "Authentication succeeded!");

        // Expect tally file gets created
        let tally_file_path = utils::get_tally_file_path();
        assert!(tally_file_path.exists(), "Tally file not created");
        
        utils::remove_pam_service_file();
    }
}
