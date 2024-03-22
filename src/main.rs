use zero2prod::startup::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    run(tokio::net::TcpListener::bind("127.0.0.1:3000").await?).await
}
