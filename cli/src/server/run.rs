
use crate::cli::Commands;

use super::Server;

use tokio::io::{self, AsyncReadExt, AsyncWriteExt, Error, ErrorKind};
use std::str;

type ErrorGen = Box<dyn std::error::Error + Send + Sync>;


impl Server {

    pub async fn run() -> Result<(), ErrorGen> {
        let server: Server = Server::init().await?;
    
        loop {
            let (socket, _) = server.listener.accept().await?;
    
            let (mut rd, mut wr) = io::split(socket);
    
            let db = server.db.clone();
    
            tokio::spawn(async move {
    
                let mut buffer = vec![];
    
                rd.read_to_end(&mut buffer).await?;
    
                let command_str = match str::from_utf8(&mut buffer) {
                    Ok(v) => Ok(v),
                    Err(e) => Err(Error::new(ErrorKind::Other, e))
                };
    
                let command: Commands = match serde_json::from_str(&command_str?) {
                    Ok(data) => data,
                    Err(_) => {
                        eprintln!("Error at the CLI");
                        let msg = format!("Error at the CLI");
                        let error_string = Error::new(ErrorKind::Other, msg);
                        return Err(error_string)
                    }
                };
    
                match Server::process_command(command, db).await {
                    Ok(msg) => {
                        println!("{msg}");
                        let r: &[u8] = msg.as_bytes();
                        wr.write_all(r).await?;
                    },
                    Err(e) => {
                        eprintln!("Error: {e}");
                        let error_string = format!("Error: {}", e.to_string());
                        let r: &[u8] = error_string.as_bytes();
                        wr.write_all(r).await?;
                    },
                }
    
                Ok::<_, io::Error>(())
            });
            
        }
    }

}

