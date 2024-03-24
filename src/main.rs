use secrecy::ExposeSecret;
use sqlx::PgPool;
use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;
use zero2prod::telemetry::{get_subscriber, init_subscriber};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let configuration = get_configuration().expect("A configuration file should be present");
    let db_pool = PgPool::connect_lazy(configuration.database.connection_string().expose_secret())
        .expect("A connection pool to Postgres should be instatiated");
    let address = format!(
        "{}:{}",
        configuration.application.host, configuration.application.port
    );
    run(tokio::net::TcpListener::bind(address).await?, db_pool).await
}
