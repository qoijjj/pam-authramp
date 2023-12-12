extern crate pam;

mod options;

use pam::constants::{PamFlag, PamResultCode};
use pam::module::{PamHandle, PamHooks};
use pam::pam_try;
use std::ffi::CStr;

use options::Options;

// Action argument defines position in PAM stack
#[derive(Debug)]
enum Actions {
    PREAUTH,
    AUTHSUCC,
    AUTHFAIL,
}

struct PamRampDelay;

pam::pam_hooks!(PamRampDelay);
impl PamHooks for PamRampDelay {
    fn sm_authenticate(pamh: &mut PamHandle, _args: Vec<&CStr>, _flags: PamFlag) -> PamResultCode {
        let mut opts = Options::args_parse(_args, _flags);
        opts.user = pam_try!(pamh.get_user(None));

        match opts.action {
            Actions::PREAUTH | Actions::AUTHSUCC => PamResultCode::PAM_SUCCESS,
            Actions::AUTHFAIL => PamResultCode::PAM_AUTH_ERR,
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
