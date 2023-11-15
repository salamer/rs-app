// use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
// use flate2::read::GzDecoder;
// use std::io::prelude::*;
// use std::io::Cursor;
// use base64::{Engine as _, engine::general_purpose};
// use std::io;

// #[get("/")]
// async fn hello() -> impl Responder {
//     HttpResponse::Ok().body("Hello world!")
// }

// #[post("/echo")]
// async fn echo(req_body: String) -> impl Responder {
//     // let v = req_body.to_vec();
//     println!("req_body {:?}", req_body);
//     let bytes = general_purpose::STANDARD_NO_PAD.decode(req_body).unwrap();

//     // let mut dr = GzDecoder::new(&*bytes);

//     // let mut s = String::new();
//     // dr.read_to_string(&mut s).unwrap();
//     // println!("Decompressed: {:?}", s);

//     let mut d = Vec::new();
//     let mut rdr = snap::read::FrameDecoder::new(&bytes[..]);
//     io::copy(&mut rdr, &mut d).unwrap();

//     // let mut d = Vec::new();
//     // match dr.read_to_end(&mut d) {
//     //     Ok(_) => {
//     //         println!("Decompressed: {:?}", std::str::from_utf8(&d).unwrap());
//     //     }
//     //     Err(e) => {
//     //         println!("Error: {:?}", e);
//     //     }
//     // }
//     // println!("Decompressed: {:?}", std::str::from_utf8(&d).unwrap());
//     HttpResponse::Ok().body(())
// }

// async fn manual_hello() -> impl Responder {
//     HttpResponse::Ok().body("Hey there!")
// }

// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     HttpServer::new(|| {
//         App::new()
//             .service(hello)
//             .service(echo)
//             .route("/hey", web::get().to(manual_hello))
//     })
//     .bind(("127.0.0.1", 8080))?
//     .run()
//     .await
// }

use bytes::BytesMut;
use http::{Request, Response};
use hyper::{body::HttpBody as _, Client};
use std::convert::Infallible;
use std::io::stdout;
use tokio::io::{self, AsyncWriteExt as _};
use tower::{service_fn, Service, ServiceBuilder, ServiceExt};
use tower_http::{compression::Compression, decompression::DecompressionLayer, BoxError};
use http::{
    header::{HeaderName, HeaderValue},
    Method, StatusCode,
};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

async fn fetch_url(url: hyper::Uri) -> Result<()> {
    let client = Client::new();

    let mut builder = hyper::Request::builder().uri(url.to_string());

    let req_headers = builder.headers_mut().unwrap();

    req_headers.insert("accept-encoding", "gzip, deflate, br".parse().unwrap());

    if let Some(headers) = builder.headers_mut() {
        headers.insert("accept-encoding", "gzip, deflate, br".parse().unwrap());
    }

    let req = builder.body(hyper::Body::empty()).unwrap();

    let mut res = client.request(req).await?;

    println!("Response: {}", res.status());
    println!("Headers: {:#?}\n", res.headers());

    // Stream the body, writing each chunk to stdout as we get it
    // (instead of buffering and printing at the end).

    let mut data = Vec::new();

    while let Some(next) = res.data().await {
        let chunk = next?;
        data.push(chunk);
    }

    println!("\n\nDone! {:?} bytes retrieved", data);

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let url = "http://localhost:8090/";

    fetch_url(url.parse::<hyper::Uri>().unwrap()).await
}
