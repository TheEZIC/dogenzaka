use std::fs;
use std::fs::File;
use std::io::Write;
use encoding_rs::UTF_16LE;
use crate::cryptographer::process_dogenzaka;
use crate::file_workers::file_process_args::FileProcessArgs;
use crate::path_traits::ReplaceExtension;

pub fn decrypt_script(args: FileProcessArgs) {
    let file_name = args.path.file_name().unwrap();
    println!("[Decrypting script file] {}", file_name.to_str().unwrap());

    let file_bytes = fs::read(&args.path).unwrap();
    let decrypted_file_bytes = process_dogenzaka(&file_bytes, args.key.as_str());
    let (decoded_content, _encoding, _error) = UTF_16LE.decode(&decrypted_file_bytes);

    let mut output_path = args.output_path.clone();

    if args.path == args.output_path {
        output_path.replace_extension("d.a");
    }

    println!("{}", output_path.clone().to_str().unwrap());

    let mut file = File::create(output_path).unwrap();
    file.write_all(&decoded_content.as_bytes()).unwrap();
}