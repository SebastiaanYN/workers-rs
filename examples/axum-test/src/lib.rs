use axum::{body::BoxBody, routing::get, Json};
use serde_json::json;
use tower::Service;
use worker::{body::Body, event, fetch, Context, Env, Result};

mod utils;

#[event(fetch)]
pub async fn main(
    req: http::Request<Body>,
    _env: Env,
    _ctx: Context,
) -> Result<http::Response<BoxBody>> {
    utils::set_panic_hook();

    let res = axum::Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route(
            "/fetch",
            get(|| async {
                let req = http::Request::get("https://httpbin.org/headers")
                    .body("".to_string())
                    .unwrap();
                let res = fetch(req).await;
            }),
        )
        .route("/json", get(|| async { Json(json!({ "foo": "bar" })) }))
        .call(req)
        .await
        .unwrap();

    Ok(res)
}
