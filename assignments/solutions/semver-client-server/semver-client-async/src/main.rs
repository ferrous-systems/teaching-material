use async_std::{
    future::timeout,
    io::prelude::*,
    net::{Shutdown, TcpStream},
    task::{self, sleep},
};
use log::{error, info, warn};
use rand::Rng;
use semver::{Crate, Program, SemVer};
use semver_api::{ApiResponse, Command, Update};
use std::{convert::TryInto, error::Error, time::Duration};

fn main() -> Result<(), Box<dyn Error>> {
    pretty_env_logger::init();
    let program_name = "hello_bin".to_string();
    let program = Program::new(program_name.clone());

    let commands = vec![
        Command::Put(Crate::Program(program)),
        Command::Update(Update {
            crate_name: "ertjwjbrkwrkerbwkhrba".to_string(),
            version: SemVer::new_short(1),
        }),
        Command::Update(Update {
            crate_name: program_name.clone(),
            version: SemVer::new_short(1),
        }),
        Command::Update(Update {
            crate_name: program_name.clone(),
            version: SemVer::new_short(2),
        }),
        Command::Update(Update {
            crate_name: program_name.clone(),
            version: SemVer::new_short(2),
        }),
        Command::Update(Update {
            crate_name: program_name.clone(),
            version: SemVer::new_short(3),
        }),
        Command::Update(Update {
            crate_name: program_name.clone(),
            version: SemVer::new_short(4),
        }),
        Command::Get(program_name.clone()),
    ];

    let erratic = true;

    task::block_on(async {
        let mut handles = vec![];
        for command in commands {
            if erratic {
                let h = task::spawn(async move {
                    match send_command_with_backoff(command, drop).await {
                        Ok(_) => {}
                        Err(e) => error!("{}", e),
                    }
                });
                handles.push(h);
            } else {
                match send_command_with_backoff(command, drop).await {
                    Ok(_) => {}
                    Err(e) => error!("{}", e),
                }
            }
        }

        for handle in handles {
            handle.await;
        }
    });

    Ok(())
}

// truncated backoff: do not back off further than `cap` iterations
fn backoff(iteration: usize, cap: usize) -> u64 {
    let exp = iteration.min(cap) as u32;
    let limit = 2u64.pow(exp) - 1;
    let mut rng = rand::thread_rng();
    rng.gen_range(0..limit)
}

async fn send_command_with_backoff(
    command: Command,
    handler: impl Fn(Option<String>) + Clone,
) -> anyhow::Result<()> {
    const TIMEOUT: Duration = Duration::from_millis(2000);
    const RETRY_FRAME_MS: u64 = 10;
    for i in 1..10 {
        let command_fut = send_command(command.clone(), handler.clone()); // <- no .await!
        let invocation = timeout(TIMEOUT, command_fut);
        match invocation.await {
            Ok(res) => {
                return res;
            }
            Err(_timeout) => {
                let delay = Duration::from_millis((backoff(i, 10) + 1) * RETRY_FRAME_MS);
                warn!("timeout - waiting for {:?}", delay);
                sleep(delay).await;
            }
        }
    }
    anyhow::bail!("too many timeouts - giving up")
}

async fn send_command(
    command: Command,
    handler: impl Fn(Option<String>) + Clone,
) -> anyhow::Result<()> {
    let mut command: String = command.try_into()?;
    info!("→ {}", command);
    command.push_str("\n");
    let port = std::env::var("PORT").unwrap_or("7878".to_string());
    let mut connection = TcpStream::connect(format!("127.0.0.1:{}", port)).await?;
    connection.write(command.as_bytes()).await?;
    connection.shutdown(Shutdown::Write)?;
    let mut buffer = String::new();
    connection.read_to_string(&mut buffer).await?;
    let result: ApiResponse = buffer.as_str().try_into()?;
    match result.0 {
        Ok(payload) => {
            info!("← OK {:?}", payload);
            handler(payload)
        }
        Err(error) => error!("← ERR {:?}", error),
    }
    Ok(())
}
