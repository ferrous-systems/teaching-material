fn main() -> Result<(), Box<Error>> {
    something_dangerious()?;

    something_harmless();

    Ok(()) // Kinda like `return 0` in C
}