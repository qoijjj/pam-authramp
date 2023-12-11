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
        let mut action = Actions::AUTHFAIL;

        args.iter().for_each(|&carg| {
            let arg = carg.to_str().expect("Invalid Argument UTF-8");

            match arg {
                "preauth" => action = Actions::PREAUTH,
                "authsucc" => action = Actions::AUTHSUCC,
                "authfail" => action = Actions::AUTHFAIL,
                _ => (),
            }
        });

        Self {
            action,
            user: "".to_string(),
        }
    }
}

pam::pam_hooks!(PamRampDelay);
impl PamHooks for PamRampDelay {
    fn sm_authenticate(pamh: &mut PamHandle, _args: Vec<&CStr>, _flags: PamFlag) -> PamResultCode {
        let mut opts = Options::args_parse(pamh, _args, _flags);

        opts.user = pam_try!(pamh.get_user(None));

        match opts.action {
            Actions::PREAUTH => PamResultCode::PAM_SUCCESS,
            Actions::AUTHSUCC => PamResultCode::PAM_SUCCESS,
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
