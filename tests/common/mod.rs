use std::{
    fs::{copy, remove_file},
    path::Path,
};

use pam_client::{conv_mock::Conversation, Context, Flag};

const LIBRARY_PATH: &str = dotenv!("TEST_LIBRARY_PATH");
const SERVICE_DIR: &str = dotenv!("TEST_SERVICE_DIR");

pub type TestResult = Result<(), Box<dyn std::error::Error>>;

fn copy_library() {
    copy("target/release/libpam_rampdelay.so", LIBRARY_PATH).expect("Failed to copy library");
}

fn delete_library() {
    if Path::new(LIBRARY_PATH).exists() {
        remove_file(LIBRARY_PATH).expect("Failed to remove library");
    }
}

fn copy_service(srv: &str) {
    copy("tests/conf/".to_owned() + srv, SERVICE_DIR.to_owned() + srv)
        .expect("Failed to copy service");
}

fn delete_service(srv: &str) {
    let path = &(SERVICE_DIR.to_owned() + srv);
    if Path::new(path).exists() {
        remove_file(path).expect("Failed to remove service");
    }
}

pub fn test_service(srv: &str, u_name: &str, u_pwd: &str) -> TestResult {
    copy_library();
    copy_service(srv);

    let mut ctx = Context::new(
        srv, // Service name
        None,
        Conversation::with_credentials(u_name, u_pwd),
    )?;

    ctx.authenticate(Flag::NONE)?;
    ctx.acct_mgmt(Flag::NONE)?;

    delete_library();
    delete_service(srv);
    Ok(())
}
