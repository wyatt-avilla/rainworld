use axum::Router;
use axum::routing::get;
use clap::Parser;

mod arg_parse;

async fn root_handler() -> &'static str {
    "Hello world"
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = arg_parse::Args::parse();

    tracing_subscriber::fmt()
        .with_max_level(args.log_level)
        .init();

    log::info!("Running server on port {}", args.port);
    log::info!("Expecting ESP32 at '{}'", args.esp32_url);
    log::info!("Expecting database at '{}'", args.influx_db_url);

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", args.port).as_str()).await?;

    let app = Router::new().route("/", get(root_handler));

    axum::serve(listener, app).await?;

    Ok(())
}
