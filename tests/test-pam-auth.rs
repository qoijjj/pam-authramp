#[macro_use]
extern crate dotenv_codegen;

mod common;

#[cfg(test)]
mod test_pam_auth {

    use std::fs;

    use crate::common::utils::get_pam_context;

    use super::common::utils;
    use pam_client::Flag;

    const USER_NAME: &str = dotenv!("TEST_USER_NAME");
    const USER_PWD: &str = dotenv!("TEST_USER_PWD");

    #[test]
    fn test_valid_auth_success() {
        utils::init_and_clear_test(|| {
            let mut ctx = get_pam_context(USER_NAME, USER_PWD);

            // Expect the authentication to succeed
            ctx.authenticate(Flag::NONE).expect("Authentication failed");
            ctx.acct_mgmt(Flag::NONE)
                .expect("Account management failed")
        });
    }

    #[test]
    fn test_invalid_auth_creates_tally() {
        utils::init_and_clear_test(|| {
            let mut ctx = get_pam_context(USER_NAME, "INVALID");

            // Expect an error during authentication (invalid credentials)
            let auth_result = ctx.authenticate(Flag::NONE);
            assert!(auth_result.is_err(), "Authentication succeeded!");

            // Expect tally file gets created
            let tally_file_path = utils::get_tally_file_path(USER_NAME);
            assert!(tally_file_path.exists(), "Tally file not created")
        });
    }

    #[test]
    fn test_consecutive_invalid_adds_tally() {
        utils::init_and_clear_test(|| {
            let mut ctx = get_pam_context(USER_NAME, "INVALID");

            let mut count = 0;
            let total_tries = 2;

            while count < total_tries {
                // Expect an error during authentication (invalid credentials)
                let auth_result = ctx.authenticate(Flag::NONE);
                assert!(auth_result.is_err(), "Authentication succeeded!");

                count += 1;
            }

            // Expect tally file gets created
            let tally_file_path = utils::get_tally_file_path(USER_NAME);
            assert!(tally_file_path.exists(), "Tally file not created");

            // Expect tally count
            let ini_content = fs::read_to_string(tally_file_path).unwrap();
            assert!(ini_content.contains(&format!("count={}", total_tries)));
        })
    }

    #[test]
    fn test_valid_auth_clears_tally() {
        utils::init_and_clear_test(|| {
            let mut ctx = get_pam_context(USER_NAME, "INVALID");

            // Expect an error during authentication (invalid credentials)
            let auth_result = ctx.authenticate(Flag::NONE);
            assert!(auth_result.is_err(), "Authentication succeeded!");

            // Expect tally file gets created
            let tally_file_path = utils::get_tally_file_path(USER_NAME);
            assert!(tally_file_path.exists(), "Tally file not created");

            // Expect tally count to increase
            let ini_content = fs::read_to_string(&tally_file_path).unwrap();
            assert!(
                ini_content.contains("count=1"),
                "Expected tally count to increase"
            );

            let mut ctx = get_pam_context(USER_NAME, USER_PWD);

            // Expect an error during authentication (invalid credentials)
            let auth_result = ctx.authenticate(Flag::NONE);
            assert!(auth_result.is_ok(), "Authentication failed!");

            ctx.acct_mgmt(Flag::NONE)
                .expect("Account management failed");

            // Expect tally count to decrease
            let ini_content = fs::read_to_string(&tally_file_path).unwrap();
            assert!(ini_content.contains("count=0"), "Expected tally count = 0");
        })
    }

    /*#[test]
    fn test_failures_exceed_free_tries_cause_bounce() {
        utils::init_and_clear_test(|| {
            let mut ctx = get_pam_context(USER_NAME, "INVALID");

            let mut count = 0;
            // free tries default is 6
            let total_tries = 7;

            while count < total_tries {
                // Expect an error during authentication (invalid credentials)
                let auth_result = ctx.authenticate(Flag::NONE);
                assert!(auth_result.is_err(), "Authentication succeeded!");

                count += 1;
            }

            // Expect tally file gets created
            let tally_file_path = utils::get_tally_file_path(USER_NAME);
            assert!(tally_file_path.exists(), "Tally file not created");

            // Expect tally count
            let ini_content = fs::read_to_string(tally_file_path).unwrap();
            assert!(ini_content.contains(&format!("count={}", total_tries)));

            let log = ctx.conversation().log.clone();

            // expect pam log message
            let log_str = format!("{:?}", &log);
            assert_eq!(&log_str, "[Error(\"Account locked!\")]");
        });
    }*/
}
