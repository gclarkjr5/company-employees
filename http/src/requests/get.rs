use hyper::{Body, Response, StatusCode};
use std::iter::FromIterator;

use super::{ErrorGen, HyperRequest, MISSING};

impl HyperRequest {
    pub async fn get(self) -> Result<Body, ErrorGen> {

        let company = self.db.lock().await;

        let params = match HyperRequest::get_params(&self).await {
            Ok(p) => p,
            Err(_) => {
                let response = Response::builder()
                    .status(StatusCode::UNPROCESSABLE_ENTITY)
                    .body(MISSING.into())?;

                return Ok(response.into_body())
            }
        };
        
        match params.get("department").unwrap().as_str() {
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

                        let response = Response::new(Body::from(string_vec.join("\n")));

                        Ok(response.into_body())
        
                    },
                    Err(e) => {
                        let response = Response::new(Body::from(e.to_string()));
                        Ok(response.into_body())
                    }
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

                        let response = Response::new(Body::from(string_vec.join("\n")));

                        Ok(response.into_body())
                    },
                    Err(e) => {
                        let response = Response::new(Body::from(e.to_string()));
                        Ok(response.into_body())
                    },
                }
            },
        }
    }
}

