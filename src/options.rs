extern crate ini;
extern crate lazy_static;
extern crate serde;
extern crate users;

use std::collections::HashMap;
use std::env;
use std::ffi::CStr;
use std::path::{Path, PathBuf};

use self::ini::Ini;
use self::lazy_static::lazy_static;
use self::serde::Deserialize;
use self::users::{get_user_by_name, User};
use crate::Actions;
use pam::constants::PamResultCode;
use pam::{constants::PamFlag, module::PamResult};

const DEFAULT_TALLY_DIR: &str = "/var/run/rampdelay";
const DEFAULT_CONFIG_PATH: &str = "/etc/security/rampdelay.conf";
lazy_static! {
    static ref CONFIG_PATH: String = {
        if let Ok(val) = env::var("TEST_CONFIG_PATH") {
            String::from(val)
        } else {
            String::from(DEFAULT_CONFIG_PATH)
        }
    };
}

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
/*
            let action_map: HashMap<&str, Actions> = [
                ("preauth", Actions::PREAUTH),
                ("authsucc", Actions::AUTHSUCC),
                ("authfail", Actions::AUTHFAIL),
            ].iter().copied().collect();
            
            opts.action = args
                .iter()
                .find_map(|&carg| carg.to_str().ok().and_then(|arg| action_map.get(arg).cloned()));
*/
        opts
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dotenv_codegen::dotenv;

    pub type TestResult = Result<(), Box<dyn std::error::Error>>;

    const USER_NAME: &str = dotenv!("TEST_USER_NAME");

    #[test]
    fn test_conf_path() -> TestResult {
        env::set_var("TEST_CONFIG_PATH", "tests/conf/rampdelay.conf");
        assert_eq!(
            CONFIG_PATH.as_str(),
            "tests/conf/rampdelay.conf",
            "Expected testing config path"
        );
        Ok(())
    }

    #[test]
    fn test_conf_tally_dir_default() -> TestResult {
        env::set_var("TEST_CONFIG_PATH", "/bad/path");
        let args = [].to_vec();
        let flags: PamFlag = 0;
        let opts = Options::build(USER_NAME.to_string(), args, flags);
        assert_eq!(
            opts.tally_dir, DEFAULT_TALLY_DIR,
            "Expected default tally_dir value"
        );
        Ok(())
    }

    #[test]
    fn test_conf_tally_dir() -> TestResult {
        env::set_var("TEST_CONFIG_PATH", "tests/conf/rampdelay.conf");
        let args = [].to_vec();
        let flags: PamFlag = 0;
        let opts = Options::build(USER_NAME.to_string(), args, flags);
        assert_eq!(
            opts.tally_dir, "./tests/tally",
            "Expected ./tests/tally tall_dir value"
        );
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
