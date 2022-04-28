use company_employees::common::Company;

pub fn setup(company: &mut Company) -> &mut Company {

    let employees = vec![
        ("gary".to_string(), "sales".to_string()),
        ("aleks".to_string(), "finance".to_string()),
        ("aalesund".to_string(), "finance".to_string())
    ];

    for (k, v) in employees.iter() {
        Company::add_employee(company, &k, &v).unwrap();
    }

    company
}