use std::{fs, mem, slice};
use std::fs::File;
use std::io::Write;
use crate::cryptographer::encrypt_dogenzaka;
use crate::file_workers::file_process_args::FileEncryptArgs;
use crate::clear_extension::ClearExtension;

pub fn encrypt_script(args: FileEncryptArgs) {
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

    let encrypted_content = encrypt_dogenzaka(&args.path, &encoded_content);

    let mut output_path = args.output_path.clone();

    if args.path == args.output_path {
        output_path.clear_extension();
        output_path.set_extension("e.a");
    }

    println!("{}", output_path.clone().to_str().unwrap());

    let mut file = File::create(output_path).unwrap();
    file.write_all(&encrypted_content).unwrap();
}