use std::convert::Infallible;
use std::net::SocketAddr;
use hyper::{Body, Request, Response, Server, Method, StatusCode, Error};
use hyper::service::{make_service_fn, service_fn};
use std::sync::Arc;
use tokio::sync::Mutex;
use company_employees::common::Company;
use std::collections::HashMap;
use url::form_urlencoded;

type Db = Arc<Mutex<Company>>;

static MISSING: &[u8] = b"Missing field";

async fn shutdown_signal() {
    // Wait for the CTRL+C signal
    tokio::signal::ctrl_c()
        .await
        .expect("failed to install CTRL+C signal handler");
}

async fn handle_company_reqests(req: Request<Body>) -> Result<Response<Body>, Error> {
    
    let mut response = Response::new(Body::empty());

    match (req.method(), req.uri().path()) {
        (&Method::GET, "/company/all") => {

            let company = Company::init().await.expect("error initializing company");
            
            let company_ser = serde_json::to_vec(&company.employee_list).expect("issue serializing employee list");

            *response.body_mut() = Body::from(company_ser);
        },
        (&Method::GET, "/company/department/sales") => {

            let company = Company::init().await.expect("error initializing company");

            let emps = company.get_employees(&false, &Some("sales".to_string())).await.expect("error getting employees of department");
            
            let company_ser = serde_json::to_vec(&emps.employee_list).expect("issue serializing employee list");

            *response.body_mut() = Body::from(company_ser);

        },
        (&Method::GET, "/company/department/finance") => {
            let company = Company::init().await.expect("error initializing company");

            let emps = company.get_employees(&false, &Some("finance".to_string())).await.expect("error getting employees of department");
            
            let company_ser = serde_json::to_vec(&emps.employee_list).expect("issue serializing employee list");

            *response.body_mut() = Body::from(company_ser);
        },
        (&Method::POST, "/company") => {
            let query = if let Some(q) = req.uri().query() {
                q
            } else {
                return Ok(Response::builder()
                    .status(StatusCode::UNPROCESSABLE_ENTITY)
                    .body(MISSING.into())
                    .unwrap());
            };

            let params = form_urlencoded::parse(query.as_bytes())
                .into_owned()
                .collect::<HashMap<String, String>>();

            let name = if let Some(n) = params.get("name") {
                n
            } else {
                return Ok(Response::builder()
                    .status(StatusCode::UNPROCESSABLE_ENTITY)
                    .body(MISSING.into())
                    .unwrap());
            };

            let department = if let Some(dept) = params.get("department") {
                dept
            } else {
                return Ok(Response::builder()
                    .status(StatusCode::UNPROCESSABLE_ENTITY)
                    .body(MISSING.into())
                    .unwrap());
            };

            let mut company = Company::init().await.expect("error initializing company");

            company.add_employee(name, department).await.expect("error adding employee to department");
            
            company.save().await.expect("error saving company to file");
            
            let output = format!("Added {} to the {} department.", name, department);

            *response.body_mut() = Body::from(output);
        },
        _ => {
            *response.status_mut() = StatusCode::NOT_FOUND;
        },
    }
    
    Ok(response)
}

#[tokio::main]
async fn main() {
    // We'll bind to 127.0.0.1:3000
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    // let db = Arc::new(Mutex::new(company));

    // A `Service` is needed for every connection, so this
    // creates one from our `hello_world` function.
    let make_svc = make_service_fn(|_conn| async {
        // service_fn converts our function into a `Service`
        Ok::<_, Infallible>(service_fn(handle_company_reqests))
    });

    let server = Server::bind(&addr).serve(make_svc);

    let graceful = server.with_graceful_shutdown(shutdown_signal());

    // Run this server for... forever!
    if let Err(e) = graceful.await {
        eprintln!("server error: {}", e);
    }
}
