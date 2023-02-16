use crate::{
    body::{Body, StreamableBody},
    cf::CloudflareProperties,
};

use wasm_bindgen::JsCast;
use worker_sys::{Headers, Request, RequestInit};

fn version_from_string(version: &str) -> http::Version {
    match version {
        "HTTP/0.9" => http::Version::HTTP_09,
        "HTTP/1.0" => http::Version::HTTP_10,
        "HTTP/1.1" => http::Version::HTTP_11,
        "HTTP/2.0" => http::Version::HTTP_2,
        "HTTP/3.0" => http::Version::HTTP_3,
        _ => unreachable!("no other versions exist"),
    }
}

pub fn request_from_js(req: Request) -> http::Request<Body> {
    let mut builder = http::Request::builder()
        .method(&*req.method())
        .uri(req.url())
        .version(version_from_string(&req.cf().http_protocol()));

    for (name, value) in req.headers() {
        builder = builder.header(name, value);
    }

    let body = match req.body() {
        Some(body) => Body::Some(
            wasm_streams::ReadableStream::from_raw(body.dyn_into().unwrap()).into_stream(),
        ),
        None => Body::None,
    };

    builder
        .extension(CloudflareProperties::from(req.cf()))
        .body(body)
        .unwrap()
}

pub fn request_to_js<B>(req: http::Request<B>) -> Request
where
    B: http_body::Body + 'static,
{
    let headers = Headers::new().unwrap();
    for (name, value) in req.headers() {
        if let Ok(value) = value.to_str() {
            headers.append(name.as_str(), value).unwrap();
        }
    }

    let method = req.method().to_string();
    let uri = req.uri().to_string();
    let body = wasm_streams::ReadableStream::from_stream(StreamableBody(req.into_body()));

    let mut init = RequestInit::new();
    init.method(&method)
        .headers(&headers)
        .body(Some(&body.into_raw()));

    Request::new_with_str_and_init(&uri, &init).unwrap()
}

// #[test]
// fn url_param_works() {
//     let url = Url::parse("https://example.com/foo.html?a=foo&b=bar&a=baz").unwrap();
//     assert_eq!(url.param("a").as_deref(), Some("foo"));
//     assert_eq!(url.param("b").as_deref(), Some("bar"));
//     assert_eq!(url.param("c").as_deref(), None);
//     let mut a_values = url.param_iter("a");
//     assert_eq!(a_values.next().as_deref(), Some("foo"));
//     assert_eq!(a_values.next().as_deref(), Some("baz"));
//     assert_eq!(a_values.next(), None);
// }

// #[test]
// fn clone_mut_works() {
//     let req = Request::new(
//         "https://example.com/foo.html?a=foo&b=bar&a=baz",
//         Method::GET,
//     )
//     .unwrap();
//     assert!(!req.immutable);
//     let mut_req = req.clone_mut().unwrap();
//     assert!(mut_req.immutable);
// }
