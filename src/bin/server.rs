use tokio::io;


use company_employees::common;

#[tokio::main]
async fn main() -> io::Result<()> {

    common::run_server().await?;

    Ok(())
}
 

    
    