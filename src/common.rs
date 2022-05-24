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

    pub fn format_employees(&self, employees: &HashMap<String, Vec<String>>) {
        let mut keys: Vec<_> = employees.keys().collect();
        keys.sort();

        // let e: HashMap<_, Vec<_>> = employees
        //     .iter()
        //     .map(|(_, val)| val.sort())
        //     .collect();

        // keys
        //     .iter()
        //     .for_each(|key| {
        //         let emps = employees.get(*key).unwrap().to_owned();
        //         println!("For the {} department the following employees exist: {}", key, emps.join(", "));
        //     });


        for key in keys {
            let mut emps = employees.get(key).unwrap().to_owned();
            emps.sort();
            println!("For the {} department the following employees exist: {}", key, emps.join(", "));
        }
    }

    pub fn save(&self) {

        let file = File::create("company.json").unwrap();

        serde_json::to_writer(file, self).unwrap();

    }
}