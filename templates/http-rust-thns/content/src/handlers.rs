use spin_sdk::http::{conversions::IntoBody, IntoResponse, Params, Request, Response};

use crate::api_models::HelloYouResponseModel;

pub(crate) fn hello(_: Request, _: Params) -> anyhow::Result<impl IntoResponse> {
    Ok(Response::builder()
        .status(200)
        .header("content-type", "text/plain")
        .body("Hello, Fermyon")
        .build())
}

pub(crate) fn hello_you(_: Request, params: Params) -> anyhow::Result<impl IntoResponse> {
    let Some(name) = params.get("name") else {
        return Ok(Response::new(400, ()));
    };

    let result = HelloYouResponseModel {
        greeting: String::from("Hello"),
        name: String::from(name),
        composed_greeting: format!("Hello, {}", name),
    };
    Ok(Response::builder()
        .status(200)
        .header("Content-Type", "application/json")
        .body(result.into_body())
        .build())
}
