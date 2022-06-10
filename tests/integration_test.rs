mod common;
use tokio::fs;
use company_employees::common::Company;
// use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use assert_cmd::Command;
use serde::{Serialize, Deserialize};
use std::str;


// missing arguments should fail
#[tokio::test]
async fn test_missing_arguments() {
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
#[tokio::test]
async fn test_no_clients_to_get() {

    Command::cargo_bin("client").unwrap().arg("clear").assert();

    // let mut client = Command::cargo_bin("client").unwrap();

    // no employees have been added so these should fail
    Command::cargo_bin("client").unwrap().arg("get").arg("--all").assert().failure();
    Command::cargo_bin("client").unwrap().arg("get").arg("--department").arg("sales").assert().failure();
}

// add should add employees
#[tokio::test]
async fn test_add_employee() {
    
    Command::cargo_bin("client").unwrap().arg("clear").assert();

    // let mut client = Command::cargo_bin("client").unwrap();

    Command::cargo_bin("client").unwrap().arg("add").arg("gary").arg("sales").assert().success();

    let left = serde_json::json!({"employee_list":{"sales":["gary"]}}).to_string();

    let contents = fs::read("company.json").await.expect("error reading file");
    let right = str::from_utf8(&contents).expect("error deserializing data");

    assert_eq!(left, right)

}

// duplicate add should fail on duplicate employee
#[tokio::test]
async fn test_add_duplicate_employee() {
    
    Command::cargo_bin("client").unwrap().arg("clear").assert();

    Command::cargo_bin("client").unwrap().arg("add").arg("gary").arg("sales").assert().success();
    Command::cargo_bin("client").unwrap().arg("add").arg("gary").arg("sales").assert().failure();

    let left = serde_json::json!({"employee_list":{"sales":["gary"]}}).to_string();

    let contents = fs::read("company.json").await.expect("error reading file");
    let right = str::from_utf8(&contents).expect("error deserializing data");

    assert_eq!(left, right)

}


// get should work for department
#[tokio::test]
async fn test_get_department() {
    
    Command::cargo_bin("client").unwrap().arg("clear").assert();

    // add some employees
    Command::cargo_bin("client").unwrap().arg("add").arg("gary").arg("sales").assert().success();
    Command::cargo_bin("client").unwrap().arg("add").arg("gary").arg("finance").assert().success();
    Command::cargo_bin("client").unwrap().arg("add").arg("aleks").arg("sales").assert().success();

    // get employee from the department should work
    Command::cargo_bin("client").unwrap().arg("get").arg("--department").arg("sales").assert().success();

}

// get should work for all
#[tokio::test]
async fn test_get_all() {
    
    Command::cargo_bin("client").unwrap().arg("clear").assert();

    // add some employees
    Command::cargo_bin("client").unwrap().arg("add").arg("gary").arg("sales").assert().success();
    Command::cargo_bin("client").unwrap().arg("add").arg("gary").arg("finance").assert().success();
    Command::cargo_bin("client").unwrap().arg("add").arg("aleks").arg("sales").assert().success();

    // get employee from all departments should work
    Command::cargo_bin("client").unwrap().arg("get").arg("--all").assert().success();

}

// clear should empty
#[tokio::test]
async fn test_clear_company() {
    
    Command::cargo_bin("client").unwrap().arg("clear").assert();

    // add some employees and make sure the file contents contains them
    Command::cargo_bin("client").unwrap().arg("add").arg("gary").arg("sales").assert().success();
    Command::cargo_bin("client").unwrap().arg("add").arg("aleks").arg("sales").assert().success();

    let left = serde_json::json!({"employee_list":{"sales":["gary", "aleks"]}}).to_string();

    let contents = fs::read("company.json").await.expect("error reading file");
    let right = str::from_utf8(&contents).expect("error deserializing data");

    assert_eq!(left, right);

    // now clear out file and check that its empty
    Command::cargo_bin("client").unwrap().arg("clear").assert();

    let left = serde_json::json!({"employee_list":{}}).to_string();

    let contents = fs::read("company.json").await.expect("error reading file");
    let right = str::from_utf8(&contents).expect("error deserializing data");

    assert_eq!(left, right);

}
