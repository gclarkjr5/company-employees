#[cfg(test)]
mod tests {
    use super::super::*;


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