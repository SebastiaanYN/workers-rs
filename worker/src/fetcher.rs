use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use worker_sys::{Fetcher as FetcherSys, Response as ResponseSys};

use crate::{
    body::Body, env::EnvBinding, request::request_to_js, response::response_from_js, Result,
};

/// A struct for invoking fetch events to other Workers.
pub struct Fetcher(FetcherSys);

impl Fetcher {
    /// Invoke a fetch event with an existing [Request].
    pub async fn fetch<B>(&self, request: http::Request<B>) -> Result<http::Response<Body>>
    where
        B: http_body::Body + 'static,
    {
        let promise = self.0.fetch(&request_to_js(request));
        let resp_sys: ResponseSys = JsFuture::from(promise).await?.dyn_into()?;
        Ok(response_from_js(resp_sys))
    }
}

impl EnvBinding for Fetcher {
    const TYPE_NAME: &'static str = "Fetcher";
}

impl JsCast for Fetcher {
    fn instanceof(val: &wasm_bindgen::JsValue) -> bool {
        val.is_instance_of::<Fetcher>()
    }

    fn unchecked_from_js(val: wasm_bindgen::JsValue) -> Self {
        Self(val.into())
    }

    fn unchecked_from_js_ref(val: &wasm_bindgen::JsValue) -> &Self {
        unsafe { &*(val as *const JsValue as *const Self) }
    }
}

impl From<Fetcher> for JsValue {
    fn from(service: Fetcher) -> Self {
        JsValue::from(service.0)
    }
}

impl AsRef<wasm_bindgen::JsValue> for Fetcher {
    fn as_ref(&self) -> &wasm_bindgen::JsValue {
        &self.0
    }
}

impl From<FetcherSys> for Fetcher {
    fn from(inner: FetcherSys) -> Self {
        Self(inner)
    }
}
