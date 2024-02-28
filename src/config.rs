use std::{env, net::Ipv4Addr, path::PathBuf, str::FromStr};

use dotenvy::dotenv;
use tracing::log::LevelFilter;

#[derive(Clone)]
pub struct State {
    pub config: Config,
}

#[derive(Clone)]
pub struct Config {
    pub log_level: LevelFilter,
    pub serving_port: u16,
    pub target_port: u32,
    pub connect_timeout: u32,
    pub ping_period: u32,
}

impl Config {
    pub fn new() -> Result<Self, anyhow::Error> {
        dotenv().unwrap_or(PathBuf::new());

        // TODO: replace ? with expect
        let log_level: LevelFilter =
            LevelFilter::from_str(env::var("LOG_LEVEL").expect("LOG_LEVEL not set").as_str())?;
        let http_port = env::var("SERVING_PORT")
            .expect("SERVING_PORT not set")
            .parse()?;
        let target_port = env::var("TARGET_PORT")
            .expect("TARGET_PORT not set")
            .parse()?;
        let connect_timeout = env::var("CONNECT_TIMEOUT")
            .expect("CONNECT_TIMEOUT not set")
            .parse()?;
        let ping_period = env::var("PING_PERIOD")
            .expect("PING_PERIOD not set")
            .parse()?;

        let conf = Config {
            log_level,
            serving_port: http_port,
            target_port,
            connect_timeout,
            ping_period,
        };

        Ok(conf)
    }

    // TODO: proper nets parsing
    pub fn parse_ipv4_nets() -> Result<ipnetwork::Ipv4Network, anyhow::Error> {
        let test = ipnetwork::Ipv4Network::new(Ipv4Addr::new(192, 168, 0, 0), 16)?;

        return Ok(test);
    }
}
