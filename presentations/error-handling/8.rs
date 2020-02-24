use std::fs::File;

type DynError = Box<dyn std::error::Error + Send + Sync + 'static>;

fn main() -> Result<(), DynError> {
    let f = File::open("broken")?;

    let x: i32 = "ABC".parse()?;

    Ok(())
}
