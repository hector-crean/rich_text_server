use clap::Parser;
use dotenv::dotenv;
use rich_text_server::{errors, AppState};
use std::convert::From;
use std::env::Args;
use std::{
    env,
    net::SocketAddr,
    sync::{Arc, Mutex},
};
use tokio::sync::broadcast;
use tower_http::trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer};
use tracing::Level;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(next_line_help = true)]
struct Cli {
    #[arg(short, long)]
    data_dir: String,
    #[arg(short, long)]
    port: u16,
}

#[tokio::main]
async fn main() -> errors::Result<()> {
    dotenv().ok();

    let Cli { data_dir, port } = Cli::parse();

    let mut env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| {
        // axum logs rejections from built-in extractors with the `axum::rejection`
        // target, at `TRACE` level. `axum::rejection=trace` enables showing those events
        "example_tracing_aka_logging=debug,tower_http=debug,axum::rejection=trace,parelthon_server=debug,error,info".into()
    });

    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(env_filter)
        .with(tracing_subscriber::fmt::layer())
        .init();

    // 0.0.0.0: This IP address is a way to specify that the socket should bind to all available network interfaces on
    // the host machine. It's a common choice when you want your service to be reachable from outside networks.
    // let addr = SocketAddr::from(([0, 0, 0, 0], port));
    let addr = SocketAddr::from(([127, 0, 0, 1], port));

    let tcp_listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    tracing::debug!("listening on {}", addr);

    let router = AppState::new(data_dir).router().await?;

    axum::serve(tcp_listener, router).await?;

    Ok(())
}
