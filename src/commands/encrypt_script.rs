use std::{fs, mem, slice};
use crate::cryptographer::process_dogenzaka;
use crate::file_workers::file::write_output_file;
use crate::file_workers::file_process_args::FileProcessArgs;

pub fn encrypt_script(args: FileProcessArgs) {
    let file_name = args.path.file_name().unwrap();
    println!("[Encrypting script file] {}", file_name.to_str().unwrap());

    let file_string = fs::read_to_string(&args.path).unwrap();
    let mut file_utf16 = file_string.encode_utf16().collect::<Vec<u16>>();
    file_utf16.push(0);

    // https://stackoverflow.com/a/30838655
    let encoded_content = unsafe {
        slice::from_raw_parts(
            file_utf16.as_ptr() as *const u8,
            file_utf16.len() * mem::size_of::<u16>(),
        )
    }.to_vec();

    let encrypted_content = process_dogenzaka(&encoded_content, args.key.as_str());

    write_output_file(&args, &encrypted_content, "e.script.a");
}