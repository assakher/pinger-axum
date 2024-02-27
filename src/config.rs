use std::{env, net::Ipv4Addr, str::FromStr};

use tracing::log::LevelFilter;

#[derive(Clone)]
pub struct State {
    pub config: Config,
}

#[derive(Clone)]
pub struct Config {
    pub log_level: LevelFilter,
    pub http_port: u16,
    pub target_port: u32,
}

impl Config {
    pub fn new() -> Result<Self, anyhow::Error> {
        let log_level: LevelFilter = LevelFilter::from_str(
            env::var("LOG_LEVEL")
                .unwrap_or("debug".to_string())
                .as_str(),
        )?;
        let http_port = env::var("HTTP_PORT").unwrap_or("8080".into()).parse()?;
        let target_port = env::var("TARGET_PORT").unwrap_or("80".into()).parse()?;

        let conf = Config {
            log_level,
            http_port,
            target_port,
        };

        Ok(conf)
    }

    pub fn parse_ipv4_nets() -> Result<ipnetwork::Ipv4Network, anyhow::Error> {
        let test = ipnetwork::Ipv4Network::new(Ipv4Addr::new(192, 168, 0, 0), 16)?;

        return Ok(test);
    }
}
