use std::fs::{remove_dir, remove_dir_all};
use std::io;
use std::path::PathBuf;

use std::{
    fs::{remove_file, File},
    io::Write,
};

use pam_client::conv_mock::Conversation;
use pam_client::Context;

pub const SRV_DIR: &str = "/etc/pam.d";
pub const PAM_SRV: &str = "test-rampdelay";

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

fn clear_tally_dir() -> Result<(), io::Error> {
    remove_dir_all("/var/run/rampdelay")?;
    Ok(())
}

pub fn init_and_clear_test<F>(test: F)
where
    F: FnOnce(),
{
    create_pam_service_file().expect("Failed to create PAM service file");
    test();
    remove_pam_service_file().expect("Failed to remove PAM service file");
    clear_tally_dir().expect("Failes clearing tally dir");
}

pub fn get_pam_context(u_name: &str, u_pwd: &str) -> Context<Conversation> {
    Context::new(PAM_SRV, None, Conversation::with_credentials(u_name, u_pwd))
        .expect("Failed creating PAM context!")
}

pub fn get_tally_file_path(u_name: &str) -> PathBuf {
    PathBuf::from("/var/run/rampdelay").join(u_name)
}
