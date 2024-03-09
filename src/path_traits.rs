use std::ffi::OsStr;
use std::path::PathBuf;

pub trait ClearExtension {
    fn clear_extension(&mut self) -> ();
}

impl ClearExtension for PathBuf {
    fn clear_extension(&mut self) -> () {
        match self.extension() {
            Some(_) => {
                self.set_extension("");
                self.clear_extension();
            }
            None => ()
        };
    }
}

pub trait ReplaceExtension {
    fn replace_extension<S: AsRef<OsStr>>(&mut self, extension: S) -> ();
}

impl ReplaceExtension for PathBuf {
    fn replace_extension<S: AsRef<OsStr>>(&mut self, extension: S) -> () {
        self.clear_extension();
        self.set_extension(extension);
    }
}