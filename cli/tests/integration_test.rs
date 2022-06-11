use std::fs;
use assert_cmd::Command;
use std::str;
use tokio::runtime::Runtime;
use cli::cli;

const COMPANY: &str = "../data/company.json";


// missing arguments should fail
#[test]
fn test_missing_arguments() {

    // spawn a server
    let rt = Runtime::new().unwrap();

    // // start server
    rt.spawn(cli::run_server());

    // wait for server to come up
    std::thread::sleep(std::time::Duration::from_millis(50));

    // run this command to ensure that the server is working, without the server up, this will cause failure
    Command::cargo_bin("client").unwrap().arg("clear").assert().success();

    // get cmds
    Command::cargo_bin("client").unwrap().assert().failure();

    // client without any commands
    Command::cargo_bin("client").unwrap().assert().failure();

    // get without subcommands
    Command::cargo_bin("client").unwrap().arg("get").assert().failure();

    // get with department subcommand but missing department specification
    Command::cargo_bin("client").unwrap().arg("get").arg("-d").assert().failure();

    // add cmds without subcommands
    Command::cargo_bin("client").unwrap().arg("add").assert().failure();

    // add name but missing department
    Command::cargo_bin("client").unwrap().arg("add").arg("name").assert().failure();

}

// get with no client should fail for all and department
#[test]
fn test_no_clients_to_get() {

    // spawn a server
    let rt = Runtime::new().unwrap();

    // // start server
    rt.spawn(cli::run_server());

    // wait for server to come up
    std::thread::sleep(std::time::Duration::from_millis(50));

    // run this command to ensure that the server is working, without the server up, this will cause failure
    // additionally this is setting up with an empty file
    Command::cargo_bin("client").unwrap().arg("clear").assert().success();

    // let mut client = Command::cargo_bin("client").unwrap();

    // no employees have been added so these should fail
    Command::cargo_bin("client").unwrap().arg("get").arg("--all").assert().failure();
    Command::cargo_bin("client").unwrap().arg("get").arg("--department").arg("sales").assert().failure();
}

// add should add employees
#[test]
fn test_add_employee() {
    // spawn a server
    let rt = Runtime::new().unwrap();

    // // start server
    rt.spawn(cli::run_server());

    // wait for server to come up
    std::thread::sleep(std::time::Duration::from_millis(50));

    // run this command to ensure that the server is working, without the server up, this will cause failure
    // additionally this is setting up with an empty file
    Command::cargo_bin("client").unwrap().arg("clear").assert().success();

    Command::cargo_bin("client").unwrap().arg("add").arg("gary").arg("sales").assert().success();

    let left = serde_json::json!({"employee_list":{"sales":["gary"]}}).to_string();

    let contents = fs::read(COMPANY).expect("error reading file");
    let right = str::from_utf8(&contents).expect("error deserializing data");

    assert_eq!(left, right);
    
    

}

// duplicate add should fail on duplicate employee
#[test]
fn test_add_duplicate_employee() {
    // spawn a server
    let rt = Runtime::new().unwrap();

    // // start server
    rt.spawn(cli::run_server());

    // wait for server to come up
    std::thread::sleep(std::time::Duration::from_millis(50));

    // run this command to ensure that the server is working, without the server up, this will cause failure
    // additionally this is setting up with an empty file
    Command::cargo_bin("client").unwrap().arg("clear").assert().success();

    // try to add same employee + department twice
    Command::cargo_bin("client").unwrap().arg("add").arg("gary").arg("sales").assert().success();
    Command::cargo_bin("client").unwrap().arg("add").arg("gary").arg("sales").assert().failure();

    let left = serde_json::json!({"employee_list":{"sales":["gary"]}}).to_string();

    let contents = fs::read(COMPANY).expect("error reading file");
    let right = str::from_utf8(&contents).expect("error deserializing data");

    assert_eq!(left, right)

}


// get should work for department
#[test]
fn test_get_department() {
    // spawn a server
    let rt = Runtime::new().unwrap();

    // // start server
    rt.spawn(cli::run_server());

    // wait for server to come up
    std::thread::sleep(std::time::Duration::from_millis(50));

    // run this command to ensure that the server is working, without the server up, this will cause failure
    // additionally this is setting up with an empty file
    Command::cargo_bin("client").unwrap().arg("clear").assert().success();

    // add some employees
    Command::cargo_bin("client").unwrap().arg("add").arg("gary").arg("sales").assert().success();
    Command::cargo_bin("client").unwrap().arg("add").arg("gary").arg("finance").assert().success();
    Command::cargo_bin("client").unwrap().arg("add").arg("aleks").arg("sales").assert().success();

    // get employee from the department should work
    Command::cargo_bin("client").unwrap().arg("get").arg("--department").arg("sales").assert().success();

}

// get should work for all
#[test]
fn test_get_all() {
    // spawn a server
    let rt = Runtime::new().unwrap();

    // // start server
    rt.spawn(cli::run_server());

    // wait for server to come up
    std::thread::sleep(std::time::Duration::from_millis(50));

    // run this command to ensure that the server is working, without the server up, this will cause failure
    // additionally this is setting up with an empty file
    Command::cargo_bin("client").unwrap().arg("clear").assert().success();

    // add some employees
    Command::cargo_bin("client").unwrap().arg("add").arg("gary").arg("sales").assert().success();
    Command::cargo_bin("client").unwrap().arg("add").arg("gary").arg("finance").assert().success();
    Command::cargo_bin("client").unwrap().arg("add").arg("aleks").arg("sales").assert().success();

    // get employee from all departments should work
    Command::cargo_bin("client").unwrap().arg("get").arg("--all").assert().success();

}

// clear should empty
#[test]
fn test_clear_company() {
    // spawn a server
    let rt = Runtime::new().unwrap();

    // // start server
    rt.spawn(cli::run_server());

    // wait for server to come up
    std::thread::sleep(std::time::Duration::from_millis(50));

    // run this command to ensure that the server is working, without the server up, this will cause failure
    // additionally this is setting up with an empty file
    Command::cargo_bin("client").unwrap().arg("clear").assert().success();

    // add some employees and make sure the file contents contains them
    Command::cargo_bin("client").unwrap().arg("add").arg("gary").arg("sales").assert().success();
    Command::cargo_bin("client").unwrap().arg("add").arg("aleks").arg("sales").assert().success();

    // lets assert that employees exist now before removing them
    let left = serde_json::json!({"employee_list":{"sales":["gary", "aleks"]}}).to_string();
    let contents = fs::read(COMPANY).expect("error reading file");
    let right = str::from_utf8(&contents).expect("error deserializing data");

    assert_eq!(left, right);

    // now clear out file and check that its empty
    Command::cargo_bin("client").unwrap().arg("clear").assert();

    let left = serde_json::json!({"employee_list":{}}).to_string();

    let contents = fs::read(COMPANY).expect("error reading file");
    let right = str::from_utf8(&contents).expect("error deserializing data");

    assert_eq!(left, right);

}
