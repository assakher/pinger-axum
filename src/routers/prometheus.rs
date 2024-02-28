use std::future::ready;

use axum::{routing::get, Json, Router};
use metrics_exporter_prometheus::PrometheusHandle;
use serde::{Deserialize, Serialize};
use strum_macros::AsRefStr;

use crate::errors::AppError;

pub fn metrics_router(prometheus_recorder: PrometheusHandle) -> Router {
    Router::new().route("/metrics", get(move || ready(prometheus_recorder.render())))
}
