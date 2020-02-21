fn read_file(path: &std) -> Result<String, io::Error> {
     let f = File::open(path)?;

     let mut buffer = String::new();
     f.read_to_end(&mut buffer)?;

     Ok(buffer)
}
