mod common;
use company_employees::common::Company;
use tokio::fs::File;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};

#[tokio::test]
async fn server_initializes_empty_company_no_file_present() {
    // delete file to simulate starting without file
    File::remove_file("company.json").await.unwrap();

    // start server
    // common::setup().await.unwrap();


    
}

// #[tokio::test]
// async fn test_get_all_employees() {
//     // let mut company = Company::new().await.unwrap();
    
//     common::setup().await.unwrap();

//     let all_employees = company.get_employees(&true, &None).await.unwrap();

//     assert_eq!(
//         (
//             all_employees.employee_list.get_key_value(&"sales".to_string()),
//             all_employees.employee_list.get_key_value(&"finance".to_string())
//         ),
//         (
//             Some((&"sales".to_string(),&vec!["gary".to_string()])),
//             Some((&"finance".to_string(),&vec!["aalesund".to_string(), "aleks".to_string()]))
//         )
//     )
// }

// #[tokio::test]
// async fn test_get_dept_employees() {
//     let mut company = Company::new().await.unwrap();
    
//     common::setup(&mut company).await.unwrap();

//     let dept_employees = company.get_employees(&false, &Some("finance".to_string())).await.unwrap();

//     assert_eq!(
//         dept_employees.employee_list.get_key_value(&"finance".to_string()),
//         Some((&"finance".to_string(),&vec!["aalesund".to_string(), "aleks".to_string()]))
//     )
// }