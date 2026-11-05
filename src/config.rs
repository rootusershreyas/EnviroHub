use std::{env, net::SocketAddr};

pub struct Config {
    pub bind_addr: SocketAddr,
    pub database_url: String,
    pub database_max_connections: u32,
}

impl Config {
    pub fn from_env() -> Result<Self, Box<dyn std::error::Error>> {
        let bind_addr = match env::var("BIND_ADDR") {
            Ok(value) => value.parse()?,
            Err(env::VarError::NotPresent) => "127.0.0.1:3000".parse()?,
            Err(error) => return Err(error.into()),
        };
        let database_url = env::var("DATABASE_URL")?;
        if database_url.trim().is_empty() {
            return Err("DATABASE_URL must not be empty".into());
        }
        let database_max_connections = match env::var("DATABASE_MAX_CONNECTIONS") {
            Ok(value) => value.parse()?,
            Err(env::VarError::NotPresent) => 10,
            Err(error) => return Err(error.into()),
        };
        if database_max_connections == 0 {
            return Err("DATABASE_MAX_CONNECTIONS must be greater than zero".into());
        }
        Ok(Self {
            bind_addr,
            database_url,
            database_max_connections,
        })
    }
}
