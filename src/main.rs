use sqlx::PgPool;
use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let configuration = get_configuration().expect("A configuration file should be present");
    let db_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("A connection pool to Postgres should be instatiated");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    run(tokio::net::TcpListener::bind(address).await?, db_pool).await
}
