use tokio::io;
use http::http;

#[tokio::main]
async fn main() -> io::Result<()> {

    http::run_server().await?;

    Ok(())
}
