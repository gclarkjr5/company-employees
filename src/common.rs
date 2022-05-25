use std::fs::File;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;



/// A company with a list of departments and the employees that work in those
/// departments.
#[derive(Debug, Serialize, Deserialize)]
pub struct Company {

    /// key-value pairs where keys are the departments of the company, and the value is a vector of employee
    /// names
    pub employee_list: HashMap<String, Vec<String>>
}


impl Company {

    /// instantiates a new company with no employees in it
    pub fn new() -> Company {

        Company {
            employee_list: HashMap::new()
        }
        
    }

    /// reads in the current Company data if it exists
    /// if not it will create a new empty one
    pub fn init() -> Company {

        let file = File::open("company.json");

        match file {
            Ok(data) => {

                let company: Company = serde_json::from_reader(data).unwrap();

                company

            },
            Err(_) => {
                let file = File::create("company.json").unwrap();

                let company = Company::new();

                serde_json::to_writer(file, &company).unwrap();

                company
            }
        }
    }

    pub fn clear(&self) {
        let file = File::create("company.json").unwrap();

        let fresh = Company::new();

        serde_json::to_writer(file, &fresh).unwrap();
    }

    pub fn save(&self) {

        let file = File::create("company.json").unwrap();

        serde_json::to_writer(file, self).unwrap();

    }
}