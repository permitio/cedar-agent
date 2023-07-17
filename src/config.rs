use fmt::Debug;
use std::borrow::Borrow;
use std::fmt;
use std::path::PathBuf;

use clap::Parser;
use log::LevelFilter;

use serde::{Deserialize, Serialize};

#[derive(Parser, Serialize, Deserialize, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Config {
    #[arg(short, long)]
    pub authentication: Option<String>,
    #[arg(long)]
    pub addr: Option<String>,
    #[arg(short, long)]
    pub port: Option<u16>,
    #[arg(short, long, value_enum)]
    pub log_level: Option<LevelFilter>,
    #[arg(short, long)]
    pub data: Option<PathBuf>,
    #[arg(long)]
    pub policy: Option<PathBuf>,
}

impl Into<rocket::figment::Figment> for &Config {
    fn into(self) -> rocket::figment::Figment {
        let mut config = rocket::Config::figment();
        if let Some(authentication) = self.authentication.borrow() {
            config = config.merge(("authentication", authentication));
        }
        if let Some(addr) = self.addr.borrow() {
            config = config.merge(("address", addr));
        }
        if let Some(port) = self.port.borrow() {
            config = config.merge(("port", port));
        } else {
            config = config.merge(("port", 8180))
        }
        if let Some(data) = self.data.borrow() {
            config = config.merge(("data", data));
        }
        if let Some(policy) = self.policy.borrow() {
            config = config.merge(("policy", policy));
        }

        config
    }
}

impl Config {
    fn new() -> Self {
        Config {
            authentication: None,
            addr: None,
            port: None,
            log_level: None,
            data: None,
            policy: None,
        }
    }

    fn merge(configs: Vec<Config>) -> Config {
        let mut config = Config::new();
        for c in configs {
            config.authentication = c.authentication.or(config.authentication);
            config.addr = c.addr.or(config.addr);
            config.port = c.port.or(config.port);
            config.log_level = c.log_level.or(config.log_level);
            config.data = c.data.or(config.data);
            config.policy = c.policy.or(config.policy);
        }

        config
    }

    fn from_args() -> Self {
        Self::parse()
    }

    fn from_env() -> Self {
        match envy::from_env() {
            Ok(env) => env,
            Err(_) => Self::new(),
        }
    }
}

pub fn init() -> Config {
    let args = Config::from_args();
    let env = Config::from_env();

    Config::merge(vec![args, env])
}
