use std::fs::File;

fn main() -> io::Result<()> {
    let file = File::open("test")?;

    use_file(file);
}

fn use_file(f: File) {
    // File drops here
}