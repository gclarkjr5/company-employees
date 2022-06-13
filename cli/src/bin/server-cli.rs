use tokio::io;
use cli::cli;

#[tokio::main]
async fn main() -> io::Result<()> {

    cli::run_server().await?;

    Ok(())
}
