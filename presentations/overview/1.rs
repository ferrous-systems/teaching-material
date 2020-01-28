use std::io; <1>
use std::fs::File;

fn main() -> Result<(), io::Error> { <2>
    let open_file = File::open("test"); <3>

    let mut file = match open_file { <4>
        Ok(file) => file,
        Err(e) => return io::Error::from(e)
    };

    let mut buffer = String::new(); <5>
    file.read_to_string(&mut buffer)?; <6>
    println!("{}", buffer);

    Ok(()) <7>
}
