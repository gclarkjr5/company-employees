use super::Company;
use super::super::employee_action;

pub fn get_employees(company: &mut Company, dept: String) {
    
    match dept.as_str() {
        "all" => get_all_employees(company),
        _ => get_employees_of_dept(&dept, company)
    }

}


fn get_all_employees(company: &mut Company) {

    // get the departments in a vector
    let mut depts: Vec<&String> = company.employee_list.keys().collect();

    depts.sort();

    for dept in depts {

        let mut dept_employees: Vec<_> = company.employee_list
            .iter()
            .filter_map(|(k,v)| if k == dept {Some(v)} else {None})
            .flatten()
            .collect();
        
        dept_employees.sort();

        println!("The employees in the {} department are: {:?}", dept, dept_employees)
        
    }
}

fn get_employees_of_dept(dept: &String, company: &mut Company) {

    let dept_check = department_check(dept, company);

    match dept_check {
        Ok(true) => {
            let mut dept_employees: Vec<&String> = company.employee_list
                .iter()
                .filter_map(|(k,v)| if k == dept {Some(v)} else {None})
                .flatten()
                .collect();

            dept_employees.sort();

            println!("The employees in the {} department are: {:?}", dept, &dept_employees)
        },
        Ok(false) => println!("Not possible"),
        Err(e) => println!("{:?}", e)
    }

    

}

fn department_check(dept: &String, company: &mut Company) -> Result<bool, &'static str> {

    match company.employee_list.contains_key(dept) {
        true => Ok(true),
        false => Err("This department does not exist!")
    }

}

// use std::collections::HashMap;

// #[cfg(test)]
// mod tests {
    
//     use super::*;

//     #[test]
//     fn test_get_all_employees() {

//         let mut company = Company {
//             employee_list: HashMap::new()
//         };

        

//         assert_eq!(
//             company.employee_list.get_key_value(&"sales".to_string()),
//             Some((&"sales".to_string(), &vec!["gary".to_string()]))
//         )

//     }

//     #[test]
//     fn test_get_dept_employees() {

//         let mut company = Company {
//             employee_list: HashMap::new()
//         };

//         add_employee(&mut company, "gary".to_string(), "sales".to_string());

//         assert_eq!(
//             company.employee_list.get_key_value(&"sales".to_string()),
//             Some((&"sales".to_string(), &vec!["gary".to_string()]))
//         )

//     }
// }