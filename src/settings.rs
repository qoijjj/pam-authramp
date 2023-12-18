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
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            tally_dir: PathBuf::from(DEFAULT_TALLY_DIR),
            action: None,
            user: None,
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
        // Clone "Settings" section if it exists.
        .and_then(|ini| ini.section(Some("Settings")).cloned())
        // Map section to Settings struct, defaulting "tally_dir" if absent.
        .map(|settings| Settings {
            tally_dir: settings
                .get("tally_dir")
                .map(PathBuf::from)
                .unwrap_or_default(),
            ..Settings::default()
        })
        // Fallback to default Settings if any failures.
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

        // get user
        settings.user = user;

        if settings.action.is_none() {
            // TODO: log
            return Err(PamResultCode::PAM_AUTH_ERR);
        }

        if settings.user.is_none() {
            // TODO: log
            return Err(PamResultCode::PAM_SYSTEM_ERR);
        }

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
    // Test default settings
    fn test_default_settings() {
        let default_settings = Settings::default();

        // Check if default tally_dir is set
        assert_eq!(default_settings.tally_dir, PathBuf::from(DEFAULT_TALLY_DIR));

        // Check if action is None in default settings
        assert!(default_settings.action.is_none());

        // Check if user is None in default settings
        assert!(default_settings.user.is_none());
    }

    #[test]
    // Test building settings from an existing INI file
    fn test_build_settings_from_ini() {
        // Create a temporary directory
        let temp_dir = TempDir::new("test_build_settings_from_ini").unwrap();
        let ini_file_path = temp_dir.path().join("config.conf");

        // Create a sample INI file
        let mut i = Ini::new();
        i.with_section(Some("Settings"))
            .set("tally_dir", "/tmp/tally_dir");

        i.write_to_file(&ini_file_path).unwrap();

        // Mock command line arguments
        let args = [CStr::from_bytes_with_nul("preauth\0".as_bytes()).unwrap()].to_vec();
        let flags: PamFlag = 0;
        // Call build to create settings from the INI file
        let result = Settings::build(
            Some(User::new(9999, "test_user", 9999)),
            args,
            flags,
            Some(ini_file_path),
        );

        // Check if settings are created successfully
        assert!(result.is_ok());
        let settings = result.unwrap();

        // Check if tally_dir is read from the INI file
        assert_eq!(settings.tally_dir, PathBuf::from("/tmp/tally_dir"));

        // Check if action is mapped correctly from command line arguments
        assert_eq!(settings.action, Some(Actions::PREAUTH));

        // Check if user is retrieved successfully
        assert!(settings.user.is_some());
        assert_eq!(settings.user.unwrap().name(), "test_user");
    }

    #[test]
    // Test building settings with missing action
    fn test_build_settings_missing_action() {
        // Mock command line arguments with missing action
        let args = vec![];
        let flags: PamFlag = 0;

        // Call build with missing action
        let result = Settings::build(Some(User::new(9999, "test_user", 9999)), args, flags, None);

        // Check if an error is returned due to missing action
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), PamResultCode::PAM_AUTH_ERR);
    }

    #[test]
    // Test building settings with missing user
    fn test_build_settings_missing_user() {
        // Mock command line arguments
        let args = [CStr::from_bytes_with_nul("preauth\0".as_bytes()).unwrap()].to_vec();
        let flags: PamFlag = 0;

        // Call build with missing user
        let result = Settings::build(None, args, flags, None);

        // Check if an error is returned due to missing user
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), PamResultCode::PAM_SYSTEM_ERR);
    }
}
