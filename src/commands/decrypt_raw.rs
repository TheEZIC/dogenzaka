use std::fs;
use crate::cryptographer::process_dogenzaka;
use crate::file_workers::file::{log_file_reading_started, write_output_file};
use crate::file_workers::file_process_args::FileProcessArgs;

pub fn decrypt_raw(args: FileProcessArgs) {
    log_file_reading_started(&args.path, "[Decrypting raw file]");

    let file_bytes = fs::read(&args.path).unwrap();
    let decrypted_file_bytes = process_dogenzaka(&file_bytes, args.key.as_str());

    write_output_file(&args, &decrypted_file_bytes, "d.raw.a");
}