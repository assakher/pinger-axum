use std::time::Duration;

use axum::Router;
use metrics_exporter_prometheus::{PrometheusBuilder, PrometheusHandle};
use metrics_util::MetricKindMask;
use tower_http::{cors::CorsLayer, trace::TraceLayer};

use crate::{
    config::{self, Config},
    errors,
    routers::{self},
    tracer,
};

pub fn get_prometheus(config: &Config) -> PrometheusHandle {
    let prometheus = PrometheusBuilder::new()
        .idle_timeout(
            MetricKindMask::ALL,
            Some(Duration::from_secs(config.forget_metric_timeout as u64)),
        )
        .install_recorder()
        .expect("Failed to setup Prometheus exporter");
    prometheus
}

pub fn get_axum_app(config: &Config, prometheus: PrometheusHandle) -> Router {
    let app = Router::new()
        .with_state(config::State {
            config: config.clone(),
        })
        .nest("/system", routers::healthcheck::system_router())
        .nest("/", routers::prometheus::metrics_router(prometheus))
        .layer(CorsLayer::permissive())
        // TODO: add info span on response
        .layer(TraceLayer::new_for_http().make_span_with(tracer::CustomSpan::new()))
        .fallback(errors::handler_404);
    app
}
