use std::fs::File;
use clap::{Parser, Subcommand, CommandFactory, ErrorKind};

use company_employees::common::Company;

/// An application to add employees to a company and also see who exists
#[derive(Parser)]
#[clap(author, about, long_about = None)]
struct Cli {
    /// The operation to perform on the company
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
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

fn main() {

    // parse the cli args
    let cli = Cli::parse();

    // create company from existing file
    let mut company = Company::init();

    // get the subcommand from the cli and execute it
    match &cli.command {
        Commands::Add {name, department} => {

            match company.add_employee(name, department) {
                Ok(c) => {
                    c.save();
                    println!("{} has been added to the {} department.", name, department)
                },
                Err(e) => {
                    let mut cmd = Cli::command();
                    cmd.error(
                        ErrorKind::ValueValidation,
                        e,
                    )
                    .exit();
                }
            }
        },
        Commands::Get {all, department} => {

            if *all {

                match company.get_employees(&"all".to_string()) {
                    Ok(employees) => {
                        company.format_employees(&employees)
                    },
                    Err(e) => {
                        let mut cmd = Cli::command();
                        cmd.error(
                            ErrorKind::ValueValidation,
                            e,
                        )
                        .exit();
                    }
                }

            } else {
                if let Some(dept) = department {
                    match company.get_employees(dept) {
                        Ok(employees) => {
                            company.format_employees(&employees)
                        },
                        Err(e) => {
                            let mut cmd = Cli::command();
                            cmd.error(
                                ErrorKind::ValueValidation,
                                e,
                            )
                            .exit();
                        }
                    }
                } else {
                    let mut cmd = Cli::command();
                    cmd.error(
                        ErrorKind::EmptyValue,
                        "No value provided for the department which is required if not getting all.",
                    )
                    .exit();
                } 
            }
        },
        Commands::Clear => {

            let file = File::create("company.json").unwrap();

            let fresh = Company::new();

            serde_json::to_writer(file, &fresh).unwrap();

            println!("Cleared the company")
        },
    };  

}