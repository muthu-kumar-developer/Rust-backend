use axum::Router;
use dotenvy::dotenv;
use tokio::net::TcpListener;

mod dbconfig;
mod error;
mod handler;
mod model;
mod route;

#[tokio::main]
pub async fn main() {
    dotenv().ok();
    let pool = dbconfig::create_pool().await;
    let app = route::create_route(pool);
    let listener = TcpListener::bind("127.0.0.1:3000")
        .await
        .expect("Server not running");
    axum::serve(listener, app)
        .await
        .expect("server is crashing")
}

