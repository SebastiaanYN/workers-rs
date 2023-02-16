use crate::body::Body;
use crate::body::StreamableBody;

use wasm_bindgen::JsCast;
use worker_sys::Headers;
use worker_sys::Response;
use worker_sys::ResponseInit;

pub fn response_from_js(res: Response) -> http::Response<Body> {
    let mut builder = http::Response::builder().status(res.status());

    for (name, value) in res.headers() {
        builder = builder.header(name, value);
    }

    let body = match res.body() {
        Some(body) => Body::Some(
            wasm_streams::ReadableStream::from_raw(body.dyn_into().unwrap()).into_stream(),
        ),
        None => Body::None,
    };

    builder.body(body).unwrap()
}

pub fn response_to_js<B>(res: http::Response<B>) -> Response
where
    B: http_body::Body + 'static,
{
    let headers = Headers::new().unwrap();
    for (name, value) in res.headers() {
        if let Ok(value) = value.to_str() {
            headers
                .append(name.as_str(), value)
                .expect("failed to append header");
        }
    }

    let mut init = ResponseInit::new();
    init.status(res.status().as_u16()).headers(&headers);

    let body = wasm_streams::ReadableStream::from_stream(StreamableBody(res.into_body()));
    Response::new_with_opt_stream_and_init(Some(body.into_raw().unchecked_into()), &init).unwrap()
}
