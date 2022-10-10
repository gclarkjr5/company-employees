use hyper::{Body, Request};
use std::str;
use tokio::io::{Error, ErrorKind};
use std::collections::HashMap;
use url::form_urlencoded;

use super::server::{Db, ErrorGen};

mod get;
mod post;

static MISSING: &[u8] = b"Missing field";

pub struct HyperRequest {
    pub req: Request<Body>,
    pub db: Db
}

impl HyperRequest {
    pub async fn init(req: Request<Body>, db: Db) -> Result<Self, ErrorGen> {
        let request = Self {
            req: req,
            db: db
        };

        Ok(request)
    }

    pub async fn get_params(&self) -> Result<HashMap<String, String>, ErrorGen> {

        let msg = str::from_utf8(MISSING).unwrap();
        let error_string = Error::new(ErrorKind::Other, msg);
     
    
        let query = if let Some(q) = self.req.uri().query() {
            q
        } else {
            return Err(Box::new(error_string))
        };
    
        let params = form_urlencoded::parse(query.as_bytes())
            .into_owned()
            .collect::<HashMap<String, String>>();
    
        let mut parameters: HashMap<String, String> = HashMap::new();
    
        for param in params.keys() {
            if let Some(p) = params.get(param) {
                let key = param.to_string();
                let value = p.to_string();
    
                parameters.insert(key, value)
            } else {
                return Err(Box::new(error_string))
            };
        }
    
        Ok(parameters)
    
    }
}