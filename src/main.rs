use std::process::ExitCode;
use crate::cli::Cli;

mod cli;
mod cryptographer;
mod file_workers;
mod commands;
mod path_traits;

fn main() -> ExitCode {
    Cli::new().run();
    ExitCode::SUCCESS
}
