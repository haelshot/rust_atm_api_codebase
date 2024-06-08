use tokio::net::TcpListener;
use std::io;
use axum;
use rust_atm::urls::app_router;
mod rust_atm;
mod app;


#[tokio::main]
async fn main() -> io::Result<()> {

    let router = app_router().await;
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("Starting Rust-Axum server ....");
    println!("Running server ....");
    println!("Listening on http://{}", listener.local_addr()?);
    println!("CTRL click on link to open");
    println!("CTRL c to close connection");

    axum::serve(listener, router).await
}
