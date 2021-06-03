use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    maybe_dangerous()?;
    Ok(())
}
