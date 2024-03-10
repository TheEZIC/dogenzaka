use std::path::{PathBuf};

use clap::{Arg, ArgMatches, Command, value_parser};
use clap::parser::ValueSource;

use crate::{
    commands::{
        decrypt_script::*,
        encrypt_script::*,
        decrypt_raw::*,
        encrypt_raw::*,
    },
    file_workers::file_process_args::*,
};

pub struct Cli {
    app: Command,
}

impl Cli {
    pub fn new() -> Cli {
        let path_arg = Self::create_path_arg();
        let output_arg = Self::create_output_arg();
        let key_arg = Self::create_key_arg();

        let app = Command::new("Dogenzaka")
            .about("Program to work with Dogenzaka engine files")
            .author("TheEZIC")
            .arg_required_else_help(true)
            .subcommand(
                Command::new("decrypt-script")
                    .about("Decrypts provided dogenzaka script")
                    .arg(&path_arg)
                    .arg(&output_arg)
                    .arg(&key_arg)
            )
            .subcommand(
                Command::new("encrypt-script")
                    .about("Encrypts provided dogenzaka script")
                    .arg(&path_arg)
                    .arg(&output_arg)
                    .arg(&key_arg)
            )
            .subcommand(
                Command::new("decrypt-raw")
                    .about("Decrypts provided dogenzaka raw binary")
                    .arg(&path_arg)
                    .arg(&output_arg)
                    .arg(&key_arg)
            )
            .subcommand(
                Command::new("encrypt-raw")
                    .about("Encrypts provided dogenzaka raw binary")
                    .arg(&path_arg)
                    .arg(&output_arg)
                    .arg(&key_arg)
            );

        Cli { app }
    }

    pub fn run(&self) {
        let matches = self.app.clone().get_matches();

        match matches.subcommand() {
            Some((command, sub_matches)) => {
                let path = Self::parse_path_arg(sub_matches);
                let output_path = Self::parse_output_path_arg(sub_matches, &path);
                let key = Self::parse_string_arg(sub_matches, "key");

                let args = FileProcessArgs {
                    path,
                    output_path,
                    key,
                };

                match command {
                    "decrypt-script" => decrypt_script(args),
                    "encrypt-script" => encrypt_script(args),
                    "decrypt-raw" => decrypt_raw(args),
                    "encrypt-raw" => encrypt_raw(args),
                    _ => panic!("unknown command provided")
                };
            }
            _ => panic!("unknown command provided"),
        }
    }

    fn create_path_arg() -> Arg {
        Arg::new("path")
            .required(true)
            .value_parser(value_parser!(PathBuf))
    }

    fn create_output_arg() -> Arg {
        Arg::new("output")
            .short('o')
            .long("output")
            .aliases(["out"])
            .required(false)
            .value_parser(value_parser!(PathBuf))
    }

    fn create_key_arg() -> Arg {
        Arg::new("key")
            .short('k')
            .long("key")
            .value_parser(value_parser!(String))
            .default_value("Hlk9D28p")
    }

    fn parse_string_arg(sub_matches: &ArgMatches, id: &str) -> String {
        sub_matches.get_one::<String>(id)
            .unwrap()
            .to_owned()
    }

    fn parse_path_arg(sub_matches: &ArgMatches) -> PathBuf {
        let path = sub_matches.get_one::<PathBuf>("path")
            .unwrap()
            .to_owned();

        if !path.exists() {
            let file_name = path.to_str().unwrap();
            panic!("There is no such file {}", file_name);
        }

        return path;
    }

    fn parse_output_path_arg(sub_matches: &ArgMatches, path: &PathBuf) -> PathBuf {
        return match sub_matches.value_source("output") {
            Some(ValueSource::CommandLine | ValueSource::EnvVariable) => {
                sub_matches
                    .get_one::<PathBuf>("output")
                    .unwrap()
                    .to_owned()
            }
            _ => path.clone(),
        };
    }
}