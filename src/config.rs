use std::{env, net::Ipv4Addr, path::PathBuf, str::FromStr};

use dotenvy::dotenv;
use ipnetwork::Ipv4Network;
use lazy_static::lazy_static;
use regex::Regex;
use tracing::log::LevelFilter;

lazy_static! {
    static ref ADDR_PATTERN: Regex =
        Regex::new(r"^(?P<addr>[\d.]+)/(?P<mask>\d+),?(?P<offset>[^,]*?),?(?P<limit>[^,]*?)$")
            .unwrap();
}

pub struct Address {
    pub addr: Ipv4Network,
    pub offset: u8,
    pub limit: u8,
}

impl Address {
    fn new(addr: &str, mask: &str, limit: &str, offset: &str) -> Result<Self, anyhow::Error> {
        let parsed_addr = Ipv4Addr::from_str(addr)?;
        let parsed_mask = mask.parse()?;
        let parsed_offset = offset.parse().unwrap_or(0);
        let parsed_limit = limit.parse().unwrap_or(255);
        return Ok(Address {
            addr: Ipv4Network::new(parsed_addr, parsed_mask)?,
            limit: parsed_limit,
            offset: parsed_offset,
        });
    }
}

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
    pub forget_metric_timeout: u32,
    pub max_concurrent_connections: usize,
}

impl Config {
    pub fn new() -> Result<Self, anyhow::Error> {
        dotenv().unwrap_or(PathBuf::new());

        let log_level: LevelFilter =
            LevelFilter::from_str(env::var("LOG_LEVEL").expect("LOG_LEVEL not set").as_str())
                .expect("LOG_LEVEL should be trace, debug, info or error");
        let http_port = env::var("SERVING_PORT")
            .expect("SERVING_PORT not set")
            .parse()
            .expect("SERVING_PORT is not a valid u16");
        let target_port = env::var("TARGET_PORT")
            .expect("TARGET_PORT not set")
            .parse()
            .expect("TARGET_PORT is not a valid u32");
        let connect_timeout = env::var("CONNECT_TIMEOUT")
            .expect("CONNECT_TIMEOUT not set")
            .parse()
            .expect("CONNECT_TIMEOUT is not a valid u32");
        let ping_period = env::var("PING_PERIOD")
            .expect("PING_PERIOD not set")
            .parse()
            .expect("PING_PERIOD is not a valid u32");
        let forget_metric_timeout = env::var("FORGET_METRIC_TIMEOUT")
            .expect("FORGET_METRIC_TIMEOUT not set")
            .parse()
            .expect("FORGET_METRIC_TIMEOUT is not a valid u32");
        let max_concurrent_connections = env::var("MAX_CONCURRENT_CONNECTIONS")
            .expect("MAX_CONCURRENT_CONNECTIONS not set")
            .parse()
            .expect("MAX_CONCURRENT_CONNECTIONS is not a valid u32");

        let conf = Config {
            log_level,
            serving_port: http_port,
            target_port,
            connect_timeout,
            ping_period,
            forget_metric_timeout,
            max_concurrent_connections,
        };

        Ok(conf)
    }

    pub fn parse_ipv4_nets() -> Result<Vec<Address>, anyhow::Error> {
        let addr_lines = env::var("NETWORKS").expect("NETWORKS not set");
        let mut processd_addreses = vec![];
        for line in addr_lines.split(";") {
            match ADDR_PATTERN.captures(line) {
                Some(caps) => {
                    let addr = &caps["addr"];
                    let mask = &caps["mask"];
                    let offset = caps.name("offset").map_or("0", |m| m.as_str());
                    let limit = caps.name("limit").map_or("255", |m| m.as_str());
                    println!("{addr}, {mask},{offset}, {limit}");
                    match Address::new(addr, mask, limit, offset) {
                        Ok(a) => processd_addreses.push(a),
                        Err(e) => {
                            tracing::error!("Error parsing line '{line}' to address: {e:?}")
                        }
                    }
                }
                None => {
                    tracing::error!("LINE is not matched: {line}; please use pattern x.x.x.x/y");
                }
            }
        }

        return Ok(processd_addreses);
    }
}
