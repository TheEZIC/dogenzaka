use std::fs::File;
use std::io::Write;
use crate::file_workers::file_process_args::FileProcessArgs;
use crate::path_traits::ReplaceExtension;

pub fn write_output_file(args: &FileProcessArgs, data: &[u8], extension: &str) {
    let mut output_path = args.output_path.clone();

    if args.path == args.output_path {
        output_path.replace_extension(extension);
    }

    println!("{}", output_path.clone().to_str().unwrap());

    let mut file = File::create(output_path).unwrap();
    file.write_all(data).unwrap();
}