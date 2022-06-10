use super::super::super::common::Company;


#[tokio::test]
#[should_panic]
async fn test_get_empty_employees() {
    let mut company = Company::new().await.unwrap();
    
    company.get_employees(&true, &None).await.unwrap();

}

#[tokio::test]
async fn test_get_all_employees() {
    let mut company = Company::new().await.unwrap();

    let employees = vec![
        ("sales".to_string(), vec!["gary".to_string()]),
        ("finance".to_string(), vec!["aleks".to_string(), "aalesund".to_string()])
    ];

    for (k, v) in employees.iter() {
        company.employee_list.entry(k.to_string()).or_insert(v.to_vec());
    }

    let all_employees = company.get_employees(&true, &None).await.unwrap();

    assert_eq!(
        (
            all_employees.employee_list.get_key_value(&"sales".to_string()),
            all_employees.employee_list.get_key_value(&"finance".to_string())
        ),
        (
            Some((&"sales".to_string(),&vec!["gary".to_string()])),
            Some((&"finance".to_string(),&vec!["aalesund".to_string(), "aleks".to_string()]))
        )
    )
}

#[tokio::test]
async fn test_get_dept_employees() {
    let mut company = Company::new().await.unwrap();

    let employees = vec![
        ("sales".to_string(), vec!["gary".to_string()]),
        ("finance".to_string(), vec!["aleks".to_string(), "aalesund".to_string()])
    ];

    for (k, v) in employees.iter() {
        company.employee_list.entry(k.to_string()).or_insert(v.to_vec());
    }

    let dept_employees = company.get_employees(&false, &Some("finance".to_string())).await.unwrap();

    assert_eq!(
        dept_employees.employee_list.get_key_value(&"finance".to_string()),
        Some((&"finance".to_string(),&vec!["aalesund".to_string(), "aleks".to_string()]))
    )
}