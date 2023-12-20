use anyhow::Ok;
use clap::{Parser, Subcommand};
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Write;
use xshell::{cmd, Shell};

const RUNNER: &str = " 
[target.x86_64-unknown-linux-gnu] \n\
runner = 'sudo -E'";

const ALIAS: &str = "[alias] \n\
xtask = 'run --package xtask --'";

fn set_and_remove_sudo_runner<F>(sudo_f: F)
where
    F: FnOnce(),
{
    // Open the file in append mode, creating it if it doesn't exist
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(".cargo/config.toml")
        .expect("Unable to open or create file");

    // Append the content to the file
    file.write_all(RUNNER.as_bytes())
        .expect("Unable to write to file");

    sudo_f();

    file = File::create(".cargo/config.toml").expect("Unable to create file");

    file.write_all(ALIAS.as_bytes())
        .expect("Unable to write to file");
}

/// pam-rampdelay development tool
#[derive(Parser, Debug)]
#[command(arg_required_else_help = true)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    // pam authentication integration test
    PamTest,
    Lint,
    Fix,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let sh = Shell::new()?;

    match &cli.command {
        Some(Commands::Lint) => {
            cmd!(sh, "cargo fmt --check").run()?;
            cmd!(sh, "cargo clippy").run()?;
        }
        Some(Commands::Fix) => {
            cmd!(sh, "cargo fmt").run()?;
            cmd!(sh, "cargo clippy --fix --allow-dirty").run()?;
        }
        Some(Commands::PamTest) => {
            cmd!(sh, "cargo build").run()?;
            cmd!(
                sh,
                "sudo cp target/debug/libpam_authramp.so /lib64/security"
            )
            .run()?;
            set_and_remove_sudo_runner(|| {
                let _ = cmd!(sh, "cargo test --test '*' -- --test-threads=1").run();
                let _ = cmd!(sh, "sudo rm -f /lib64/security/libpam_authramp.so").run();
            })
        }
        None => {}
    }
    Ok(())
}
