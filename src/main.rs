//import axum and other dependencies
use axum::{routing::get, Router};
use std::net::SocketAddr;
#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new().route("/", get(|| async { "Hello, World!" }));

    // fire up the server
    let ip_address = SocketAddr::from(([127, 0, 0, 1], 3456));
    axum::Server::bind(&ip_address)
        .serve(app.into_make_service())
        .await
        .unwrap();
    println!("Ignition started on http://{}", ip_address.to_string());
}
