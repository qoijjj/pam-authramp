use std::{fs, path::PathBuf};

use crate::{settings::Settings, Actions};
use chrono::{DateTime, Utc};
use ini::Ini;
use pam::constants::PamResultCode;

#[derive(Debug, PartialEq)]
pub struct Tally {
    pub tally_file: Option<PathBuf>,
    pub failures_count: i32,
    pub failure_instant: DateTime<Utc>,
}

impl Default for Tally {
    fn default() -> Self {
        Tally {
            tally_file: None,
            failures_count: 0,
            failure_instant: Utc::now(),
        }
    }
}

impl Tally {
    pub fn open(settings: &Settings) -> Result<Self, PamResultCode> {
        let mut tally = Tally::default();
        let user = settings.user.as_ref().ok_or(PamResultCode::PAM_AUTH_ERR)?;
        let tally_file = settings.tally_dir.join(user.name());

        // Check if the file exists
        let result = if tally_file.exists() {
            // If the file exists, attempt to load values from it
            Ini::load_from_file(&tally_file)
                .map_err(|_| PamResultCode::PAM_SYSTEM_ERR)
                .and_then(|i| {
                    // If the "Fails" section exists, extract and set values
                    if let Some(fails_section) = i.section(Some("Fails")) {
                        if let Some(count) = fails_section.get("count") {
                            tally.failures_count = count.parse().unwrap_or(0);
                        }
                        if let Some(instant) = fails_section.get("instant") {
                            tally.failure_instant = instant.parse().unwrap_or_default();
                        }
                        // Handle specific actions based on settings.action
                        match settings.action {
                            Some(Actions::AUTHSUCC) => {
                                // If action is AUTHFAIL, update count
                                tally.failures_count = 0;

                                // Write the updated values back to the file
                                let mut i = Ini::new();
                                i.with_section(Some("Fails"))
                                    .set("count", tally.failures_count.to_string());

                                i.write_to_file(&tally_file)
                                    .map_err(|_| PamResultCode::PAM_SYSTEM_ERR)?;
                            }
                            Some(Actions::AUTHFAIL) => {
                                // If action is AUTHFAIL, update count and instant
                                tally.failures_count += 1;
                                tally.failure_instant = Utc::now();
                                // Write the updated values back to the file
                                let mut i = Ini::new();
                                i.with_section(Some("Fails"))
                                    .set("count", tally.failures_count.to_string())
                                    .set("instant", tally.failure_instant.to_string());

                                i.write_to_file(&tally_file)
                                    .map_err(|_| PamResultCode::PAM_SYSTEM_ERR)?;
                            }
                            _ => {}
                        }

                        Ok(())
                    } else {
                        // If the section doesn't exist, return an error
                        Err(PamResultCode::PAM_SYSTEM_ERR)
                    }
                })
        } else {
            // If the file doesn't exist, create it
            fs::create_dir_all(tally_file.parent().unwrap())
                .map_err(|_| PamResultCode::PAM_SYSTEM_ERR)?;

            let mut i = Ini::new();
            i.with_section(Some("Fails"))
                .set("count", tally.failures_count.to_string())
                .set("instant", tally.failure_instant.to_string());

            // Write the INI file to disk
            i.write_to_file(&tally_file)
                .map_err(|_| PamResultCode::PAM_SYSTEM_ERR)?;

            Ok(())
        };

        // Map the final result to the Tally structure
        result.map(|_| tally)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempdir::TempDir;
    use users::User;

    #[test]
    fn test_open_existing_tally_file() {
        // Create a temporary directory
        let temp_dir = TempDir::new("test_open_existing_tally_file").unwrap();
        let tally_file_path = temp_dir.path().join("test_user_a");

        // Create an existing INI file
        let mut i = Ini::new();
        i.with_section(Some("Fails"))
            .set("count", "42")
            .set("instant", "2023-01-01T00:00:00Z");

        i.write_to_file(tally_file_path).unwrap();

        // Create settings and call open
        let settings = Settings {
            user: Some(User::new(9999, "test_user_a", 9999)),
            tally_dir: temp_dir.path().to_path_buf(),
            action: Some(Actions::PREAUTH),
            ..Default::default()
        };

        // Test: Open existing tally file
        let result = Tally::open(&settings);

        // Check if the Tally struct is created with expected values
        assert!(result.is_ok());
        let tally = result.unwrap();
        assert_eq!(tally.failures_count, 42);
        assert_eq!(
            tally.failure_instant,
            DateTime::parse_from_rfc3339("2023-01-01T00:00:00Z").unwrap()
        );
    }

    #[test]
    fn test_open_nonexistent_tally_file() {
        // Create a temporary directory
        let temp_dir = TempDir::new("test_open_nonexistent_tally_file").unwrap();
        let tally_file_path = temp_dir.path().join("test_user_b");

        // Create settings and call open
        let settings = Settings {
            user: Some(User::new(9999, "test_user_b", 9999)),
            tally_dir: temp_dir.path().to_path_buf(),
            ..Default::default()
        };

        // Test: Open nonexistent tally file
        let result = Tally::open(&settings);

        // Check if the Tally struct is created with default values
        assert!(result.is_ok());
        let tally = result.unwrap();
        assert_eq!(tally.failures_count, 0);

        // Check if the INI file has been created with default values
        let ini_content = fs::read_to_string(tally_file_path).unwrap();
        assert!(ini_content.contains("[Fails]"));
        assert!(ini_content.contains("count=0"));
    }

    #[test]
    fn test_open_auth_fail_updates_values() {
        // Create a temporary directory
        let temp_dir = TempDir::new("test_open_auth_fail_updates_values").unwrap();
        let tally_file_path = temp_dir.path().join("test_user_c");

        // Create an existing INI file with some initial values
        let mut i = Ini::new();
        i.with_section(Some("Fails"))
            .set("count", "2")
            .set("instant", "2023-01-01T00:00:00Z");
        i.write_to_file(&tally_file_path).unwrap();

        // Create settings and call open with AUTHFAIL action
        let settings = Settings {
            user: Some(User::new(9999, "test_user_c", 9999)),
            tally_dir: temp_dir.path().to_path_buf(),
            action: Some(Actions::AUTHFAIL),
            free_tries: 6,
            ramp_multiplier: 50,
            base_delay_seconds: 30,
        };

        let tally = Tally::open(&settings).unwrap();

        // Check if the values are updated on AUTHFAIL
        assert_eq!(tally.failures_count, 3); // Assuming you increment the count
                                             // Also, assert that the instant is updated to the current time

        // Optionally, you can assert that the file is updated
        let ini_content = fs::read_to_string(&tally_file_path).unwrap();
        assert!(ini_content.contains("count=3"));
        // Also, assert the instant value in the INI file

        // Additional assertions as needed
    }

    #[test]
    fn test_open_auth_succ_resets_tally() {
        // Create a temporary directory
        let temp_dir = TempDir::new("test_open_auth_succ_deletes_file").unwrap();
        let tally_file_path = temp_dir.path().join("test_user_d");

        // Create an existing INI file
        let mut i = Ini::new();
        i.with_section(Some("Fails"))
            .set("count", "2")
            .set("instant", "2023-01-01T00:00:00Z");
        i.write_to_file(&tally_file_path).unwrap();

        // Create settings and call open with AUTHSUCC action
        let settings = Settings {
            user: Some(User::new(9999, "test_user_d", 9999)),
            tally_dir: temp_dir.path().to_path_buf(),
            action: Some(Actions::AUTHSUCC),
            free_tries: 6,
            ramp_multiplier: 50,
            base_delay_seconds: 30,
        };

        let _tally = Tally::open(&settings).unwrap();

        // Expect tally count to decrease
        let ini_content = fs::read_to_string(&tally_file_path).unwrap();
        assert!(ini_content.contains("count=0"), "Expected tally count = 0");
    }
}
