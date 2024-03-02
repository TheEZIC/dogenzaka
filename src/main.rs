use std::process::ExitCode;
use crate::cli::Cli;

mod cli;
mod cryptographer;
mod file_workers;
mod commands;
mod clear_extension;

fn main() -> ExitCode {
    Cli::new().run();
    ExitCode::SUCCESS
}
