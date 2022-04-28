mod employees;
use employees::Company;


fn main() {
    let mut company = Company::new();

    let mut output: &'static str = "";

    while output != "end" {

        let input = employees::read_input("What would you like to do? Add/Get an employee/s or End operations [add/get/end]");

        if let Err(e) = employees::parsed_action(&input) {
            eprintln!("{}", e);
        }

        output = match input.as_str() {
            "add" => {
                let employee_name = employees::read_input("What is the name of the employee to add?");
                let employee_dept = employees::read_input("What is the department they are a part of?");


                if let Err(e) = Company::add_employee(&mut company, &employee_name, &employee_dept) {
                    println!("{}", e);
                }

                println!("{} has been added to the {} department.", employee_name, employee_dept);

                "add"
            },
            "get" => {
                let input = employees::read_input("Which department of employees do you want to get? Type 'all' if you want all employees in the company.");
                
                let employees = Company::get_employees(&mut company, input);

                match employees {
                    Ok(i) => {
                        let mut keys: Vec<_> = i.keys().collect();

                        keys.sort();

                        for key in keys {
                            let mut emps = i.get(key).unwrap().to_owned();

                            emps.sort();

                            println!("For the {} department the following employees exist: {}", key, emps.join(", "));

                        }
                    },
                    Err(e) => eprintln!("{}", e),
                }

                "get"
            },
            "end" => "end",
            _ => "not possible"
        };   
    }
}