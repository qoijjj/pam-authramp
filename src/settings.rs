use crate::Actions;
use ini::Ini;
use once_cell::sync::Lazy;
use pam::constants::PamFlag;
use std::collections::HashMap;
use std::env;
use std::ffi::CStr;
use std::path::PathBuf;
use users::{get_user_by_name, User};

const DEFAULT_TALLY_DIR: &str = "/var/run/rampdelay";
const DEFAULT_CONFIG_PATH: &str = "/etc/security/rampdelay.conf";
static CONFIG_PATH: Lazy<String> = Lazy::new(|| {
    env::var("TEST_CONFIG_PATH").unwrap_or_else(|_| String::from(DEFAULT_CONFIG_PATH))
});

#[derive(Debug)]
pub struct Settings {
    pub action: Option<Actions>,
    pub user: Option<User>,
    pub tally_dir: String,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            tally_dir: String::from(DEFAULT_TALLY_DIR),
            action: None,
            user: None,
        }
    }
}

impl Settings {
    pub fn build(username: String, args: Vec<&CStr>, _flags: PamFlag) -> Settings {
        // Load INI file.
        let mut opts = Ini::load_from_file(PathBuf::from(CONFIG_PATH.to_string()).as_path())
            .ok()
            // Clone "Settings" section if it exists.
            .and_then(|ini| {
                ini.section(Some("Settings"))
                    .map(|settings| settings.clone())
            })
            // Map section to Settings struct, defaulting "tally_dir" if absent.
            .map(|settings| Settings {
                tally_dir: settings
                    .get("tally_dir")
                    .map(|s| s.to_string())
                    .unwrap_or_default(),
                ..Settings::default()
            })
            // Fallback to default Settings if any failures.
            .unwrap_or_else(|| Settings::default());

        // create possible action collection
        let action_map: HashMap<&str, Actions> = [
            ("preauth", Actions::PREAUTH),
            ("authsucc", Actions::AUTHSUCC),
            ("authfail", Actions::AUTHFAIL),
        ]
        .iter()
        .copied()
        .collect();

        // map argument to action
        opts.action = args.iter().find_map(|&carg| {
            carg.to_str()
                .ok()
                .and_then(|arg| action_map.get(arg).cloned())
        });

        // get user
        opts.user = get_user_by_name(&username);

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
        let opts = Settings::build(USER_NAME.to_string(), args, flags);
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
        let opts = Settings::build(USER_NAME.to_string(), args, flags);
        assert!(opts.action.is_none(), "Expected action to be None");
        Ok(())
    }

    #[test]
    fn test_action_preauth() -> TestResult {
        set_test_conf_path();
        let args = [CStr::from_bytes_with_nul("preauth\0".as_bytes())?].to_vec();
        let flags: PamFlag = 0;
        let opts = Settings::build(USER_NAME.to_string(), args, flags);
        assert_eq!(
            opts.action,
            Some(Actions::PREAUTH),
            "Expected action to be Preauth"
        );
        Ok(())
    }

    #[test]
    fn test_action_authfail() -> TestResult {
        set_test_conf_path();
        let args = [CStr::from_bytes_with_nul("authfail\0".as_bytes())?].to_vec();
        let flags: PamFlag = 0;
        let opts = Settings::build(USER_NAME.to_string(), args, flags);
        assert_eq!(
            opts.action,
            Some(Actions::AUTHFAIL),
            "Expected action to be Authfail"
        );
        Ok(())
    }

    #[test]
    fn test_conf_load_uname_valid() -> TestResult {
        set_test_conf_path();

        let args: Vec<&CStr> = [].to_vec();
        let flags: PamFlag = 0;

        let opts = Settings::build(USER_NAME.to_string(), args, flags);
        assert!(opts.user.is_some(), "Expected user to be Some");
        Ok(())
    }

    #[test]
    fn test_conf_load_uname_invalid() -> TestResult {
        set_test_conf_path();

        let args: Vec<&CStr> = [].to_vec();
        let flags: PamFlag = 0;

        let opts = Settings::build("invalid".to_string(), args, flags);
        assert!(opts.user.is_none(), "Expected user to be none");
        Ok(())
    }
}
