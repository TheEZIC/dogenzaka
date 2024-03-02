use std::fs;
use std::fs::File;
use std::io::Write;
use encoding_rs::UTF_16LE;
use crate::cryptographer::encrypt_dogenzaka;
use crate::file_workers::file_process_args::FileEncryptArgs;
use crate::clear_extension::ClearExtension;

pub fn encrypt_script(args: FileEncryptArgs) {
    let file_name = args.path.file_name().unwrap();
    println!("[Encrypting script file] {}", file_name.to_str().unwrap());

    let file_string = fs::read_to_string(&args.path).unwrap();
    let file_str = file_string.into_boxed_str();

    let mut encoder = UTF_16LE.new_encoder();
    let mut encoded_content = vec![0u8; file_str.len()];
    let _ = encoder.encode_from_utf8(&file_str, &mut encoded_content, false);

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