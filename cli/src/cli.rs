use clap::{self, Parser, Subcommand};
use tokio::io::{self, AsyncReadExt, AsyncWriteExt, Error, ErrorKind};
use tokio::net::TcpListener;
use common::common::Company;
use std::sync::Arc;
use tokio::sync::Mutex;
use serde::{Deserialize, Serialize};
use std::str;

type Db = Arc<Mutex<Company>>;

/// An application to add employees to a company and also see who exists
#[derive(Parser)]
#[clap(author, about, long_about = None)]
pub struct Cli {
    /// The operation to perform on the company
    #[clap(subcommand)]
    pub command: Commands,
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

pub async fn run_server() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:6379").await?;

    let company = match Company::init().await {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error: {e}");
            return Err(e)
        }
    };

    let db = Arc::new(Mutex::new(company));

    loop {
        let (socket, _) = listener.accept().await?;

        let (mut rd, mut wr) = io::split(socket);

        let db = db.clone();

        tokio::spawn(async move {

            let mut buffer = vec![];

            rd.read_to_end(&mut buffer).await?;

            let command_str = str::from_utf8(&mut buffer).unwrap();

            let command: Commands = match serde_json::from_str(&command_str) {
                Ok(data) => data,
                Err(_) => {
                    eprintln!("Error at the CLI");
                    let msg = format!("Error at the CLI");
                    let error_string = Error::new(ErrorKind::Other, msg);
                    return Err(error_string)
                }
            };

            match process_command(command, db).await {
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

async fn process_command(command: Commands, db: Db) -> io::Result<String> {

    // get the subcommand from the cli and execute it
    match &command {
        Commands::Add {name, department} => {

            let mut company = db.lock().await;

            match company.add_employee(name, department).await {
                Ok(c) => {
                    println!("{:?}", c);
                    let msg = format!("{name} has been added to the {department} department.");
                    c.save().await?;
                    return Ok(msg)
                },
                Err(e) => {
                    let msg = Error::new(ErrorKind::Other, e);
                    return Err(msg)
                }
            }

        },
        Commands::Get {all, department} => {

            if !*all && department.is_none() {
                let msg = Error::new(ErrorKind::Other, "Either --all or --department must be defined for a GET command");
                return Err(msg)
            }

            let company = db.lock().await;
            
            match company.get_employees(all, department).await {
                Ok(c) => {
                    println!("{:?}", c);
                    let mut string_vec = vec![];
                    for (dept, employees) in c.employee_list.iter() {
                        let employees_string = employees.join(", ");
                        let string = format!("For the {dept} department the following employees exist: {employees_string}");
                        string_vec.push(string)
                    }

                    return Ok(string_vec.join("\n"))
                },
                Err(e) => {
                    let msg = Error::new(ErrorKind::Other, e);
                    return Err(msg)
                }
            }
        },
        Commands::Clear => {

            let mut company = db.lock().await;

            match company.clear().await {
                Ok(_) => {
                    return Ok("company cleared".to_string())
                },
                Err(e) => {
                    let msg = Error::new(ErrorKind::Other, e);
                    return Err(msg)
                },
            }
        },
    };
}