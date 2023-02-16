use std::ops::Deref;

use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;

use worker_sys::{RequestInit, Response, WorkerGlobalScope};

use crate::{body::Body, request::request_to_js, response::response_from_js, AbortSignal, Result};

async fn fetch_impl<B>(
    req: http::Request<B>,
    signal: Option<&AbortSignal>,
) -> Result<http::Response<Body>>
where
    B: http_body::Body + 'static,
{
    let mut init = RequestInit::new();
    init.signal(signal.map(|s| s.deref()));

    let worker: WorkerGlobalScope = js_sys::global().unchecked_into();
    let req = request_to_js(req);
    let promise = worker.fetch_with_request_and_init(&req, &init);
    let resp = JsFuture::from(promise).await?;
    let edge_response: Response = resp.dyn_into()?;

    Ok(response_from_js(edge_response))
}

pub async fn fetch<B>(req: http::Request<B>) -> Result<http::Response<Body>>
where
    B: http_body::Body + 'static,
{
    fetch_impl(req, None).await
}

pub async fn fetch_with_signal<B>(
    req: http::Request<B>,
    signal: &AbortSignal,
) -> Result<http::Response<Body>>
where
    B: http_body::Body + 'static,
{
    fetch_impl(req, Some(signal)).await
}
