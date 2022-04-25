use super::Company;

pub fn add_employee(company: &mut Company, name: String, dept: String) -> &mut Company {

    // if the dept exists
    match company.employee_list.contains_key(&dept) {

        // if the employee already exists for the dept
        true => match check_employee_dept_existence(company, &name, &dept) {
            true => {
                println!("{} already exists for {} in the company", &name, &dept);
                company
            },

            // add new employee to existing dept
            false => add_employee_existing_department(company, name, dept)
        },

        // add new dept-employee combo
        false => {
            company.employee_list.insert(dept, vec![name]);
            company
        }
    }
}

fn check_employee_dept_existence(company: &mut Company, name: &String, dept: &String) -> bool {

    let employees = get_dept_employees(company, dept);

    employees.contains(name)

}

fn get_dept_employees(company: &mut Company, dept: &String) -> Vec<String> {
    let dept_employees: Vec<_> = company.employee_list
        .iter()
        .filter_map(
            |(k,v)|
            if k == dept {
                Some(v.to_owned())
            } else {
                None
            }
        )
        .flatten()
        .collect();

    dept_employees
}

fn add_employee_existing_department(company: &mut Company, name: String, dept: String) -> &mut Company{
    let mut employees = get_dept_employees(company, &dept);

    employees.push(name);

    company.employee_list.entry(dept).or_insert(employees);

    company
}


use std::collections::HashMap;

#[cfg(test)]
mod tests {
    
    use super::*;

    #[test]
    fn test_add_employee() {

        let mut company = Company {
            employee_list: HashMap::new()
        };

        let company = add_employee(&mut company, "gary".to_string(), "sales".to_string());

        assert_eq!(
            company.employee_list.get_key_value(&"sales".to_string()),
            Some((&"sales".to_string(), &vec!["gary".to_string()]))
        )

    }

    #[test]
    fn test_add_duplicate_employee() {

        let mut company = Company {
            employee_list: HashMap::new()
        };

        let mut company = add_employee(&mut company, "gary".to_string(), "sales".to_string());
        let company = add_employee(&mut company, "gary".to_string(), "sales".to_string());

        assert_eq!(
            company.employee_list.get_key_value(&"sales".to_string()),
            Some((&"sales".to_string(), &vec!["gary".to_string()]))
        )

    }


    // #[test]
    // fn test_add_multiple_employees_same_department() {
    //     let mut company = Company {
    //         employee_list: HashMap::new()
    //     };

    //     company = add_employee(&mut company, "gary".to_string(), "sales".to_string());
    //     company = add_employee(&mut company, "aleks".to_string(), "sales".to_string());

    //     assert_eq!(
    //         company.employee_list.get_key_value(&"sales".to_string()),
    //         Some((&"sales".to_string(), &vec!["gary".to_string(), "aleks".to_string()]))
    //     )

    // }

    // #[test]
    // fn test_add_multiple_employees_different_departments() {
    //     let mut company = Company {
    //         employee_list: HashMap::new()
    //     };

    //     company = add_employee(&mut company, "gary".to_string(), "sales".to_string());
    //     company = add_employee(&mut company, "aleks".to_string(), "finance".to_string());

    //     assert_eq!(
    //         company.employee_list.get_key_value(&"sales".to_string()),
    //         Some((&"sales".to_string(), &vec!["gary".to_string()]))
    //     );

    //     assert_eq!(
    //         company.employee_list.get_key_value(&"finance".to_string()),
    //         Some((&"finance".to_string(), &vec!["aleks".to_string()]))
    //     )

    // }
}