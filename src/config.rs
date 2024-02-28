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
    // pub prometheus_endpoint_uri: String,
    // pub prometheus_push_period: Duration,
    // pub prometheus_user: String,
    // pub prometheus_password: String,
    // pub subnets: Vec<>,
}

impl Config {
    pub fn new() -> Result<Self, anyhow::Error> {
        dotenv().unwrap_or(PathBuf::new());

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
        // let prometheus_endpoint_uri = env::var("PROMETHEUS_ENDPOINT_URI")
        //     .expect("PROMETHEUS_ENDPOINT_URI not set")
        //     .parse()?;
        // let prometheus_push_period = Duration::from_secs(
        //     env::var("PROMETHEUS_PUSH_PERIOD")
        //         .expect("PROMETHEUS_PUSH_PERIOD not set")
        //         .parse()
        //         .expect("PROMETHEUS_PUSH_PERIOD is not a valid u32"),
        // );
        // let prometheus_user = env::var("PROMETHEUS_USER")
        //     .expect("PROMETHEUS_USER not set")
        //     .parse()?;
        // let prometheus_password = env::var("PROMETHEUS_PASSWORD")
        //     .expect("PROMETHEUS_PASSWORD not set")
        //     .parse()?;

        let conf = Config {
            log_level,
            serving_port: http_port,
            target_port,
            connect_timeout,
            ping_period,
            // prometheus_endpoint_uri,
            // prometheus_push_period,
            // prometheus_user,
            // prometheus_password,
        };

        Ok(conf)
    }

    pub fn parse_ipv4_nets() -> Result<ipnetwork::Ipv4Network, anyhow::Error> {
        let test = ipnetwork::Ipv4Network::new(Ipv4Addr::new(192, 168, 0, 0), 16)?;

        return Ok(test);
    }
}
