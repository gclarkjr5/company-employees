use super::super::super::common::Company;


#[test]
#[should_panic]
fn test_get_empty_employees() {
    let mut company = Company::new();
    
    Company::get_employees(&mut company, "all".to_string()).unwrap();

}

#[test]
fn test_get_all_employees() {
    let mut company = Company::new();

    let employees = vec![
        ("sales".to_string(), vec!["gary".to_string()]),
        ("finance".to_string(), vec!["aleks".to_string(), "aalesund".to_string()])
    ];

    for (k, v) in employees.iter() {
        company.employee_list.entry(k.to_string()).or_insert(v.to_vec());
    }

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

    let employees = vec![
        ("sales".to_string(), vec!["gary".to_string()]),
        ("finance".to_string(), vec!["aleks".to_string(), "aalesund".to_string()])
    ];

    for (k, v) in employees.iter() {
        company.employee_list.entry(k.to_string()).or_insert(v.to_vec());
    }

    let dept_employees = Company::get_employees(&mut company, "finance".to_string()).unwrap();

    assert_eq!(
        dept_employees.get_key_value(&"finance".to_string()),
        Some((&"finance".to_string(),&vec!["aalesund".to_string(), "aleks".to_string()]))
    )
}