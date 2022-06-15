use std::net::SocketAddr;
use hyper::{Body, Request, Response, Server, Method, StatusCode};
use hyper::service::{make_service_fn, service_fn};
use std::sync::Arc;
use tokio::sync::Mutex;
use common::common::Company;
use std::collections::HashMap;
use url::form_urlencoded;
use std::iter::FromIterator;

static MISSING: &[u8] = b"Missing field";

type ErrorGen = Box<dyn std::error::Error + Send + Sync>;
type Db = Arc<Mutex<Company>>;

pub async fn run_server() -> Result<(), ErrorGen> {
    // We'll bind to 127.0.0.1:3000
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    let c = Company::init().await.unwrap();
    
    let db: Db = Arc::new(Mutex::new(c));

    // A `Service` is needed for every connection, so this
    let make_svc = make_service_fn(move |_conn| {

        let db_clone = db.clone();

        async {
            Ok::<_, ErrorGen>(service_fn(move |req| {
                handle_company_reqests(req, db_clone.clone())
            }))
        }
        
    });

    let server = Server::bind(&addr).serve(make_svc);

    let graceful = server.with_graceful_shutdown(shutdown_signal());

    // Run this server for... forever!
    if let Err(e) = graceful.await {
        eprintln!("server error: {}", e);
    }

    Ok(())

}


async fn shutdown_signal() {
    // Wait for the CTRL+C signal
    tokio::signal::ctrl_c()
        .await.unwrap();
}

async fn handle_company_reqests(req: Request<Body>, db: Db) -> Result<Response<Body>, ErrorGen> {

    let mut company = db.lock().await;

    let mut response = Response::new(Body::empty());

    match (req.method(), req.uri().path()) {
        
        (&Method::GET, "/company") => {
            let query = if let Some(q) = req.uri().query() {
                q
            } else {
                return Ok(
                    Response::builder()
                    .status(StatusCode::UNPROCESSABLE_ENTITY)
                    .body(MISSING.into())?
                );
            };

            let params = form_urlencoded::parse(query.as_bytes())
                .into_owned()
                .collect::<HashMap<String, String>>();

            let department = if let Some(dept) = params.get("department") {
                dept
            } else {
                return Ok(
                    Response::builder()
                    .status(StatusCode::UNPROCESSABLE_ENTITY)
                    .body(MISSING.into())?
                );
            };

            match department.as_str() {
                "all" => {

                    match company.get_employees(&true, &None).await {
                        Ok(c) => {

                            let mut company_vec = Vec::from_iter(c.employee_list);

                            company_vec.sort();

                            let mut string_vec = vec![];

                            for (dept, employees) in company_vec {
                                let string = format!("For the {dept} department the following employees exist: {}", employees.join(", "));
                                
                                string_vec.push(string)
                            }

                            *response.body_mut() = Body::from(string_vec.join("\n"));

                        },
                        Err(e) => *response.body_mut() = Body::from(e.to_string()),
                    }
                },
                dept => {

                    match company.get_employees(&false, &Some(dept.to_string())).await {
                        Ok(c) => {
                            let mut string_vec = vec![];
                            for (dept, employees) in c.employee_list.iter() {
                                let employees_string = employees.join(", ");
                                let string = format!("For the {dept} department the following employees exist: {employees_string}");
                                string_vec.push(string)
                            }
                            *response.body_mut() = Body::from(string_vec.join("\n"));
                        },
                        Err(e) => *response.body_mut() = Body::from(e.to_string()),
                    }
                },
            }  
        },
        (&Method::POST, "/company") => {
            let query = if let Some(q) = req.uri().query() {
                q
            } else {
                return Ok(
                    Response::builder()
                    .status(StatusCode::UNPROCESSABLE_ENTITY)
                    .body(MISSING.into())?
                );
            };

            let params = form_urlencoded::parse(query.as_bytes())
                .into_owned()
                .collect::<HashMap<String, String>>();

            let name = if let Some(n) = params.get("name") {
                n
            } else {
                return Ok(
                    Response::builder()
                    .status(StatusCode::UNPROCESSABLE_ENTITY)
                    .body(MISSING.into())?
                );
            };

            let department = if let Some(dept) = params.get("department") {
                dept
            } else {
                return Ok(
                    Response::builder()
                    .status(StatusCode::UNPROCESSABLE_ENTITY)
                    .body(MISSING.into())?
                );
            };

            match company.add_employee(name, department).await {
                Ok(_) => {
                    company.save().await?;
                    let output = format!("Added {} to the {} department.", name, department);

                    *response.body_mut() = Body::from(output);
                },
                Err(e) => *response.body_mut() = Body::from(e.to_string()),
            }; 
        },
        (&Method::POST, "/company/clear") => {
            match company.clear().await {
                Ok(_) => {
                    company.save().await?;

                    *response.body_mut() = Body::from("Company cleared.");
                },
                Err(e) =>  *response.body_mut() = Body::from(e.to_string()),
            }
        },
        _ => {
            *response.status_mut() = StatusCode::NOT_FOUND;
        },
    }
    
    Ok(response)
}