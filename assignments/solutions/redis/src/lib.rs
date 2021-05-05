use anyhow::Result;
pub use redis::Client;
use redis::Connection;
use redis::{Commands, IntoConnectionInfo};

pub struct RedisClient {
    connection: Connection,
}

impl RedisClient {
    pub fn new<T: IntoConnectionInfo>(params: T) -> Result<Self> {
        let client = redis::Client::open(params)?;
        Ok(RedisClient {
            connection: client.get_connection()?,
        })
    }
    pub fn set(&mut self, key: &str, data: &[u8]) -> Result<()> {
        self.connection.set(key, data)?;
        Ok(())
    }

    pub fn get(&mut self, key: &str) -> Result<Vec<u8>> {
        let result = self.connection.get(key)?;
        Ok(result)
    }
}
