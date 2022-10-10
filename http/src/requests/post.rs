use hyper::{Body, Response, StatusCode};

use super::{ErrorGen, HyperRequest, MISSING};


impl HyperRequest {

    pub async fn post(self) -> Result<Body, ErrorGen> {
        let mut company = self.db.lock().await;

        let params = match HyperRequest::get_params(&self).await {
            Ok(p) => p,
            Err(_) => {
                let response = Response::builder()
                    .status(StatusCode::UNPROCESSABLE_ENTITY)
                    .body(MISSING.into())?;

                return Ok(response.into_body())
            }
        };
        
        let name = params.get("name").unwrap();
        let department = params.get("department").unwrap();
        

        match company.add_employee(name, department).await {
            Ok(_) => {
                company.save().await?;
                let output = format!("Added {} to the {} department.", name, department);

                let response = Response::new(Body::from(output));

                Ok(response.into_body())
            },
            Err(e) => {
                let response = Response::new(Body::from(e.to_string()));
                Ok(response.into_body())
            },
        }
    }
}

