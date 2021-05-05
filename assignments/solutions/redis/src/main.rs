use anyhow::Result;
use my_redis::RedisClient;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
enum Action {
    Get { key: String },
    Set { key: String, value: String },
}

#[derive(Debug, StructOpt)]
struct Options {
    #[structopt(subcommand)]
    action: Action,

    #[structopt(long, default_value = "redis://127.0.0.1/")]
    host: String,
}

fn main() -> Result<()> {
    let options = Options::from_args();
    println!("{:?}", options);

    let mut c = RedisClient::new(options.host)?;

    match options.action {
        Action::Get { key } => {
            let value = c.get(&key)?;
            let s = std::str::from_utf8(value.as_slice())?;
            println!("value is {}", s);
        }
        Action::Set { key, value } => {
            c.set(&key, value.as_bytes())?;
            println!("setting {} to {}", key, value);
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    const KEY_DATA: &'static str = "DATA";

    #[test]
    fn test_roundtrip() -> Result<()> {
        let mut c2 = RedisClient::new("redis://127.0.0.1/")?;

        let data = b"Hello \x52\x75\x73\x74";
        c2.set(KEY_DATA, data)?;

        assert_eq!(data, c2.get(KEY_DATA)?.as_slice());
        Ok(())
    }
}
