use axum::Router;
use axum::routing::get;
use clap::Parser;

/// Backend for rainworld
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Port to run the server on
    #[arg(short, long, default_value_t = shared::BACKEND_SERVER_PORT)]
    port: u16,

    /// Number of workers for the server
    #[arg(short, long, default_value_t = 1)]
    workers: usize,

    /// Log level, one of (INFO, WARN, ERROR, DEBUG, TRACE)
    #[arg(short, long, default_value_t = tracing::Level::INFO)]
    log_level: tracing::Level,
}

async fn root_handler() -> &'static str {
    "Hello world"
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    tracing_subscriber::fmt()
        .with_max_level(args.log_level)
        .init();

    log::info!(
        "Running server on port {} with {} worker{}",
        args.port,
        args.workers,
        if args.workers > 1 { "s" } else { "" }
    );

    let app = Router::new().route("/", get(root_handler));
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", args.port).as_str())
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
