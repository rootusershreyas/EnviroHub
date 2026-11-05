use std::{env, net::SocketAddr};

pub struct Config {
    pub bind_addr: SocketAddr,
}

impl Config {
    pub fn from_env() -> Result<Self, Box<dyn std::error::Error>> {
        let bind_addr = match env::var("BIND_ADDR") {
            Ok(value) => value.parse()?,
            Err(env::VarError::NotPresent) => "127.0.0.1:3000".parse()?,
            Err(error) => return Err(error.into()),
        };
        Ok(Self { bind_addr })
    }
}
