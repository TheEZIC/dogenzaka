use std::fs;
use crate::cryptographer::process_dogenzaka;
use crate::file_workers::file::write_output_file;
use crate::file_workers::file_process_args::FileProcessArgs;

pub fn decrypt_raw(args: FileProcessArgs) {
    let file_name = args.path.file_name().unwrap();
    println!("[Decrypting raw file] {}", file_name.to_str().unwrap());

    let file_bytes = fs::read(&args.path).unwrap();
    let decrypted_file_bytes = process_dogenzaka(&file_bytes, args.key.as_str());

    write_output_file(&args, &decrypted_file_bytes, "d.raw.a");
}