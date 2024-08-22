use zero2prod::run;
use std::net::TcpListener;
#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let listener = TcpListener::bind("localhost:0").expect("Can't bind port 8000");
    println!("Listening on port {}", listener.local_addr().unwrap().port());
    run(listener)?.await
}