//! The cloud server side of this project - holds a collection of images that can be uploaded or
//! downloaded upon request

use holo_server::service::ServerService;
use hyper::server::conn::http1;
use hyper_util::rt::TokioIo;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("0.0.0.0:0").await.unwrap();
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
