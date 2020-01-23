use std::fs::File;

fn main() {
    let file = File::open("test").unwrap();

    use_file(file);
}

fn use_file(f: File) {
    // File drops here
}