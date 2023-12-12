use crate::Actions;
use pam::constants::PamFlag;
use std::ffi::CStr;

#[derive(Debug)]
pub struct Options {
    pub action: Actions,
    pub user: String,
    pub tally_dir: String,
}

impl Options {
    pub fn args_parse(args: Vec<&CStr>, _flags: PamFlag) -> Self {
        // init action argument
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
            tally_dir: String::new(),
        }
    }
}
