//! The cloud server side of this project - holds a collection of images that can be uploaded or
//! downloaded upon request

use std::env;

use holo_server::service::ServerService;
use hyper::server::conn::http1;
use hyper_util::rt::TokioIo;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let port = env::var("PORT").unwrap_or_else(|_| "1221".to_string());
    let addr = format!("0.0.0.0:{}", port);
    let listener = TcpListener::bind(&addr).await.unwrap();
    println!(
        "Listening on http://localhost:{}",
        listener.local_addr().unwrap().port()
    );

    loop {
        // Handle connections
        let (socket, _) = listener
            .accept()
            .await
            .expect("Error accepting incoming connection");

        let io = TokioIo::new(socket);

        let service = ServerService;
        tokio::spawn(async move {
            if let Err(e) = http1::Builder::new().serve_connection(io, service).await {
                eprintln!("Error serving connection: {}", e);
            }
        });
    }
}
