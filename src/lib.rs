extern crate pam;

use pam::constants::{PamFlag, PamResultCode};
use pam::module::{PamHandle, PamHooks};
use std::ffi::CStr;
struct PamRampDelay;

#[derive(Debug)]
enum Actions {
    PREAUTH,
    AUTHSUCC,
    AUTHFAIL,
}

#[derive(Debug)]
struct Options {
    action: Actions,
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
        Self { action }
    }
}

pam::pam_hooks!(PamRampDelay);
impl PamHooks for PamRampDelay {
    fn sm_authenticate(_pamh: &mut PamHandle, _args: Vec<&CStr>, _flags: PamFlag) -> PamResultCode {
        let opts = Options::args_parse(_pamh, _args, _flags);

        PamResultCode::PAM_SUCCESS
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
