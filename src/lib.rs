// mod employee_action;
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

    pub fn add_employee(
        &mut self,
        employee_name: &String,
        employee_dept: &String
    ) -> Result<&mut Company, &'static str> {
        
        // check if employee exists
        let department_employees = self.employee_list.get_mut(employee_dept);
        

        match department_employees {
            Some(x) => match x.contains(&employee_name) {
                true => {
                    Err("The employee {} already exists for the {} department")
                },
                false => {
                    x.push(employee_name.to_owned());
                    x.sort();
                    Ok(self)
                }
            },
            None => {
                self.employee_list.insert(employee_dept.to_owned(), vec![employee_name.to_owned()]);
                Ok(self)
            }
        }
    }

    pub fn get_employees(
        &mut self,
        department: String
    ) -> Result<HashMap<String, Vec<String>>, &'static str> {

        if self.employee_list.is_empty() {
            return Err("No employees have been added to the company yet.")
        }


        match department.as_str() {
            "all" => Ok(
                self.employee_list.to_owned()
            ),
            _ => match self.employee_list.contains_key(&department) {
                true => Ok(
                    get_dept_employees(self, department)
                ),
                false => Err("Department doesnt exist")
            }
        }
    }
}

pub fn read_input(message: &'static str) -> String {

    println!("{}", message);

    let mut input = String::new();

    // std::io::stdin().read_line(&mut input);

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

fn get_dept_employees(
    company: &mut Company,
    department: String
) -> HashMap<String, Vec<String>> {
    
    let dept_employees: Vec<String> = company.employee_list
        .iter()
        .filter_map(|(k,v)| if *k == department {Some(v.to_owned())} else {None})
        .flatten()
        .collect();


    let vec_dept = vec![(department, dept_employees)];

    HashMap::from_iter(vec_dept)
}



#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_add_employee() {

        let mut company = Company::new();

        // let employees = vec![
        //     (&"gary".to_string(), &"sales".to_string())
        // ];

        Company::add_employee(&mut company, &"gary".to_string(), &"sales".to_string()).unwrap();

        assert_eq!(
            company.employee_list.get_key_value(&"sales".to_string()),
            Some((&"sales".to_string(), &vec!["gary".to_string()]))
        )

    }

    #[test]
    #[should_panic]
    fn test_add_duplicate_employee_to_same_department() {

        let mut company = Company::new();

        Company::add_employee(&mut company, &"gary".to_string(), &"sales".to_string()).unwrap();
        Company::add_employee(&mut company, &"gary".to_string(), &"sales".to_string()).unwrap();

    }


    #[test]
    fn test_add_multiple_employees_same_department() {
        let mut company = Company::new();

        Company::add_employee(&mut company, &"gary".to_string(), &"sales".to_string()).unwrap();
        Company::add_employee(&mut company, &"aleks".to_string(), &"sales".to_string()).unwrap();

        assert_eq!(
            company.employee_list.get_key_value(&"sales".to_string()),
            Some((&"sales".to_string(), &vec!["aleks".to_string(), "gary".to_string()]))
        )
    }

    #[test]
    fn test_add_multiple_employees_different_departments() {
        let mut company = Company::new();

        Company::add_employee(&mut company, &"gary".to_string(), &"sales".to_string()).unwrap();
        Company::add_employee(&mut company, &"aleks".to_string(), &"finance".to_string()).unwrap();
        Company::add_employee(&mut company, &"aalesund".to_string(), &"finance".to_string()).unwrap();

        assert_eq!(
            (
                company.employee_list.get_key_value(&"sales".to_string()),
                company.employee_list.get_key_value(&"finance".to_string())
            ),
            (
                Some((&"sales".to_string(),&vec!["gary".to_string()])),
                Some((&"finance".to_string(),&vec!["aalesund".to_string(), "aleks".to_string()]))
            )
        );

    }

    #[test]
    #[should_panic]
    fn test_get_empty_employees() {
        let mut company = Company::new();
        
        Company::get_employees(&mut company, "all".to_string()).unwrap();

    }

    #[test]
    fn test_get_all_employees() {
        let mut company = Company::new();

        Company::add_employee(&mut company, &"gary".to_string(), &"sales".to_string()).unwrap();
        Company::add_employee(&mut company, &"aleks".to_string(), &"finance".to_string()).unwrap();
        Company::add_employee(&mut company, &"aalesund".to_string(), &"finance".to_string()).unwrap();

        let all_employees = Company::get_employees(&mut company, "all".to_string()).unwrap();

        assert_eq!(
            (
                all_employees.get_key_value(&"sales".to_string()),
                all_employees.get_key_value(&"finance".to_string())
            ),
            (
                Some((&"sales".to_string(),&vec!["gary".to_string()])),
                Some((&"finance".to_string(),&vec!["aalesund".to_string(), "aleks".to_string()]))
            )
        )
    }

    #[test]
    fn test_get_dept_employees() {
        let mut company = Company::new();

        Company::add_employee(&mut company, &"gary".to_string(), &"sales".to_string()).unwrap();
        Company::add_employee(&mut company, &"aleks".to_string(), &"finance".to_string()).unwrap();
        Company::add_employee(&mut company, &"aalesund".to_string(), &"finance".to_string()).unwrap();

        let dept_employees = Company::get_employees(&mut company, "finance".to_string()).unwrap();

        assert_eq!(
            dept_employees.get_key_value(&"finance".to_string()),
            Some((&"finance".to_string(),&vec!["aalesund".to_string(), "aleks".to_string()]))
        )
    }
}


