mod settings;

extern crate dotenv_codegen;
extern crate ini;
extern crate once_cell;
extern crate pam;
extern crate users;

use pam::constants::{PamFlag, PamResultCode};
use pam::module::{PamHandle, PamHooks};
use pam::pam_try;
use settings::Settings;
use std::ffi::CStr;

// Action argument defines position in PAM stack
#[derive(Debug, Default, Clone, Copy, PartialEq)]
enum Actions {
    PREAUTH,
    AUTHSUCC,
    #[default]
    AUTHFAIL,
}

struct PamRampDelay;

pam::pam_hooks!(PamRampDelay);
impl PamHooks for PamRampDelay {
    fn sm_authenticate(pamh: &mut PamHandle, _args: Vec<&CStr>, _flags: PamFlag) -> PamResultCode {
        let settings = pam_try!(Settings::build(
            pam_try!(pamh.get_user(None)),
            _args,
            _flags
        ));

        match settings.action {
            Some(Actions::PREAUTH) | Some(Actions::AUTHSUCC) => PamResultCode::PAM_SUCCESS,
            Some(Actions::AUTHFAIL) => PamResultCode::PAM_AUTH_ERR,
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
