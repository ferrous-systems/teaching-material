use std::error::Error;

fn main() -> Result<(), Box<Error>> {
    maybe_dangerous()?;
    Ok(())
}
