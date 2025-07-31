use axum::Router;
use axum::routing::get;
use clap::Parser;

mod arg_parse;
mod database;

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
    log::info!("Expecting database at '{}'", args.influxdb_url);

    let client = database::Client::new(
        &args.influxdb_database_name,
        &args.influxdb_url,
        &args.influxdb_auth_token_file,
    )?;

    let test_query = client
        .query(format!("SELECT * from {}", args.influxdb_database_name).as_str())
        .await;
    dbg!(&test_query);

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", args.port).as_str()).await?;

    let app = Router::new().route("/", get(root_handler));

    axum::serve(listener, app).await?;

    Ok(())
}
