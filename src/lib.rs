extern crate pam;

use std::ffi::CStr;
use pam::module::{PamHandle, PamHooks};
use pam::constants::{PamResultCode, PamFlag};
struct PamRampDelay;

pam::pam_hooks!(PamRampDelay);
impl PamHooks for PamRampDelay {
  fn sm_authenticate(_pamh: &mut PamHandle, _args: Vec<&CStr>, _flags: PamFlag) -> PamResultCode {
    println!("authenticating");
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
