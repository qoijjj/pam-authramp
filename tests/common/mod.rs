#[cfg(test)]
pub mod utilities {
    use std::{
        fs::{copy, remove_file},
        path::Path,
    };

    use pam_client::{conv_mock::Conversation, Context, Flag};

    const LIBRARY_PATH: &str = dotenv!("TEST_LIBRARY_PATH");
    const SERVICE_DIR: &str = dotenv!("TEST_SERVICE_DIR");

    pub type TestResult = Result<(), Box<dyn std::error::Error>>;

    fn copy_library() {
        let src_path = Path::new("target/release/libpam_rampdelay.so");
        let dest_path = Path::new(LIBRARY_PATH);

        copy(src_path, dest_path).expect("Failed to copy library");
    }

    fn delete_library() {
        let path = Path::new(LIBRARY_PATH);

        if path.exists() {
            remove_file(path).expect("Failed to remove library");
        }
    }

    fn copy_service(srv: &str) {
        let src_path = Path::new("tests/conf").join(srv);
        let dest_path = Path::new(SERVICE_DIR).join(srv);

        copy(src_path, dest_path).expect("Failed to copy service");
    }

    fn delete_service(srv: &str) {
        let path = Path::new(SERVICE_DIR).join(srv);

        if path.exists() {
            remove_file(path).expect("Failed to remove service");
        }
    }

    pub fn test_service(srv: &str, u_name: &str, u_pwd: &str) -> TestResult {
        copy_library();

        copy_service(srv);

        let mut ctx = Context::new(
            srv, // Service name
            None,
            Conversation::with_credentials(u_name, u_pwd),
        )?;

        ctx.authenticate(Flag::NONE)?;
        ctx.acct_mgmt(Flag::NONE)?;

        delete_library();
        delete_service(srv);

        Ok(())
    }
}
