pub mod document;
pub mod errors;
pub mod fs_ops;
pub mod rich_text;
use std::path::Path;

use axum::{
    extract::DefaultBodyLimit,
    routing::{get, post},
    Router,
};

use document::Document;
use http::{header::CONTENT_TYPE, Method};

use tower::ServiceBuilder;
use tower_http::{
    cors::{Any, CorsLayer},
    trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer},
};
use tracing::Level;

use axum::{
    body::Bytes,
    extract::MatchedPath,
    http::{HeaderMap, Request},
    response::{Html, Response},
};
use std::time::Duration;
use tokio::net::TcpListener;
use tower_http::classify::ServerErrorsFailureClass;
use tracing::{info_span, Span};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

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
        let http_trace_layer = TraceLayer::new_for_http();

        let cors_layer = CorsLayer::new()
            // allow `GET` and `POST` when accessing the resource
            .allow_methods([Method::GET, Method::POST])
            // allow requests from any origin
            .allow_origin(Any)
            .allow_headers([CONTENT_TYPE]);

        let router = Router::new()
            .route("/editor", post(rich_text::post::upsert_props))
            .layer(cors_layer)
            .layer(http_trace_layer)
            .with_state(self);

        Ok(router)
    }
}
