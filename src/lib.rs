mod settings;
mod tally;

extern crate chrono;
extern crate ini;
extern crate once_cell;
extern crate pam;
extern crate tempdir;
extern crate users;

use pam::constants::{PamFlag, PamResultCode, PAM_ERROR_MSG};
use pam::conv::Conv;
use pam::module::{PamHandle, PamHooks};
use pam::pam_try;
use settings::Settings;
use std::ffi::CStr;
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

fn bounce_auth(pamh: &mut PamHandle, settings: &Settings, tally: &Tally) -> PamResultCode {
    if tally.failures_count > settings.free_tries {
        if let Ok(Some(conv)) = pamh.get_item::<Conv>() {
            pam_try!(conv.send(PAM_ERROR_MSG, "Account locked!"));
        }
    }
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
        init_rampdelay(pamh, args, flags, |pamh, settings, tally| {
            if tally.failures_count > settings.free_tries {
                Err(bounce_auth(pamh, settings, tally))
            } else {
                Ok(PamResultCode::PAM_SUCCESS)
            }
        })
        .unwrap_or(PamResultCode::PAM_SUCCESS)
    }
}
