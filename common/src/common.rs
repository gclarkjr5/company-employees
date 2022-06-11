use std::collections::HashMap;
use std::str;
use tokio::fs;
use tokio::io::{self};
use serde::{Deserialize, Serialize};

const COMPANY: &str = "../data/company.json";

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
    pub async fn new() -> io::Result<Company> {
        let company = Company {employee_list: HashMap::new()};

        Ok(company)
    }

    // reads in the current Company data if it exists
    // if not it will create a new empty one
    pub async fn init() -> io::Result<Company> {

        let contents = match fs::read(COMPANY).await {
            Ok(f) => f,
            Err(_) => {
                println!("No storage for company. Creating a new one.");

                let new_company = Company::new().await?;

                let company = serde_json::to_vec(&new_company).unwrap();

                fs::write(COMPANY, &company).await?;

                fs::read(COMPANY).await?
            }
        };

        let string_content = str::from_utf8(&contents).unwrap();
        let company: Company = serde_json::from_str(&string_content).unwrap();

        Ok(company)
    }

    pub async fn clear(&mut self) -> io::Result<&mut Company> {

        self.employee_list.clear();

        let company = serde_json::to_vec(&self).unwrap();

        fs::write(COMPANY, &company).await?;

        Ok(self)
    }

    pub async fn save(&self) -> io::Result<&Company> {

        let company = serde_json::to_vec(&self).unwrap();

        fs::write(COMPANY, &company).await?;

        println!("company saved");

        Ok(self)

    }
}