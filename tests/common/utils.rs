use std::io;
use std::path::PathBuf;

use std::{
    fs::{remove_file, File},
    io::Write,
};

pub const SRV_DIR: &str = "/etc/pam.d";
pub const PAM_SRV: &str = "test-rampdelay";

const USER_NAME: &str = dotenv!("TEST_USER_NAME");

fn create_pam_service_file() -> io::Result<()> {
    let mut file = File::create(PathBuf::from(SRV_DIR).join(PAM_SRV))?;

    let content = "auth        required                                     libpam_authramp.so preauth \n\
                  auth        sufficient                                   pam_unix.so nullok \n\
                  auth        [default=die]                                libpam_authramp.so authfail \n\
                  account     required                                     libpam_authramp.so";

    file.write_all(content.as_bytes())?;
    Ok(())
}

fn remove_pam_service_file() -> io::Result<()> {
    remove_file(PathBuf::from(SRV_DIR).join(PAM_SRV))?;
    Ok(())
}

pub fn create_and_remove_pam_service<F>(test: F)
where
    F: FnOnce(),
{
    create_pam_service_file().expect("Failed to create PAM service file");
    test();
    remove_pam_service_file().expect("Failed to remove PAM service file");
}

pub fn get_tally_file_path() -> PathBuf {
    PathBuf::from("/var/run/rampdelay").join(USER_NAME)
}
