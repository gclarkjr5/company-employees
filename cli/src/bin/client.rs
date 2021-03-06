use tokio::io::{self, AsyncWriteExt, AsyncReadExt};
use tokio::net::TcpStream;
use clap::{CommandFactory, Parser, ErrorKind};
use cli::cli::*;
use std::str;

type ErrorGen = Box<dyn std::error::Error + Send + Sync>;

#[tokio::main]
async fn main() -> Result<(), ErrorGen> {

    // connect to server
    let stream = TcpStream::connect("127.0.0.1:6379").await?;

    // split server into reader & writer
    let (mut rd, mut wr) = stream.into_split();

    // Write data in the background
    let writer = tokio::spawn(async move {
        // parse the cli args
        let cli = Cli::parse();

        // serialize the command struct and write it to the server
        let encoded = serde_json::to_vec(&cli.command)?;
        wr.write_all(&encoded).await?;

        Ok::<_, io::Error>(())
    });

    let reader = tokio::spawn(async move {

        let mut buffer = vec![];

        rd.read_to_end(&mut buffer).await?;

        match str::from_utf8(&mut buffer[0..5]) {
            Ok("Error") => {
                let mut cmd = Cli::command();
                    cmd.error(
                        ErrorKind::ValueValidation,
                        str::from_utf8(&mut buffer).expect("error converting buffer"),
                    )
                    .exit();
            },
            Ok(_) => println!("{}", str::from_utf8(&mut buffer).expect("error converting buffer")),
            Err(_) => eprintln!("error converting buffer")
        }

        Ok::<_, io::Error>(())
    });

    reader.await?.unwrap();
    writer.await?.unwrap();

    Ok(())
}