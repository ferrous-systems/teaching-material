use prost::Message;
mod phone;
use anyhow::Result;
use phone::config::Config;
use phone::phonebook::{Entry, PhoneBook};
use redis::IntoConnectionInfo;
use redis_client::RedisClient;

struct ProtoBufRedisClient {
    redis_client: RedisClient,
}

impl ProtoBufRedisClient {
    fn new<T: IntoConnectionInfo>(params: T) -> Result<Self> {
        let redis_client = RedisClient::new(params)?;
        Ok(Self { redis_client })
    }
}

impl ProtoBufRedisClient {
    const KEY_CONFIG: &'static str = "CONFIG";
    const KEY_PHONE_BOOK: &'static str = "PHONE_BOOK";

    fn get_phone_book(&mut self) -> Result<PhoneBook> {
        let encoded = self.redis_client.get(Self::KEY_PHONE_BOOK)?;
        let phone_book = PhoneBook::decode(encoded.as_slice())?;
        Ok(phone_book)
    }

    fn set_phone_book(&mut self, phone_book: &PhoneBook) -> Result<()> {
        let mut buf = Vec::with_capacity(phone_book.encoded_len());
        phone_book.encode(&mut buf)?;
        self.redis_client
            .set(Self::KEY_PHONE_BOOK, buf.as_slice())?;
        Ok(())
    }

    fn get_config(&mut self) -> Result<Config> {
        let encoded = self.redis_client.get(Self::KEY_CONFIG)?;
        let config = Config::decode(encoded.as_slice())?;
        Ok(config)
    }

    fn set_config(&mut self, config: &Config) -> Result<()> {
        let mut buf = Vec::with_capacity(config.encoded_len());
        config.encode(&mut buf)?;
        self.redis_client.set(Self::KEY_CONFIG, buf.as_slice())?;
        Ok(())
    }
}

fn main() -> Result<()> {
    let mut client = ProtoBufRedisClient::new("redis://127.0.0.1/")?;

    let mut entry = Entry::default();
    entry.name = "Hello Person".to_string();
    entry.number = "+46 70 1740605".to_string();
    entry.set_category(phone::phonebook::entry::Category::Personal);
    entry.set_kind(phone::phonebook::entry::Kind::Company);

    let mut phone_book = PhoneBook::default();
    phone_book.entries.push(entry);
    client.set_phone_book(&phone_book)?;

    let phone_book = client.get_phone_book()?;
    for e in &phone_book.entries {
        println!("{:?}", e);
    }

    let mut config = Config::default();
    config.set_mode(phone::config::config::Mode::Dnd);
    config.serial_number = "5387452476".to_string();

    client.set_config(&config)?;
    println!("{:?}", client.get_config()?);
    Ok(())
}
