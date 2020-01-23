fn multiple_possible_failures() -> Result<String,String> {
    this_can_fail(true)?;
    println!("After 1st potential error.");
    this_can_fail(false)?;
    println!("After 2nd potential error.");
    Ok(String::from("All done."))
}

fn main() {
    multiple_possible_failures();
}