use std::error::Error;

use xshell::{cmd, pushd, write_file};

fn main() -> Result<(), Box<dyn Error>> {
    #[allow(unused_must_use)]
    {
        let _p = pushd("calc-ffi")?;
        cmd!("cargo build").run()?;
        let header = cmd!("cbindgen --parse-dependencies --lang c").read()?;
        write_file("calc_ffi.h", header);
        // -l:exact_file_name doesn't work under macos ;<
        //cmd!("gcc -I. ./demo.c -L../target/debug -l:libcalc_ffi.a -lpthread -ldl -o demo").run()?;
        cmd!("gcc -I. ./demo.c -L../target/debug -lcalc_ffi -lpthread -ldl -o demo").run()?;
    }
    Ok(())
}
