extern crate pam;

use pam::constants::{PamFlag, PamResultCode};
use pam::module::{PamHandle, PamHooks};
use pam::pam_try;
use std::ffi::CStr;
struct PamRampDelay;

#[derive(Debug)]
enum Actions {
    PREAUTH,
    AUTHSUCC,
    AUTHFAIL,
}

#[derive(Debug)]
pub struct Options {
    action: Actions,
    user: String,
}

impl Options {
    fn args_parse(_pamh: &mut PamHandle, args: Vec<&CStr>, _flags: PamFlag) -> Self {
        let action = args
            .iter()
            .find_map(|&carg| {
                let arg = carg.to_str().expect("Invalid Argument UTF-8");

                match arg {
                    "preauth" => Some(Actions::PREAUTH),
                    "authsucc" => Some(Actions::AUTHSUCC),
                    "authfail" => Some(Actions::AUTHFAIL),
                    _ => None,
                }
            })
            .unwrap_or(Actions::AUTHFAIL);

        Self {
            action,
            user: String::new(),
        }
    }
}

pam::pam_hooks!(PamRampDelay);
impl PamHooks for PamRampDelay {
    fn sm_authenticate(pamh: &mut PamHandle, _args: Vec<&CStr>, _flags: PamFlag) -> PamResultCode {
        let mut opts = Options::args_parse(pamh, _args, _flags);
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
