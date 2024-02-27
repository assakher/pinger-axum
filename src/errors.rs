use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct ErrorMsg {
    error: String
}

impl ErrorMsg {
    fn from_err_msg(msg: String) -> Self {
        ErrorMsg { error: msg }
    }
}
pub enum AppError {
    InternalError(anyhow::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let (status, error_message) = match self {
            AppError::InternalError(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()),
        };

        let body = Json(ErrorMsg::from_err_msg(error_message));

        (status, body).into_response()
    }
}

pub async fn handler_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, Json(ErrorMsg::from_err_msg("Requested url not found".into())))
}
