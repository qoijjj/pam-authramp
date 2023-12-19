use std::path::PathBuf;
#[cfg(test)]
use std::{
    fs::{remove_file, File},
    io::Write,
};

pub const SRV_DIR: &str = "/etc/pam.d";
pub const PAM_SRV: &str = "test-rampdelay";

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
