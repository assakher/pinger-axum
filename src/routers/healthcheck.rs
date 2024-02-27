use axum::{routing::get, Json, Router};
use serde::{Deserialize, Serialize};
use strum_macros::AsRefStr;

use crate::errors::AppError;

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
    Router::new()
        .route("/healthz", get(healthcheck))
        .route("/fail", get(always_fail))
}

async fn healthcheck() -> Json<HealtStatus> {
    Json(HealtStatus { status: Status::Ok })
}
async fn always_fail() -> Result<Json<HealtStatus>, AppError> {
    match "abc".parse::<i8>() {
        Ok(_) => Ok(Json(HealtStatus { status: Status::Ok })),
        Err(error) => Err(AppError::InternalError(error.into())),
    }
}
