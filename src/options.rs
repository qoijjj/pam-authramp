extern crate configparser;
extern crate serde;

use std::default::Default;
use std::ffi::CStr;

use self::configparser::ini::Ini;
use crate::Actions;
use pam::{constants::PamFlag, module::PamResult};
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Options {
    pub action: Actions,
    pub user: String,
    pub tally_dir: String,
}

impl Options {
    pub fn conf_load(user: String) -> PamResult<Options> {
        let mut opts = Options::default();
        let mut config = Ini::new();
        opts.user = user;
        Ok(opts)
    }

    pub fn args_parse(&mut self, args: Vec<&CStr>, _flags: PamFlag) -> PamResult<()> {
        // init action argument
        self.action = args
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
        Ok(())
    }
}
