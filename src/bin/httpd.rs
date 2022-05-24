// binary to run an axum web server with two routes, "/" and "/data"

use axum::{routing::get, Router};
use std::net::SocketAddr;

use axumwebserver::routes::hello::hello;
use axumwebserver::routes::data::data;

#[tokio::main]
async fn main() {
    
    let app = Router::new()
        .route("/", get(hello))
        .route("/data", get(data));        
    
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();   
}
