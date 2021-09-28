use std::{
    error::Error,
    io::{Read, Write},
    net::{Shutdown, TcpStream},
};

use log::{error, info};
use semver::{Crate, Program, SemVer};
use semver_api::{ApiResult, Command, Update};

fn main() -> Result<(), Box<dyn Error>> {
    pretty_env_logger::init();
    let program_name = "hello_bin".to_string();
    let program = Program::new(program_name.clone());
    send_command(Command::Put(Crate::Program(program)), drop)?;
    send_command(
        Command::Update(Update {
            crate_name: "ertjwjbrkwrkerbwkhrba".to_string(),
            version: SemVer::new_short(1),
        }),
        drop,
    )?;

    send_command(
        Command::Update(Update {
            crate_name: program_name.clone(),
            version: SemVer::new_short(1),
        }),
        drop,
    )?;

    send_command(
        Command::Update(Update {
            crate_name: program_name.clone(),
            version: SemVer::new_short(2),
        }),
        drop,
    )?;

    send_command(
        Command::Update(Update {
            crate_name: program_name.clone(),
            version: SemVer::new_short(2),
        }),
        drop,
    )?;

    send_command(Command::Get(program_name.clone()), |crt| match crt {
        Some(crt) => {
            let c: Crate = crt.as_str().try_into().expect("deserialization failed");
            info!("got crate: {:#?}", c);
        }
        None => error!("load crate data failed"),
    })?;
    Ok(())
}

fn send_command<F: Fn(Option<String>)>(command: Command, handler: F) -> Result<(), Box<dyn Error>> {
    let command: String = command.try_into()?;
    info!("→ {}", command);
    let port = std::env::var("PORT").unwrap_or("7878".to_string());
    let mut connection = TcpStream::connect(format!("127.0.0.1:{}", port))?;
    writeln!(connection, "{}", command)?;
    connection.shutdown(Shutdown::Write)?;
    let mut buffer = String::new();
    connection.read_to_string(&mut buffer)?;
    let result: ApiResult = buffer.as_str().try_into()?;
    dbg!();
    match result.0 {
        Ok(payload) => {
            info!("← OK {:?}", payload);
            handler(payload)
        }
        Err(error) => error!("← ERR {:?}", error),
    }
    Ok(())
}
