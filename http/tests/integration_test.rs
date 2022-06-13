use tokio::fs;
use std::str;
use http::http;
use hyper::{Body, Method, Client, Request};

const COMPANY: &str = "../data/company.json";
const BASE_URL: &str = "http://127.0.0.1:3000";


// clearing the company should work and gives us a fresh start
#[tokio::test]
async fn test_clearing() {

    // spawn server
    tokio::spawn(async {
        http::run_server().await
    });

    let expected_response = "Company cleared.";

    let data = serde_json::json!({"employee_list":{}});
    let expected_data = serde_json::to_string(&data)
        .expect("error converting json to string");

    // run a clear on the company with a success assertion to ensure the server is up and running
    let outcome = tokio::spawn(async move {

        let res = return_resp(Method::POST, "/clear").await;

        let file = fs::read(COMPANY).await.expect("error reading file");
        let data = str::from_utf8(&file).expect("error converting data");

        assert_eq!(
            (
                expected_response,
                expected_data.as_str()
            ),
            (
                res.as_str(),
                data
            )
        )
    });

    outcome.await.unwrap();

}

// getting data should error due to no employees added yet
#[tokio::test]
async fn test_getting_data_early() {
    // spawn server
    tokio::spawn(async {
        http::run_server().await
    });
    
    let department = "sales";

    let url_ending = format!("?department={}", &department);

    let expected_response = "Missing field";

    let outcome = tokio::spawn(async move {
        let res = return_resp(Method::GET, "").await;

        // handle error here in main function?
        // let res2 = return_resp(Method::GET, &*url_ending).await;

        // println!("{}", res);

        assert_eq!(expected_response, res)
    });

    outcome.await.unwrap()
}

// missing arguments should fail
#[tokio::test]
async fn test_add_employee() {
    // spawn server
    tokio::spawn(async {
        http::run_server().await
    });

    let name = "gary";
    let department = "sales";

    let url_ending = format!("?name={name}&department={department}");

    let expected_responses = (
        "Missing field",
        format!("Added {} to the {} department.", &name, &department)
    );

    // let file = fs::read(COMPANY).await.expect("error reading file");
    // let data = str::from_utf8(&file).expect("error converting data");

    // add employee
    let outcome = tokio::spawn(async move {
        let res = (
            return_resp(Method::POST, "").await,
            return_resp(Method::POST, &*url_ending).await
        );

        assert_eq!(expected_responses, (res.0.as_str(), res.1))
    });


    outcome.await.unwrap();
}

// get with no client should fail for all and department
// #[test]
// fn test_no_clients_to_get() {

//     // spawn a server
//     let rt = Runtime::new().unwrap();

//     // // start server
//     rt.spawn(cli::run_server());

//     // wait for server to come up
//     std::thread::sleep(std::time::Duration::from_millis(50));

//     // run this command to ensure that the server is working, without the server up, this will cause failure
//     // additionally this is setting up with an empty file
//     Command::cargo_bin("client").unwrap().arg("clear").assert().success();

//     // let mut client = Command::cargo_bin("client").unwrap();

//     // no employees have been added so these should fail
//     Command::cargo_bin("client").unwrap().arg("get").arg("--all").assert().failure();
//     Command::cargo_bin("client").unwrap().arg("get").arg("--department").arg("sales").assert().failure();
// }

// // add should add employees
// #[test]
// fn test_add_employee() {
//     // spawn a server
//     let rt = Runtime::new().unwrap();

//     // // start server
//     rt.spawn(cli::run_server());

//     // wait for server to come up
//     std::thread::sleep(std::time::Duration::from_millis(50));

//     // run this command to ensure that the server is working, without the server up, this will cause failure
//     // additionally this is setting up with an empty file
//     Command::cargo_bin("client").unwrap().arg("clear").assert().success();

//     Command::cargo_bin("client").unwrap().arg("add").arg("gary").arg("sales").assert().success();

//     let left = serde_json::json!({"employee_list":{"sales":["gary"]}}).to_string();

//     let contents = fs::read(COMPANY).expect("error reading file");
//     let right = str::from_utf8(&contents).expect("error deserializing data");

//     assert_eq!(left, right);
    
    

// }

// // duplicate add should fail on duplicate employee
// #[test]
// fn test_add_duplicate_employee() {
//     // spawn a server
//     let rt = Runtime::new().unwrap();

//     // // start server
//     rt.spawn(cli::run_server());

//     // wait for server to come up
//     std::thread::sleep(std::time::Duration::from_millis(50));

//     // run this command to ensure that the server is working, without the server up, this will cause failure
//     // additionally this is setting up with an empty file
//     Command::cargo_bin("client").unwrap().arg("clear").assert().success();

//     // try to add same employee + department twice
//     Command::cargo_bin("client").unwrap().arg("add").arg("gary").arg("sales").assert().success();
//     Command::cargo_bin("client").unwrap().arg("add").arg("gary").arg("sales").assert().failure();

//     let left = serde_json::json!({"employee_list":{"sales":["gary"]}}).to_string();

//     let contents = fs::read(COMPANY).expect("error reading file");
//     let right = str::from_utf8(&contents).expect("error deserializing data");

//     assert_eq!(left, right)

// }


// // get should work for department
// #[test]
// fn test_get_department() {
//     // spawn a server
//     let rt = Runtime::new().unwrap();

//     // // start server
//     rt.spawn(cli::run_server());

//     // wait for server to come up
//     std::thread::sleep(std::time::Duration::from_millis(50));

//     // run this command to ensure that the server is working, without the server up, this will cause failure
//     // additionally this is setting up with an empty file
//     Command::cargo_bin("client").unwrap().arg("clear").assert().success();

//     // add some employees
//     Command::cargo_bin("client").unwrap().arg("add").arg("gary").arg("sales").assert().success();
//     Command::cargo_bin("client").unwrap().arg("add").arg("gary").arg("finance").assert().success();
//     Command::cargo_bin("client").unwrap().arg("add").arg("aleks").arg("sales").assert().success();

//     // get employee from the department should work
//     Command::cargo_bin("client").unwrap().arg("get").arg("--department").arg("sales").assert().success();

// }

// // get should work for all
// #[test]
// fn test_get_all() {
//     // spawn a server
//     let rt = Runtime::new().unwrap();

//     // // start server
//     rt.spawn(cli::run_server());

//     // wait for server to come up
//     std::thread::sleep(std::time::Duration::from_millis(50));

//     // run this command to ensure that the server is working, without the server up, this will cause failure
//     // additionally this is setting up with an empty file
//     Command::cargo_bin("client").unwrap().arg("clear").assert().success();

//     // add some employees
//     Command::cargo_bin("client").unwrap().arg("add").arg("gary").arg("sales").assert().success();
//     Command::cargo_bin("client").unwrap().arg("add").arg("gary").arg("finance").assert().success();
//     Command::cargo_bin("client").unwrap().arg("add").arg("aleks").arg("sales").assert().success();

//     // get employee from all departments should work
//     Command::cargo_bin("client").unwrap().arg("get").arg("--all").assert().success();

// }

// // clear should empty
// #[test]
// fn test_clear_company() {
//     // spawn a server
//     let rt = Runtime::new().unwrap();

//     // // start server
//     rt.spawn(cli::run_server());

//     // wait for server to come up
//     std::thread::sleep(std::time::Duration::from_millis(50));

//     // run this command to ensure that the server is working, without the server up, this will cause failure
//     // additionally this is setting up with an empty file
//     Command::cargo_bin("client").unwrap().arg("clear").assert().success();

//     // add some employees and make sure the file contents contains them
//     Command::cargo_bin("client").unwrap().arg("add").arg("gary").arg("sales").assert().success();
//     Command::cargo_bin("client").unwrap().arg("add").arg("aleks").arg("sales").assert().success();

//     // lets assert that employees exist now before removing them
//     let left = serde_json::json!({"employee_list":{"sales":["gary", "aleks"]}}).to_string();
//     let contents = fs::read(COMPANY).expect("error reading file");
//     let right = str::from_utf8(&contents).expect("error deserializing data");

//     assert_eq!(left, right);

//     // now clear out file and check that its empty
//     Command::cargo_bin("client").unwrap().arg("clear").assert();

//     let left = serde_json::json!({"employee_list":{}}).to_string();

//     let contents = fs::read(COMPANY).expect("error reading file");
//     let right = str::from_utf8(&contents).expect("error deserializing data");

//     assert_eq!(left, right);

// }


async fn return_resp(method: Method, url_ending: &str) -> String {
    let client = Client::new();

    let req = Request::builder()
        .method(method)
        .uri(format!("{BASE_URL}/company{url_ending}"))
        .body(Body::empty())
        .expect("error sending request sent");

    let resp = client.request(req).await.expect("error getting response");

    let resp_bytes = hyper::body::to_bytes(resp.into_body()).await.unwrap();

    let resp_str = str::from_utf8(&*resp_bytes).expect("error converting bytes to str");

    resp_str.to_string()
}