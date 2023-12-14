extern crate ini;
extern crate lazy_static;
extern crate serde;
extern crate users;
extern crate once_cell;


use std::collections::HashMap;
use std::env;
use std::ffi::CStr;
use std::path::{Path, PathBuf};

use self::ini::Ini;
use self::lazy_static::lazy_static;
use self::serde::Deserialize;
use self::users::{get_user_by_name, User};
use crate::Actions;
use self::once_cell::sync::Lazy;
use pam::constants::PamResultCode;
use pam::{constants::PamFlag, module::PamResult};

const DEFAULT_TALLY_DIR: &str = "/var/run/rampdelay";
const DEFAULT_CONFIG_PATH: &str = "/etc/security/rampdelay.conf";
static CONFIG_PATH: Lazy<String> = Lazy::new(|| {
    env::var("TEST_CONFIG_PATH").unwrap_or_else(|_| DEFAULT_CONFIG_PATH.to_string())
});

#[derive(Debug)]
pub struct Options {
     pub action: Option<Actions>,
    // pub user: User,
    pub tally_dir: String,
}

impl Default for Options {
    fn default() -> Self {
        Options {
            tally_dir: String::from(DEFAULT_TALLY_DIR),
            action: None,
        }
    }
}

impl Options {
    pub fn build(_user: String, args: Vec<&CStr>, _flags: PamFlag) -> Options {
        // Load INI file.
        let mut opts = Ini::load_from_file(PathBuf::from(CONFIG_PATH.to_string()).as_path())
            .ok()
            // Clone "Settings" section if it exists.
            .and_then(|ini| {
                ini.section(Some("Settings"))
                    .map(|settings| settings.clone())
            })
            // Map section to Options struct, defaulting "tally_dir" if absent.
            .map(|settings| Options {
                tally_dir: settings
                    .get("tally_dir")
                    .map(|s| s.to_string())
                    .unwrap_or_default(),
                ..Options::default()
            })
            // Fallback to default Options if any failures.
            .unwrap_or_else(|| Options::default());

            let action_map: HashMap<&str, Actions> = [
                ("preauth", Actions::PREAUTH),
                ("authsucc", Actions::AUTHSUCC),
                ("authfail", Actions::AUTHFAIL),
            ].iter().copied().collect();
            
            opts.action = args
                .iter()
                .find_map(|&carg| carg.to_str().ok().and_then(|arg| action_map.get(arg).cloned()));

        opts
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dotenv_codegen::dotenv;

    pub type TestResult = Result<(), Box<dyn std::error::Error>>;

    const USER_NAME: &str = dotenv!("TEST_USER_NAME");

    fn set_test_conf_path() {
        // Reset the CONFIG_PATH before each test
        env::set_var("TEST_CONFIG_PATH", "tests/conf/rampdelay.conf");
    }

    #[test]
    fn test_conf_path() -> TestResult {
        set_test_conf_path();
        assert_eq!(
            CONFIG_PATH.as_str(),
            "tests/conf/rampdelay.conf",
            "Expected testing config path"
        );
        Ok(())
    }

    #[test]
    fn test_conf_tally_dir() -> TestResult {
        set_test_conf_path();
        let args = [].to_vec();
        let flags: PamFlag = 0;
        let opts = Options::build(USER_NAME.to_string(), args, flags);
        assert_eq!(
            opts.tally_dir, "./tests/tally",
            "Expected ./tests/tally tall_dir value"
        );
        Ok(())
    }

    #[test]
    fn test_action_default() -> TestResult {
        set_test_conf_path();
        let args = [].to_vec();
        let flags: PamFlag = 0;
        let opts = Options::build(USER_NAME.to_string(), args, flags);
        assert!(opts.action.is_none(), "Expected action to be None");
        Ok(())
    }

    #[test]
    fn test_action_preauth() -> TestResult {
        set_test_conf_path();
        let args = [CStr::from_bytes_with_nul("preauth\0".as_bytes())?].to_vec();
        let flags: PamFlag = 0;
        let opts = Options::build(USER_NAME.to_string(), args, flags);
        assert_eq!(opts.action, Some(Actions::PREAUTH), "Expected action to be Preauth");
        Ok(())
    }  
    
    #[test]
    fn test_action_authfail() -> TestResult {
        set_test_conf_path();
        let args = [CStr::from_bytes_with_nul("authfail\0".as_bytes())?].to_vec();
        let flags: PamFlag = 0;
        let opts = Options::build(USER_NAME.to_string(), args, flags);
        assert_eq!(opts.action, Some(Actions::AUTHFAIL), "Expected action to be Authfail");
        Ok(())
    }
    /*
    #[test]
    fn test_conf_load_uname_valid() -> TestResult {
      format  set_test_env();

        let args: Vec<&CStr> = [].to_vec();
        let flags: PamFlag = 0;

        let opts = Options::build(USER_NAME.to_string(), args, flags)
            .expect("failed loading config with valid user");
        assert_eq!(opts.user.name(), USER_NAME);

        Ok(())
    }

    #[test]
    fn test_conf_load_uname_invalid() -> TestResult {
        set_test_env();

        let args: Vec<&CStr> = [].to_vec();
        let flags: PamFlag = 0;


        let res = Options::build("invalid".to_string(), args, flags)
            .expect_err("success loading config with invalid username!");
        assert_eq!(
            res,
            PamResultCode::PAM_SYSTEM_ERR,
            "Expected PAM_SYSTEM_ERR"
        );

        Ok(())
    }
    */
}
