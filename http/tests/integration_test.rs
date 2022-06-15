use tokio::fs;
use std::str;
use http::http;
use hyper::{Body, Method, Client, Request};
use tokio::time::{sleep, Duration};

const COMPANY: &str = "../data/company.json";
const BASE_URL: &str = "http://127.0.0.1:3000";

// missing field scenarios should report such
#[tokio::test]
async fn test_missing_arguments() {
    // spawn server
    tokio::spawn(async {
        http::run_server().await
    });

    // wait for server to come up
    sleep(Duration::from_secs(2)).await;

    let name_ending = "?name=name_1";
    let dept_ending = "?department=dept_1";

    let expected_responses = vec!["Missing field"; 4];

    let methods = vec![Method::GET, Method::POST, Method::POST, Method::POST];
    let url_endings = vec!["", "", name_ending, dept_ending];

    let outcome = tokio::spawn(async move {

        let mut actual_responses = vec![];

        for i in 0..methods.len() {
            let res = return_resp(methods[i].to_owned(), url_endings[i]).await.unwrap();
            actual_responses.push(res)
        }

        println!("{:?}", actual_responses);

        assert_eq!(
            expected_responses,
            actual_responses
        )

    });

    outcome.await.unwrap()
    
}

// getting the data with no employees added yet should fail
#[tokio::test]
async fn test_get_no_employees() {
    // spawn server
    tokio::spawn(async {
        http::run_server().await
    });

    // wait for server to come up
    sleep(Duration::from_secs(2)).await;

    let url_endings = vec!["?department=all", "?department=dept_1"];
    let expected_responses = vec!["No employees have been added to the company yet."; 2];

    let outcome = tokio::spawn(async move {
        // first clear out the company just to ensure no employees are there
        // but make sure it worked
        clear_company_and_verify().await;

        // attempt to get employees from empty company
        let mut results = vec![];
        for url in url_endings {
            let res = return_resp(Method::GET, &*url).await.unwrap();
            results.push(res);
        }

        // should return info about no employees
        assert_eq!(
            expected_responses,
            results
        )

    });

    outcome.await.unwrap();

}

// test adding some employees
#[tokio::test]
async fn test_add_employees() {
    // spawn server
    tokio::spawn(async {
        http::run_server().await
    });

    // wait for server to come up
    sleep(Duration::from_secs(2)).await;

    let names_depts = vec![
        ("gary", "sales"),
        ("aleks", "finance")
    ];

    let url_endings = names_depts.iter()
        .map(|(n, d)| format!("?name={}&department={}", n, d))
        .collect::<Vec<String>>();

    let expected_responses = names_depts.iter()
        .map(|(n, d)| format!("Added {} to the {} department.", n, d))
        .collect::<Vec<String>>();
    
    let outcome = tokio::spawn(async move {
        // clear company and verify its empty
        clear_company_and_verify().await;

        // add employees
        let mut results = vec![];
        for url in url_endings {
            let res = return_resp(Method::POST, &*url).await.unwrap();
            results.push(res);
        }

        assert_eq!(expected_responses, results)
    });

    outcome.await.unwrap();
}

// fail on adding duplicate employee
#[tokio::test]
async fn test_duplicate_employee() {
    // spawn server
    tokio::spawn(async {
        http::run_server().await
    });

    // wait for server to come up
    sleep(Duration::from_secs(2)).await;

    let names_depts = vec![
        ("gary", "sales"),
        ("aleks", "finance"),
        ("gary", "sales")
    ];

    let url_endings = names_depts.iter()
        .map(|(n, d)| format!("?name={}&department={}", n, d))
        .collect::<Vec<String>>();

    // only the first 2 responses should be expected
    let mut expected_responses = names_depts[0..2].iter()
        .map(|(n, d)| format!("Added {} to the {} department.", n, d))
        .collect::<Vec<String>>();

    // add duplicate response to expected
    let duplicate_response = format!("The employee {} already exists for the {} department", names_depts[2].0, names_depts[2].1);
    expected_responses.push(duplicate_response);
    
    let outcome = tokio::spawn(async move {
        // clear out the company data
        clear_company_and_verify().await;

        // add employees
        let mut results = vec![];

        for url in url_endings {
            let res = return_resp(Method::POST, &*url).await.unwrap();
            results.push(res);
        }

        assert_eq!(expected_responses, results)
    });

    outcome.await.unwrap();
}


// gets after adding employee should return properly
#[tokio::test]
async fn test_get_departments() {
    // spawn server
    tokio::spawn(async {
        http::run_server().await
    });

    // wait for server to come up
    sleep(Duration::from_secs(2)).await;

    let names_depts = vec![
        ("gary", "sales"),
        ("aleks", "finance"),
        ("gary", "finance")
    ];

    let url_endings = names_depts.iter()
        .map(|(n, d)| format!("?name={}&department={}", n, d))
        .collect::<Vec<String>>();

    let expected_adds = names_depts.iter()
        .map(|(n, d)| format!("Added {} to the {} department.", n, d))
        .collect::<Vec<String>>();

    let dept_gets = vec!["sales", "finance", "all"];

    let get_urls = dept_gets.iter()
        .map(|url| format!("?department={}", url))
        .collect::<Vec<String>>();

    let expected_gets = vec![
        "For the sales department the following employees exist: gary",
        "For the finance department the following employees exist: aleks, gary",
        "For the finance department the following employees exist: aleks, gary\nFor the sales department the following employees exist: gary"
    ];

    let outcome = tokio::spawn(async move {
        // clear out company data
        clear_company_and_verify().await;
        
        let mut actual_adds = vec![];
        for url in url_endings {
            let res = return_resp(Method::POST, &*url).await.unwrap();
            actual_adds.push(res);
        }

        assert_eq!(expected_adds, actual_adds);

        let mut actual_gets = vec![];
        for get in get_urls {
            let res = return_resp(Method::GET, &*get).await.unwrap();
            actual_gets.push(res)
        }

        assert_eq!(expected_gets, actual_gets)
    });

    outcome.await.unwrap();
    
}


// clearing the company should work
#[tokio::test]
async fn test_clearing() {
    // spawn server
    tokio::spawn(async {
        http::run_server().await
    });

    // wait for server to come up
    sleep(Duration::from_secs(2)).await;

    // run a clear on the company with a success assertion to ensure the server is up and running
    let outcome = tokio::spawn(async move {
        clear_company_and_verify().await
    });

    outcome.await.unwrap();

}

async fn clear_company_and_verify() {
    let data = serde_json::json!({"employee_list":{}});
    let expected_data = serde_json::to_string(&data).expect("error converting json to string");

    return_resp(Method::POST, "/clear").await.unwrap();

    let file = fs::read(COMPANY).await.expect("error reading file");
    let data = str::from_utf8(&file).expect("error converting data");

    assert_eq!(expected_data, data)

}

async fn return_resp(method: Method, url_ending: &str) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    
    let client = Client::new();

    let req = Request::builder()
        .method(method)
        .uri(format!("{BASE_URL}/company{url_ending}"))
        .body(Body::empty())?;

    let resp = client.request(req).await?;

    let resp_bytes = hyper::body::to_bytes(resp.into_body()).await?;

    let resp_str = str::from_utf8(&*resp_bytes)?;

    Ok(
        resp_str.to_string()
    )
}