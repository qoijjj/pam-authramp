mod settings;
mod tally;

extern crate chrono;
extern crate ini;
extern crate once_cell;
extern crate pam;
extern crate tempdir;
extern crate users;

use chrono::{Duration, Utc};
use pam::constants::{PamFlag, PamResultCode, PAM_ERROR_MSG};
use pam::conv::Conv;
use pam::module::{PamHandle, PamHooks};
use pam::pam_try;
use settings::Settings;
use std::ffi::CStr;

use std::thread::sleep;
use tally::Tally;
use users::get_user_by_name;

// Action argument defines position in PAM stack
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub enum Actions {
    PREAUTH,
    AUTHSUCC,
    #[default]
    AUTHFAIL,
}

fn init_rampdelay<F, R>(
    pamh: &mut PamHandle,
    _args: Vec<&CStr>,
    _flags: PamFlag,
    callback: F,
) -> Result<R, PamResultCode>
where
    F: FnOnce(&mut PamHandle, &Settings, &Tally) -> Result<R, PamResultCode>,
{
    // Initialize variables
    let user = get_user_by_name(pam_try!(
        &pamh.get_user(None),
        Err(PamResultCode::PAM_AUTH_ERR)
    ));
    let settings = Settings::build(user.clone(), _args, _flags, None)?;
    let tally = Tally::open(&settings)?;

    callback(pamh, &settings, &tally)
}

// delay=multi(tries-free)⋅log(tries-free)+base
fn calc_delay(fails: i32, settings: &Settings) -> f64 {
    settings.ramp_multiplier as f64
        * (fails as f64 - settings.free_tries as f64)
        * ((fails as f64 - settings.free_tries as f64).ln())
        + settings.base_delay_seconds as f64
}

fn fmt_remaining_time(remaining_time: Duration) -> String {
    let mut formatted_time = String::new();

    if remaining_time.num_hours() > 0 {
        formatted_time.push_str(&format!("{} hours ", remaining_time.num_hours()));
    }

    if remaining_time.num_minutes() > 0 {
        formatted_time.push_str(&format!("{} minutes ", remaining_time.num_minutes() % 60));
    }

    formatted_time.push_str(&format!("{} seconds", remaining_time.num_seconds() % 60));

    formatted_time
}

fn bounce_auth(pamh: &mut PamHandle, settings: &Settings, tally: &Tally) -> PamResultCode {
    if tally.failures_count > settings.free_tries {
        if let Ok(Some(conv)) = pamh.get_item::<Conv>() {
            let delay = calc_delay(tally.failures_count, settings);

            // Calculate the time when the account will be unlocked
            let unlock_time = tally.failure_instant + Duration::seconds(delay as i64);

            while Utc::now() < unlock_time {
                // Calculate remaining time until unlock
                let remaining_time = unlock_time - Utc::now();

                // Send a message to the conversation function
                let _ = conv.send(
                    PAM_ERROR_MSG,
                    &format!(
                        "Account locked! Unlocking in {}.",
                        fmt_remaining_time(remaining_time)
                    ),
                );

                // Wait for one second
                sleep(std::time::Duration::from_secs(1));
            }

            // Account is now unlocked, continue with PAM_SUCCESS
            return PamResultCode::PAM_SUCCESS;
        }
    }

    // Account is not locked or an error occurred, return PAM_AUTH_ERR
    PamResultCode::PAM_AUTH_ERR
}
pub struct PamRampDelay;

pam::pam_hooks!(PamRampDelay);
impl PamHooks for PamRampDelay {
    fn sm_authenticate(pamh: &mut PamHandle, args: Vec<&CStr>, flags: PamFlag) -> PamResultCode {
        init_rampdelay(pamh, args, flags, |pamh, settings, tally| {
            match settings.action {
                Some(Actions::PREAUTH) => {
                    if tally.failures_count > settings.free_tries {
                        Err(bounce_auth(pamh, settings, tally))
                    } else {
                        Ok(PamResultCode::PAM_SUCCESS)
                    }
                }
                Some(Actions::AUTHFAIL) => Err(bounce_auth(pamh, settings, tally)),
                None | Some(Actions::AUTHSUCC) => Err(PamResultCode::PAM_AUTH_ERR),
            }
        })
        .unwrap_or(PamResultCode::PAM_SUCCESS)
    }

    fn acct_mgmt(pamh: &mut PamHandle, args: Vec<&CStr>, flags: PamFlag) -> PamResultCode {
        pam_try!(init_rampdelay(
            pamh,
            args,
            flags,
            |_pamh, _settings, _tally| { Ok(PamResultCode::PAM_SUCCESS) }
        ))
    }
}
