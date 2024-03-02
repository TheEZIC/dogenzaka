use std::fs;
use std::fs::File;
use std::io::{Write};
use std::path::PathBuf;
use crate::clear_extension::ClearExtension;

pub fn write_sha1_key_file(path: &PathBuf, sha1_key: &[u8]) -> () {
    let key_file_path = transform_sha1_key_path(path);
    let mut file = File::create(key_file_path).unwrap();

    file.write_all(sha1_key).unwrap();
}

pub fn read_sha1_key_file(path: &PathBuf) -> Vec<u8> {
    let key_file_path = transform_sha1_key_path(path);

    if !key_file_path.exists() || !key_file_path.is_file() {
        panic!("There is no key file. Try to decrypt file provided file first");
    }

    let content = fs::read(key_file_path).unwrap();

    content
}

fn transform_sha1_key_path(path: &PathBuf) -> PathBuf {
    let mut key_file_path = path.clone();
    key_file_path.clear_extension();
    key_file_path.set_extension("key.a");

    key_file_path
}