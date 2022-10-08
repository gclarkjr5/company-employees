
use crate::cli::Commands;
use common::common::Company;
use super::Server;

use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::io::{Error, ErrorKind};

type Db = Arc<Mutex<Company>>;
type ErrorGen = Box<dyn std::error::Error + Send + Sync>;

impl Server {

    pub async fn process_command(command: Commands, db: Db) -> Result<String, ErrorGen> {

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
                        return Err(Box::new(msg))
                    }
                }
    
            },
            Commands::Get {all, department} => {
    
                if !*all && department.is_none() {
                    let msg = Error::new(ErrorKind::Other, "Either --all or --department must be defined for a GET command");
                    return Err(Box::new(msg))
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
                        return Err(Box::new(msg))
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
                        return Err(Box::new(msg))
                    },
                }
            },
        };
    }

}

