use std::future::ready;

use axum::{routing::get, Router};
use metrics_exporter_prometheus::PrometheusHandle;

pub fn metrics_router(prometheus_recorder: PrometheusHandle) -> Router {
    Router::new().route("/metrics", get(move || ready(prometheus_recorder.render())))
}
