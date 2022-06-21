fn main() {
    let byte_string = b"allows ASCII and \xF0\x9F\x98\x80 only";
    println!("Can't be 'Display'ed {:?}", byte_string);
    if let Ok(string) = std::str::from_utf8(byte_string) {
        println!("Now can Display '{}'", string);
    }
}