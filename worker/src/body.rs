use std::{
    pin::Pin,
    task::{Context, Poll},
};

use bytes::Buf;
use futures_util::{ready, Stream};
use pin_project::pin_project;
use wasm_bindgen::JsValue;
use wasm_streams::readable::IntoStream;

#[derive(Debug)]
#[pin_project(project = BodyProj)]
pub enum Body {
    Some(#[pin] IntoStream<'static>),
    None,
}

unsafe impl Send for Body {}

impl http_body::Body for Body {
    type Data = bytes::Bytes;

    type Error = ();

    fn poll_data(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Result<Self::Data, Self::Error>>> {
        let this = match self.project() {
            BodyProj::Some(inner) => inner,
            BodyProj::None => return Poll::Ready(None),
        };

        match ready!(this.poll_next(cx)) {
            Some(Ok(res)) => Poll::Ready(Some(Ok(js_sys::Uint8Array::from(res).to_vec().into()))),
            Some(Err(_)) => Poll::Ready(Some(Err(()))),
            None => Poll::Ready(None),
        }
    }

    fn poll_trailers(
        self: Pin<&mut Self>,
        _cx: &mut Context<'_>,
    ) -> Poll<Result<Option<http::HeaderMap>, Self::Error>> {
        Poll::Ready(Ok(None))
    }
}

#[derive(Debug)]
#[pin_project]
pub struct StreamableBody<T: http_body::Body>(#[pin] pub T);

impl<T: http_body::Body> Stream for StreamableBody<T> {
    type Item = Result<JsValue, JsValue>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let this = self.project();

        match ready!(this.0.poll_data(cx)) {
            Some(Ok(data)) => Poll::Ready(Some(Ok(js_sys::Uint8Array::from(data.chunk()).into()))),
            Some(Err(_)) => Poll::Ready(Some(Ok(JsValue::NULL))),
            None => Poll::Ready(None),
        }
    }
}
