use std::collections::HashMap;

#[derive(Debug)]
pub struct Company {
    pub employee_list: HashMap<String, Vec<String>>
}

impl Company {
    pub fn new() -> Company {
            Company {
                employee_list: HashMap::new()
            }
    }
}

pub fn read_input(message: &'static str) -> String {

    println!("{}", message);

    let mut input = String::new();

    if let Err(e) = std::io::stdin().read_line(&mut input) {
        println!("{}", e)
    }
        
    input.trim().to_lowercase()

}

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