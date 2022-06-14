use cli::cli;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {

    cli::run_server().await?;

    Ok(())
}
