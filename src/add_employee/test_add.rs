use super::super::super::common::Company;

#[test]
fn test_add_employee() {

    let mut company = Company::new();

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

    let employees = vec![
        ("gary".to_string(), "sales".to_string()),
        ("gary".to_string(), "sales".to_string())
    ];

    for (k, v) in employees.iter() {
        Company::add_employee(&mut company, &k, &v).unwrap();
    }

}


#[test]
fn test_add_multiple_employees_same_department() {
    let mut company = Company::new();

    let employees = vec![
        ("gary".to_string(), "sales".to_string()),
        ("aleks".to_string(), "sales".to_string())
    ];

    for (k, v) in employees.iter() {
        Company::add_employee(&mut company, &k, &v).unwrap();
    }

    assert_eq!(
        company.employee_list.get_key_value(&"sales".to_string()),
        Some((&"sales".to_string(), &vec!["gary".to_string(), "aleks".to_string()]))
    )
}

#[test]
fn test_add_multiple_employees_different_departments() {
    let mut company = Company::new();

    let employees = vec![
        ("gary".to_string(), "sales".to_string()),
        ("aleks".to_string(), "finance".to_string()),
        ("aalesund".to_string(), "finance".to_string())
    ];

    for (k, v) in employees.iter() {
        Company::add_employee(&mut company, &k, &v).unwrap();
    }

    assert_eq!(
        (
            company.employee_list.get_key_value(&"sales".to_string()),
            company.employee_list.get_key_value(&"finance".to_string())
        ),
        (
            Some((&"sales".to_string(),&vec!["gary".to_string()])),
            Some((&"finance".to_string(),&vec!["aleks".to_string(), "aalesund".to_string()]))
        )
    );

}