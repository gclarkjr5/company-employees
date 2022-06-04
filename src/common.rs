use tokio::fs::File;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::str;


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

        // Arc::new(
        //     Mutex::new(
                Ok(company)
        //     )
        // )
    }

    // reads in the current Company data if it exists
    // if not it will create a new empty one
    pub async fn init() -> io::Result<Company> {

        let mut file = match File::open("company.json").await {
            Ok(f) => f,
            Err(_) => {
                println!("No storage for company. Creating a new one.");

                let new_company = Company::new().await?;

                let company = serde_json::to_vec(&new_company).unwrap();

                let mut new_file = File::create("company.json").await?;

                new_file.write_all(&company).await?;

                File::open("company.json").await?
            }
        };

        let mut contents = vec![];

        file.read_to_end(&mut contents).await?;

        let string_content = str::from_utf8(&contents).unwrap();
        let company: Company = serde_json::from_str(&string_content).unwrap();

        Ok(company)
    }

    pub async fn clear(&self) -> io::Result<()> {

        let new_company = Company::new().await?;

        let company = serde_json::to_vec(&new_company).unwrap();

        let mut new_file = File::create("company.json").await?;

        new_file.write_all(&company).await?;

        Ok(())
    }

    pub async fn save(&self) -> io::Result<()> {

        let mut file = File::create("company.json").await?;

        let company = serde_json::to_vec(&self).unwrap();

        file.write_all(&company).await?;

        println!("company saved");

        Ok(())

    }
}