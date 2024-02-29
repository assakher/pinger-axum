use axum::{routing::get, Json, Router};
use serde::{Deserialize, Serialize};
use strum_macros::AsRefStr;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct HealtStatus {
    status: Status,
}
#[derive(Debug, AsRefStr, Serialize, Clone, Deserialize)]
pub enum Status {
    Ok,
    Error,
}

pub fn system_router() -> Router {
    Router::new().route("/healthz", get(healthcheck))
}

async fn healthcheck() -> Json<HealtStatus> {
    Json(HealtStatus { status: Status::Ok })
}
