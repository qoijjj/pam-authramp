#[macro_use]
extern crate dotenv_codegen;
mod common;

#[cfg(test)]
mod test_pam_auth {
    use pam_client::{conv_mock::Conversation, Context, Flag};

    use super::common::utils;

    const USER_NAME: &str = dotenv!("TEST_USER_NAME");
    const USER_PWD: &str = dotenv!("TEST_USER_PWD");

    #[test]
    fn test_pam_auth_valid_credentials() {
        utils::create_pam_service_file().unwrap();

        let mut ctx = Context::new(
            utils::PAM_SRV, // Service name
            None,
            Conversation::with_credentials(USER_NAME, USER_PWD),
        )
        .unwrap();

        ctx.authenticate(Flag::NONE).unwrap();
        ctx.acct_mgmt(Flag::NONE).unwrap();

        utils::remove_pam_service_file().unwrap();
    }
}
