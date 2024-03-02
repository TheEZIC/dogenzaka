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
