use std::net::TcpListener;
use rust_hub::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let address = TcpListener::bind("127.0.0.1:8000")?;
    run(address)?.await
}