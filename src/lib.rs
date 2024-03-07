pub mod document;
pub mod errors;
pub mod fs_ops;
pub mod rich_text;
use axum::{
    extract::DefaultBodyLimit,
    routing::{get, post},
    Router,
};
use http::{header::CONTENT_TYPE, Method};

use tower_http::{
    cors::{Any, CorsLayer},
    trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer},
};
use tracing::Level;

const USER_COOKIE_NAME: &str = "user_token";
const COOKIE_MAX_AGE: &str = "9999999";

#[derive(Clone)]
pub struct AppState {
    data_directory: String,
}

impl AppState {
    pub fn new(data_directory: String) -> Self {
        Self { data_directory }
    }

    pub async fn router(self) -> errors::Result<axum::Router> {
        let http_trace_layer = TraceLayer::new_for_http()
            .make_span_with(
                DefaultMakeSpan::new()
                    .level(Level::INFO)
                    .level(Level::DEBUG),
            )
            .on_response(
                DefaultOnResponse::new()
                    .level(Level::INFO)
                    .level(Level::DEBUG),
            );

        let cors_layer = CorsLayer::new()
            // allow `GET` and `POST` when accessing the resource
            .allow_methods([Method::GET, Method::POST])
            // allow requests from any origin
            .allow_origin(Any)
            .allow_headers([CONTENT_TYPE]);

        let router = Router::new().with_state(self);

        let api = Router::new().nest("/:version/api", router);
        Ok(api)
    }
}
