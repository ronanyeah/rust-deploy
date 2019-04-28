use http::{Request, Response, StatusCode};
use reqwest;

fn handler(request: Request<()>) -> http::Result<Response<String>> {
    let response = Response::builder()
        .status(StatusCode::OK)
        .header(reqwest::header::CONTENT_TYPE, "text/html")
        .body(format!("<html><h1>hello world</h1><div></html>"))
        .expect("failed to render response");

    return Ok(response);
}
