//! HTTP server service impl for uploading and getting images

use std::{future::Future, pin::Pin};

use http_body_util::Full;
use hyper::{
    body::{self, Bytes},
    service::Service,
    Method, Request, Response, StatusCode,
};

/// A service responsible for image usage
#[derive(Default)]
pub struct ServerService;

impl Service<Request<body::Incoming>> for ServerService {
    type Response = Response<Full<Bytes>>;
    type Error = hyper::http::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn call(&self, req: Request<body::Incoming>) -> Self::Future {
        let response = Response::builder().status(StatusCode::OK);

        let message = match *req.method() {
            Method::GET => "get".as_bytes(),
            Method::POST => "post".as_bytes(),
            _ => &[],
        };

        let res = response.body(Full::new(Bytes::copy_from_slice(message)));
        Box::pin(async { res })
    }
}
