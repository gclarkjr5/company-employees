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
                    println!("{name} has been added to the {department} department.")
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
            if !*all && department.is_none() {
                let mut cmd = Cli::command();
                    cmd.error(
                        ErrorKind::ValueValidation,
                        "Either --all or --department must be defined",
                    )
                    .exit();
            }
            
            match company.get_employees(all, department) {
                Ok(c) => {
                    for (dept, employees) in c.employee_list.iter() {
                        let employees_string = employees.join(", ");
                        println!("For the {dept} department the following employees exist: {employees_string}");
                    }
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
        Commands::Clear => {

            company.clear();

            println!("Cleared the company")
        },
    };  

}