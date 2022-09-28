use std::io::Read;
use std::fs::File;

fn main() {

    let open_result = File::open("src/data/content.txt");

    let mut file = match open_result {
        Ok(file) => file,
        Err(e) => panic!("Problem opening the file: {:?}", e),
    };

    let mut content_string = String::new();
    file.read_to_string(&mut content_string).unwrap();
    println!("{}", content_string);
}
