use std::collections::HashMap;


/// A company with a list of departments and the employees that work in those
/// departments.
#[derive(Debug)]
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
}

/// asks a question to the user at the CLI and ingests the response
pub fn read_input(message: &'static str) -> String {

    println!("{}", message);

    let mut input = String::new();

    if let Err(e) = std::io::stdin().read_line(&mut input) {
        println!("{}", e)
    }
        
    input.trim().to_lowercase()

}

/// make sure that the input from the user is one of "add", "get", or "end"
pub fn parsed_action(action: &String) -> Result<&String, &'static str> {

    let action_options = vec![
        "add",
        "get",
        "end"
    ];

    match action_options.contains(&action.as_str()) {
        true => Ok(action),
        false => Err("The options must be 'add', 'get' or 'end'. Please try again.")
    }

}