#[macro_use]
extern crate dotenv_codegen;
extern crate pam_client;

use std::process::Command;

use pam_client::conv_mock::Conversation;
use pam_client::{Context, Flag};

// Authentication Integration Test
// Intended to run in containerized dev environment.

type TestResult = Result<(), Box<dyn std::error::Error>>;

const USER_NAME: &str = dotenv!("TEST_USER_NAME");
const USER_PASSWD: &str = dotenv!("TEST_USER_PASSWD");

// run before tests
fn setup() {
    // install library
    let _ = Command::new("sudo")
        .args([
            "cp",
            "target/release/libpam_rampdelay.so",
            "/lib64/security",
        ])
        .status();
    // install configuration
    let _ = Command::new("sudo")
        .args(["cp", "tests/conf/rampdelay-auth", "/etc/pam.d"])
        .status();
}

// run after tests
fn clean() {
    // remove library
    let _ = Command::new("sudo")
        .args(["rm", "/lib64/security/libpam_rampdelay.so"])
        .status();
    // remove configuration
    let _ = Command::new("sudo")
        .args(["rm", "/etc/pam.d/rampdelay-auth"])
        .status();
}

#[test]
fn valid_credentials() -> TestResult {
    setup();

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

    clean();
    Ok(())
}
