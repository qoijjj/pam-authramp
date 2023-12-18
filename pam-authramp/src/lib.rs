mod settings;
mod tally;

extern crate chrono;
extern crate ini;
extern crate once_cell;
extern crate pam;
extern crate tempdir;
extern crate users;

use pam::constants::{PamFlag, PamResultCode};
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

pub struct PamRampDelay;

pam::pam_hooks!(PamRampDelay);
impl PamHooks for PamRampDelay {
    fn sm_authenticate(pamh: &mut PamHandle, _args: Vec<&CStr>, _flags: PamFlag) -> PamResultCode {
        let user = get_user_by_name(&pamh.get_user(None).unwrap());

        let settings = pam_try!(Settings::build(user, _args, _flags, None));

        let _tally = pam_try!(Tally::open(&settings));

        match settings.action {
            Some(Actions::PREAUTH) | Some(Actions::AUTHSUCC) => PamResultCode::PAM_SUCCESS,
            Some(Actions::AUTHFAIL) => {
                return PamResultCode::PAM_AUTH_ERR;
            }
            None => PamResultCode::PAM_AUTH_ERR,
        }
    }

    fn sm_setcred(_pamh: &mut PamHandle, _args: Vec<&CStr>, _flags: PamFlag) -> PamResultCode {
        println!("set credentials");
        PamResultCode::PAM_SUCCESS
    }

    fn acct_mgmt(_pamh: &mut PamHandle, _args: Vec<&CStr>, _flags: PamFlag) -> PamResultCode {
        println!("account management");
        PamResultCode::PAM_SUCCESS
    }
}
