use common::common::Company;

use tokio::net::TcpListener;
use std::sync::Arc;
use tokio::sync::Mutex;

mod run;
mod process_command;

type Db = Arc<Mutex<Company>>;
type ErrorGen = Box<dyn std::error::Error + Send + Sync>;


#[derive(Debug)]
pub struct Server {
    listener: TcpListener,
    db: Db
}

impl Server {
    pub async fn init() -> Result<Server, ErrorGen> {

        let host: String =  "127.0.0.1".to_string();
        let port: u16 =  6379;

        let bind_address: String = host + ":" + &port.to_string();
    
        let company = match Company::init().await {
            Ok(c) => c,
            Err(e) => {
                eprintln!("Error: {e}");
                return Err(e)
            }
        };

        let server: Server = Server {
            listener: TcpListener::bind(&bind_address).await?,
            db: Arc::new(Mutex::new(company))
        };

        Ok(server)
    }
}