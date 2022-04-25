use std::collections::HashMap;

mod add_employee;
mod get_employee;

#[derive(Debug)]
pub struct Company {
    pub employee_list: HashMap<String, Vec<String>>
}


pub fn action(company: &mut Company, employee_action: String) -> &'static str {
    match employee_action.as_str() {
        "add" => {
            let name = read_input("What is the name of the employee to add?");
            let dept = read_input("What is the name of the department they are in?");

            // add employee
            add_employee::add_employee(company, name, dept);

            // return action
            "add"

        },
        "get" => {
            let i = read_input("Type the name of the department that you want. Type 'all' if you want all
        employees in the company.");

            // get all or department employees
            get_employee::get_employees(company, i);

            //return get action
            "get"
        },
        "end" => "end",
        _ => panic!("Shouldnt be a possible action")
    }
}



fn read_input(message: &'static str) -> String {

    println!("{}", message);

    let mut input = String::new();

    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    input.trim().to_lowercase().to_string()

}