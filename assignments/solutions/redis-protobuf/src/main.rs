use prost::Message;
mod phone;
use anyhow::Result;
use phone::config::Config;
use phone::phonebook::{Entry, PhoneBook};
use redis_client::RedisClient;

const KEY_CONFIG: &'static str = "CONFIG";
const KEY_PHONE_BOOK: &'static str = "PHONE_BOOK";

fn main() -> Result<()> {
    let mut client = RedisClient::new("redis://127.0.0.1/")?;

    let mut entry = Entry::default();
    entry.name = "Hello Person".to_string();
    entry.number = "+49123456".to_string();
    entry.set_category(phone::phonebook::entry::Category::Personal);
    entry.set_kind(phone::phonebook::entry::Kind::Company);

    let mut pb = PhoneBook::default();
    pb.entries.push(entry);

    let mut buf = Vec::with_capacity(3);
    pb.encode(&mut buf)?;

    client.set(KEY_PHONE_BOOK, buf.as_slice())?;

    let pb_encoded = client.get(KEY_PHONE_BOOK)?;
    let pb = PhoneBook::decode(pb_encoded.as_slice())?;
    for e in &pb.entries {
        println!("{:?}", e);
    }
    Ok(())
}
