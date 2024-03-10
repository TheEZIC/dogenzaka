use std::fs;
use encoding_rs::UTF_16LE;
use crate::cryptographer::process_dogenzaka;
use crate::file_workers::file::{log_file_reading_started, write_output_file};
use crate::file_workers::file_process_args::FileProcessArgs;

pub fn decrypt_script(args: FileProcessArgs) {
    log_file_reading_started(&args.path, "[Decrypting script file]");

    let file_bytes = fs::read(&args.path).unwrap();
    let decrypted_file_bytes = process_dogenzaka(&file_bytes, args.key.as_str());
    let (decoded_content, _encoding, _error) = UTF_16LE.decode(&decrypted_file_bytes);

    write_output_file(&args, &decoded_content.as_bytes(), "d.script.a");
}