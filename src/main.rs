use sqlx::PgPool;
use tracing::subscriber::set_global_default;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::Registry;
use tracing_subscriber::{layer::SubscriberExt, EnvFilter};
use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    LogTracer::init().expect("Logger should be set");

    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    let formatting_layer = BunyanFormattingLayer::new("zero2prod".into(), std::io::stdout);
    let subscriber = Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer);
    set_global_default(subscriber).expect("A subscriber should be set");

    let configuration = get_configuration().expect("A configuration file should be present");
    let db_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("A connection pool to Postgres should be instatiated");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    run(tokio::net::TcpListener::bind(address).await?, db_pool).await
}
