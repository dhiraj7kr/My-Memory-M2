mod routes;
mod services;
mod utils;

use axum::{Router};
use std::net::SocketAddr;
use routes::api::create_routes;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let app = create_routes();
    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    println!("Running on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
