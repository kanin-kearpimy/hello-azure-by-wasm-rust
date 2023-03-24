use anyhow::Result;
use spin_sdk::{
    http::{Request, Response},
    http_component,
};

/// A simple Spin HTTP component.
#[http_component]
fn hello_azure_by_wasm_rust(req: Request) -> Result<Response> {
    println!("{:?}", req.headers());
    Ok(http::Response::builder()
        .status(200)
        .header("content-type", "text/html")
        .body(Some("<h1>Hello Azure from WebAssembly with Rust!</h1>".into()))?)
}
