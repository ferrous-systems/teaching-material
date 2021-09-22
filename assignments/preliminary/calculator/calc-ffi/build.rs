use std::error::Error;

use xshell::{cmd, write_file};

fn main() -> Result<(), Box<dyn Error>> {
    let header = cmd!("cbindgen --parse-dependencies --lang c").read()?;
    write_file("calc_ffi.h", header).unwrap();
    println!("cargo:rerun-if-changed=src/lib.rs");
    Ok(())
}
