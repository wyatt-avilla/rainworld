use axum::Router;
use axum::routing::get;
use clap::Parser;

/// Backend for rainworld
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// IP address for accompanying ESP32
    #[arg(short, long)]
    esp32_ip: String,

    /// Port to run the server on
    #[arg(short, long, default_value_t = shared::BACKEND_SERVER_PORT)]
    port: u16,

    /// Log level, one of (INFO, WARN, ERROR, DEBUG, TRACE)
    #[arg(short, long, default_value_t = tracing::Level::INFO)]
    log_level: tracing::Level,
}

async fn root_handler() -> &'static str {
    "Hello world"
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    tracing_subscriber::fmt()
        .with_max_level(args.log_level)
        .init();

    log::info!(
        "Running server on port {}, expecting ESP32 at '{}'",
        args.port,
        args.esp32_ip
    );

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", args.port).as_str()).await?;

    let app = Router::new().route("/", get(root_handler));

    axum::serve(listener, app).await?;

    Ok(())
}
