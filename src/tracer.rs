
use axum::http::Request;
use tower_http::trace::MakeSpan;
use tracing::{Level, Span};


use uuid;

// pub fn get_tracer(config: Config) -> () {
//     tracing_subscriber::registry()
//         .with(
//             tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or(
//                 format!(
//                     "axum_app={0},tower_http={0},axum::rejection=trace",
//                     config.log_level.as_str()
//                 )
//                 .into(),
//             ),
//         )
//         .with(tracing_subscriber::fmt::layer())
//         .init()
// }

#[derive(Debug, Clone)]
pub struct CustomSpan {
    level: Level,
}
impl CustomSpan {
    pub fn new() -> Self {
        CustomSpan { level: Level::INFO }
    }
}

impl<B> MakeSpan<B> for CustomSpan {
    fn make_span(&mut self, request: &Request<B>) -> Span {
        let request_id = uuid::Uuid::new_v4().simple().to_string();
        let _level = self.level.clone();
        tracing::span!(
            Level::INFO,
            "request",
            method = %request.method(),
            uri = %request.uri(),
            version = ?request.version(),
            headers = ?request.headers(),
            request_id
        )
    }
}
