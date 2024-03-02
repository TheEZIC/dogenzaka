use std::path::PathBuf;

pub struct FileDecryptArgs {
    pub path: PathBuf,
    pub output_path: PathBuf,
    pub key: String,
}

pub struct FileEncryptArgs {
    pub path: PathBuf,
    pub output_path: PathBuf,
}
