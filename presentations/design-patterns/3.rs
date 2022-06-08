use std::fs::File;
use std::path::Path;
use std::path::PathBuf;

fn main() {
    open_file(&"test");
    let path_buf = PathBuf::from("test");
    open_file(&path_buf);
}

fn open_file<P: AsRef<Path>>(p: &P) {
    let path = p.as_ref();
    let file = File::open(path);
}