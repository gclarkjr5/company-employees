use ::cli::server::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {

    Server::run().await?;

    Ok(())
}
