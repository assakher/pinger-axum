mod components;
mod config;
mod errors;
mod requester;
mod routers;
mod shutdown;
mod tracer;

use config::Config;
use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr},
    time::Duration,
};

use tokio::net::TcpListener;

use crate::{
    components::{get_axum_app, get_prometheus},
    requester::looped_ping,
    tracer::get_tracer,
};

#[global_allocator]
static ALLOC: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc;

#[tokio::main]
pub async fn main() -> Result<(), anyhow::Error> {
    let config = Config::new()?;

    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), config.serving_port);
    get_tracer(&config);
    let nets = Config::parse_ipv4_nets()?;
    let prometheus = get_prometheus(&config);
    tracing::info!("Configured app");
    let app = get_axum_app(&config, prometheus);
    let listener = TcpListener::bind(addr).await.unwrap();
    let server = axum::serve(listener, app).with_graceful_shutdown(shutdown::shutdown_signal());
    tracing::info!("Listening on {addr}");
    tracing::info!("Begin polling...");
    let polling = tokio::spawn(async move {
        looped_ping(
            Duration::from_secs(config.connect_timeout as u64),
            Duration::from_secs(config.ping_period as u64),
            nets,
            config.target_port,
        )
        .await
    });
    server.await?;
    polling.abort();
    tracing::info!("Shutting down");
    Ok(())
}
