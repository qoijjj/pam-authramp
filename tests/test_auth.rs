extern crate pam_api;

use std::process::Command;
use pam_api::Client;


// Authentication Integration Test
// Intended to run in containerized dev environment.

static TEST_UID: u32 = 5555;

type TestResult = Result<(), Box<dyn std::error::Error>>;

// run before extern crate users;tests
fn init() {
  // create test_user
  let _ = Command::new("sudo")
                  .args(["useradd", "test_user", "-u", &TEST_UID.to_string(), "-M"])
                  .status();
  // install library
  let _ = Command::new("sudo")
                  .args(["cp", "target/release/libpam_rampdelay.so", "/lib64/security"])
                  .status();
  // install configuration
  let _ = Command::new("sudo")
                  .args(["cp", "tests/conf/test-auth", "/etc/pam.d"])
                  .status();
}

// run after tests
fn clean() {
  // delete test_user
  let _ = Command::new("sudo")
                  .args(["userdel", "test_user"])
                  .status();
  // remove library
  let _ = Command::new("sudo")
                  .args(["rm", "/lib64/security/libpam_rampdelay.so"])
                  .status();
  // remove configuration
  let _ = Command::new("sudo")
                  .args(["rm", "/etc/pam.d/test-auth"])
                  .status();
}

#[test]
fn valid_credentials() -> TestResult {
  init();
  clean();
  Ok(())
}