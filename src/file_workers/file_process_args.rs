use std::path::PathBuf;

pub struct FileProcessArgs {
    pub path: PathBuf,
    pub output_path: PathBuf,
    pub key: String,
}
