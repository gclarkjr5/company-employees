use clap::{self, Parser, Subcommand};
use serde::{Deserialize, Serialize};


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