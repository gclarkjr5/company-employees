// use company_employees::common::Company;
use tokio::io;
use assert_cmd::Command;

pub fn setup() {

    Command::cargo_bin("server").unwrap();

    // Ok(())

    // let employees = vec![
    //     ("gary".to_string(), "sales".to_string()),
    //     ("aleks".to_string(), "finance".to_string()),
    //     ("aalesund".to_string(), "finance".to_string())
    // ];

    // for (k, v) in employees.iter() {
    //     company.add_employee(&k, &v).await.unwrap();
    // }

    // Ok(company)
}