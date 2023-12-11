use std::{
    fs::{copy, remove_file},
    path::Path,
};

const LIBRARY_PATH: &str = dotenv!("TEST_LIBRARY_PATH");
const SERVICE_PATH: &str = dotenv!("TEST_SERVICE_PATH");

pub fn setup() {
    // copy library
    copy("target/release/libpam_rampdelay.so", LIBRARY_PATH)
        .expect("Failed to copy libpam_rampdelay.so");

    // copy configuration
    copy("tests/conf/test-system-auth", "/etc/pam.d/test-system-auth")
        .expect("Failed to copy test-system-auth");
}

pub fn clean() {
    if Path::new(LIBRARY_PATH).exists() {
        remove_file(LIBRARY_PATH).expect("Failed to remove libpam_rampdelay.so");
    }
    if Path::new(SERVICE_PATH).exists() {
        remove_file(SERVICE_PATH).expect("Failed to remove rampdelay-auth");
    }
}
