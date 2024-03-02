use std::path::PathBuf;
use std::str;
use rc4::{KeyInit, Rc4, StreamCipher, Key};
use rc4::consts::{U16};
use sha1::{Sha1, Digest};
use crate::file_workers::sha1_key::*;

pub fn decrypt_dogenzaka(path: &PathBuf, data: &Vec<u8>, key: &str) -> Vec<u8> {
    let mut content = data.clone();

    let mut hasher = Sha1::new();
    hasher.update(key.as_bytes());
    let hashed_key = hasher.finalize().to_vec();
    let hashed_key = &hashed_key[0..16];

    write_sha1_key_file(path, hashed_key);

    let rc4_key = Key::<U16>::from_slice(hashed_key);
    let mut rc4 = Rc4::new(rc4_key);
    rc4.apply_keystream(&mut content);

    content
}

pub  fn encrypt_dogenzaka(path: &PathBuf, data: &Vec<u8>) -> Vec<u8> {
    let mut content = data.clone();

    let hashed_key = read_sha1_key_file(path);

    let rc4_key = Key::<U16>::from_slice(&hashed_key[0..16]);
    let mut rc4 = Rc4::new(rc4_key);
    rc4.apply_keystream(&mut content);

    content
}