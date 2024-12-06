//! HTTP server service impl for uploading and getting images

use std::{fs, future::Future, pin::Pin};

use http_body_util::Full;
use hyper::{
    body::{self, Bytes},
    service::Service,
    Method, Request, Response, StatusCode,
};
use mime_guess::MimeGuess;
use rand::thread_rng;

use crate::image::choose_random_file;

/// A service responsible for image usage
#[derive(Default)]
pub struct ServerService;

impl Service<Request<body::Incoming>> for ServerService {
    type Response = Response<Full<Bytes>>;
    type Error = hyper::http::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn call(&self, req: Request<body::Incoming>) -> Self::Future {
        let response = Response::builder();

        let res = match *req.method() {
            Method::GET => match req.uri().path() {
                "/img" => {
                    let mut rng = thread_rng();

                    if let Some(image_path) = choose_random_file("img/", &mut rng) {
                        match fs::read(&image_path) {
                            Ok(image_data) => {
                                let mime_type = MimeGuess::from_path(&image_path)
                                    .first_or_octet_stream()
                                    .to_string();
                                response
                                    .status(StatusCode::OK)
                                    .header("Content-Type", mime_type)
                                    .body(Full::new(Bytes::from(image_data)))
                            }
                            Err(_) => response
                                .status(StatusCode::INTERNAL_SERVER_ERROR)
                                .body(Full::new(Bytes::from_static(b"Failed to read image"))),
                        }
                    } else {
                        response
                            .status(StatusCode::NOT_FOUND)
                            .body(Full::new(Bytes::from_static(b"No images found")))
                    }
                }
                _ => response
                    .status(StatusCode::NOT_FOUND)
                    .body(Full::new(Bytes::from_static(b"Not Found"))),
            },
            _ => response
                .status(StatusCode::METHOD_NOT_ALLOWED)
                .body(Full::new(Bytes::from_static(b"Method Not Allowed"))),
        };

        Box::pin(async { res })
    }
}
