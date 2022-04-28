mod common;
use company_employees::common::Company;

#[test]
fn test_get_all_employees() {
    let mut company = Company::new();
    
    common::setup(&mut company);

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
    
    common::setup(&mut company);

    let dept_employees = Company::get_employees(&mut company, "finance".to_string()).unwrap();

    assert_eq!(
        dept_employees.get_key_value(&"finance".to_string()),
        Some((&"finance".to_string(),&vec!["aalesund".to_string(), "aleks".to_string()]))
    )
}