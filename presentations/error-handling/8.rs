use std::fs::File;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let f = File::open("broken")?;

    let x: i32 = "ABC".parse()?;

    Ok(())
}
