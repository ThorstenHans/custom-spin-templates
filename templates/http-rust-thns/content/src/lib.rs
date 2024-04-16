use spin_sdk::http::{IntoResponse, Request, Router};
use spin_sdk::http_component;

mod api_models;
mod handlers;

#[http_component]
fn entrypoint(req: Request) -> anyhow::Result<impl IntoResponse> {
    println!("Handling request to {:?}", req.header("spin-full-url"));
    let mut router = Router::default();
    router.get("/hello", handlers::hello);
    router.post("/hello/:name", handlers::hello_you);
    Ok(router.handle(req))
}
