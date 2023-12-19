use std::path::PathBuf;

use std::{
    fs::{remove_file, File},
    io::Write,
};

pub const SRV_DIR: &str = "/etc/pam.d";
pub const PAM_SRV: &str = "test-rampdelay";

const USER_NAME: &str = dotenv!("TEST_USER_NAME");

pub fn create_pam_service_file() {
    let mut file =
        File::create(PathBuf::from(SRV_DIR).join(PAM_SRV)).expect("failed to create service file");

    let content = "auth        required                                     libpam_authramp.so preauth \n\
                  auth        sufficient                                   pam_unix.so nullok \n\
                  auth        [default=die]                                libpam_authramp.so authfail \n\
                  account     required                                     libpam_authramp.so";

    file.write_all(content.as_bytes())
        .expect("failed to create service file");
}

pub fn remove_pam_service_file() {
    remove_file(PathBuf::from(SRV_DIR).join(PAM_SRV)).expect("failed to remove service file");
}

pub fn get_tally_file_path() -> PathBuf {
    return PathBuf::from("/var/run/rampdelay").join(USER_NAME)
}
