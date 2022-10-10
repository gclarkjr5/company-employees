use ::http::server::HyperServer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {

    HyperServer::run().await?;

    Ok(())
}
