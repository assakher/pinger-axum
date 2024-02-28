mod config;
mod errors;
mod requester;
mod routers;
mod shutdown;
mod tracer;
use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr},
    path::PathBuf,
    time::Duration,
};

use axum::Router;
use config::Config;
use dotenvy::dotenv;
use metrics_exporter_prometheus::PrometheusBuilder;
use tokio::net::TcpListener;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::requester::looped_ping;

#[global_allocator]
static ALLOC: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc;

#[tokio::main]
pub async fn main() -> Result<(), anyhow::Error> {
    let config = Config::new()?;
    // console_subscriber::init();

    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), config.serving_port);
    // let console_layer = console_subscriber::spawn();
    tracing_subscriber::registry()
        // .with(console_layer)
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or(
                format!(
                    "pinger={0},tower_http={0},axum::rejection=trace",
                    config.log_level.as_str()
                )
                .into(),
            ),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
    // console_subscriber::init();
    let net = Config::parse_ipv4_nets()?;
    // let requester = Requester::new();
    let prometheus = PrometheusBuilder::new().install_recorder().expect("Failed to setup Prometheus exporter");

    let listener = TcpListener::bind(addr).await.unwrap();
    let app = Router::new()
        .with_state(config::State {
            config: config.clone(),
        })
        .nest("/system", routers::healthcheck::system_router())
        .nest("/", routers::prometheus::metrics_router(prometheus))
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http().make_span_with(tracer::CustomSpan::new()))
        .fallback(errors::handler_404);
    tracing::info!("Configured app...");
    let server = axum::serve(listener, app).with_graceful_shutdown(shutdown::shutdown_signal());
    tracing::info!("Listening on {addr}");
    tracing::info!("Began polling...");
    let polling = tokio::spawn(async move {
        looped_ping(Duration::from_secs(120), vec![net], config.target_port).await
    });
    server.await?;
    polling.abort();
    tracing::info!("Shutting down");
    Ok(())
}
