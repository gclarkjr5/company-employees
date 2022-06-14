use http::http;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {

    http::run_server().await?;

    Ok(())
}
