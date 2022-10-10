use super::{Db, ErrorGen, HyperServer};
use super::super::requests::HyperRequest;

use hyper::{Body, Request, Response, Method, StatusCode};


impl HyperServer {
    pub async fn handle_requests(req: Request<Body>, db: Db) -> Result<Response<Body>, ErrorGen> {

        let hyper_request = HyperRequest::init(req, db).await?;
    
        let mut response = Response::new(Body::empty());

        match (hyper_request.req.method(), hyper_request.req.uri().path()) {
            
            (&Method::GET, "/company") => {

                let hyper_response = hyper_request.get().await?;

                *response.body_mut() = hyper_response

            },
            (&Method::POST, "/company") => {
                let hyper_response = hyper_request.post().await?;

                *response.body_mut() = hyper_response
                 
            },
            (&Method::POST, "/company/clear") => {
                let mut company = hyper_request.db.lock().await;
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
}