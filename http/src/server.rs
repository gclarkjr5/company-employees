use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::Mutex;
use common::common::Company;
use hyper::Server;
use hyper::service::{make_service_fn, service_fn};

mod handle_requests;

pub type ErrorGen = Box<dyn std::error::Error + Send + Sync>;
pub type Db = Arc<Mutex<Company>>;

pub struct HyperServer {
    addr: SocketAddr,
    db: Db
}

impl HyperServer {
    pub async fn init() -> Result<HyperServer, ErrorGen> {

        let company: Company = Company::init().await.unwrap();

        let hyperserver: HyperServer = Self {
            addr: SocketAddr::from(([127, 0, 0, 1], 3000)),
            db: Arc::new(Mutex::new(company))
        };

        Ok(hyperserver)
    }

    pub async fn run() -> Result<(), ErrorGen> {
        
        let hyperserver = HyperServer::init().await?;
    
        // A `Service` is needed for every connection, so this
        let make_svc = make_service_fn(move |_conn| {
    
            let db_clone: Db = hyperserver.db.clone();
    
            async {
                Ok::<_, ErrorGen>(service_fn(move |req| {
                    HyperServer::handle_requests(req, db_clone.clone())
                }))
            }
            
        });
    
        let server = Server::bind(&hyperserver.addr).serve(make_svc);
    
        let graceful = server.with_graceful_shutdown(shutdown_signal());
    
        // Run this server for... forever!
        if let Err(e) = graceful.await {
            eprintln!("server error: {}", e);
        }
    
        Ok(())
    
    }
}

async fn shutdown_signal() {
    // Wait for the CTRL+C signal
    tokio::signal::ctrl_c()
        .await.unwrap();
}