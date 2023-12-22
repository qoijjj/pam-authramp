use crate::Actions;
use ini::Ini;
use pam::constants::{PamFlag, PamResultCode};
use std::collections::HashMap;
use std::ffi::CStr;
use std::path::PathBuf;
use users::User;

const DEFAULT_TALLY_DIR: &str = "/var/run/rampdelay";
const DEFAULT_CONFIG_FILE_PATH: &str = "/etc/security/rampdelay.conf";

#[derive(Debug)]
pub struct Settings {
    pub action: Option<Actions>,
    pub user: Option<User>,
    pub tally_dir: PathBuf,
    pub free_tries: i32,
    pub base_delay_seconds: i32,
    pub ramp_multiplier: i32,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            tally_dir: PathBuf::from(DEFAULT_TALLY_DIR),
            action: Some(Actions::AUTHSUCC),
            user: None,
            free_tries: 6,
            base_delay_seconds: 30,
            ramp_multiplier: 50,
        }
    }
}

impl Settings {
    pub fn build(
        user: Option<User>,
        args: Vec<&CStr>,
        _flags: PamFlag,
        _config_file: Option<PathBuf>,
    ) -> Result<Settings, PamResultCode> {
        // Load INI file.
        let mut settings = Ini::load_from_file(
            _config_file.unwrap_or_else(|| PathBuf::from(DEFAULT_CONFIG_FILE_PATH)),
        )
        .ok()
        .and_then(|ini| ini.section(Some("Settings")).cloned())
        .map(|settings| Settings {
            tally_dir: settings
                .get("tally_dir")
                .map(PathBuf::from)
                .unwrap_or_default(),
            free_tries: settings
                .get("free_tries")
                .and_then(|val| val.parse().ok())
                .unwrap_or_default(),
            base_delay_seconds: settings
                .get("base_delay")
                .and_then(|val| val.parse().ok())
                .unwrap_or_default(),
            ramp_multiplier: settings
                .get("ramp_multiplier")
                .and_then(|val| val.parse().ok())
                .unwrap_or_default(),
            ..Settings::default()
        })
        .unwrap_or_default();

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
        settings.action = args.iter().find_map(|&carg| {
            carg.to_str()
                .ok()
                .and_then(|arg| action_map.get(arg).cloned())
        });

        // set default action if none is provided
        settings.action.get_or_insert(Actions::AUTHSUCC);

        // get user
        settings.user = Some(user.ok_or(PamResultCode::PAM_SYSTEM_ERR)?);

        Ok(settings)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CStr;
    use tempdir::TempDir;
    use users::User;

    #[test]
    fn test_default_settings() {
        let default_settings = Settings::default();
        assert_eq!(default_settings.tally_dir, PathBuf::from(DEFAULT_TALLY_DIR));
        assert_eq!(default_settings.action, Some(Actions::AUTHSUCC));
        assert!(default_settings.user.is_none());
        assert_eq!(default_settings.free_tries, 6);
        assert_eq!(default_settings.base_delay_seconds, 30);
        assert_eq!(default_settings.ramp_multiplier, 50);
    }

    #[test]
    fn test_build_settings_from_ini() {
        let temp_dir = TempDir::new("test_build_settings_from_ini").unwrap();
        let ini_file_path = temp_dir.path().join("config.conf");

        let mut i = Ini::new();
        i.with_section(Some("Settings"))
            .set("tally_dir", "/tmp/tally_dir")
            .set("free_tries", "10")
            .set("base_delay", "15")
            .set("ramp_multiplier", "20");

        i.write_to_file(&ini_file_path).unwrap();

        let args = [CStr::from_bytes_with_nul("preauth\0".as_bytes()).unwrap()].to_vec();
        let flags: PamFlag = 0;

        let result = Settings::build(
            Some(User::new(9999, "test_user", 9999)),
            args,
            flags,
            Some(ini_file_path),
        );

        assert!(result.is_ok());
        let settings = result.unwrap();
        assert_eq!(settings.tally_dir, PathBuf::from("/tmp/tally_dir"));
        assert_eq!(settings.action, Some(Actions::PREAUTH));
        assert!(settings.user.is_some());
        assert_eq!(settings.user.unwrap().name(), "test_user");
        assert_eq!(settings.free_tries, 10);
        assert_eq!(settings.base_delay_seconds, 15);
        assert_eq!(settings.ramp_multiplier, 20);
    }

    #[test]
    fn test_build_settings_missing_action() {
        let args = vec![];
        let flags: PamFlag = 0;
        let result = Settings::build(Some(User::new(9999, "test_user", 9999)), args, flags, None);
        assert!(result.is_ok());
    }

    #[test]
    fn test_build_settings_missing_user() {
        let args = [CStr::from_bytes_with_nul("preauth\0".as_bytes()).unwrap()].to_vec();
        let flags: PamFlag = 0;
        let result = Settings::build(None, args, flags, None);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), PamResultCode::PAM_SYSTEM_ERR);
    }
}
