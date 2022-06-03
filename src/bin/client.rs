use tokio::io::{self, AsyncWriteExt, AsyncReadExt};
use tokio::net::TcpStream;
use serde::{Deserialize, Serialize};
use clap::{Parser, Subcommand, CommandFactory, ErrorKind};
use std::str;
// use tokio::sync::mpsc;

// use company_employees::common::Company;

/// An application to add employees to a company and also see who exists
#[derive(Parser)]
#[clap(author, about, long_about = None)]
struct Cli {
    /// The operation to perform on the company
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug, Serialize, Deserialize)]
pub enum Commands {
    /// add an employee & department combo to the company if it doesn't already exist
    Add {
        
        /// Name of the employee to add
        name: String,

        /// Department that the employee works in
        department: String
    },

    /// get the employees of a department
    // #[clap(arg_enum)]
    Get {

        /// return all employees from the company by department
        #[clap(long)]
        all: bool,

        /// get the employees of a specific department
        #[clap(short, long)]
        department: Option<String>
    },

    /// clear the entire company
    Clear,
}

#[tokio::main]
async fn main() -> io::Result<()> {

    // connect to server
    let stream = TcpStream::connect("127.0.0.1:6379").await.unwrap();

    // split server into reader & writer
    let (mut rd, mut wr) = stream.into_split();

    // Write data in the background
    let writer = tokio::spawn(async move {
        // parse the cli args
        let cli = Cli::parse();

        // serialize the command struct and write it to the server
        let encoded = serde_json::to_vec(&cli.command).unwrap();
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
                        str::from_utf8(&mut buffer).unwrap(),
                    )
                    .exit();
            },
            Ok(_) => println!("{}", str::from_utf8(&mut buffer).unwrap()),
            Err(_) => eprintln!("error converting buffer")
        }

        Ok::<_, io::Error>(())
    });

    reader.await?.unwrap();
    writer.await?.unwrap();

    Ok(())
}