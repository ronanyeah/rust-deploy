use http::{Request, Response, StatusCode};
use juniper::{graphql_value, FieldError, FieldResult, RootNode, Variables};
use now_lambda::lambda;
use reqwest;
use serde_json;
use std::error::Error;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct Query;

pub struct Mutation;

type Context = ();

#[juniper::object(Context = Context)]
impl Query {
    fn time() -> FieldResult<i32> {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|t| t.as_secs() as i32)
            .map_err(|_| FieldError::new("yeh", graphql_value!({ "internal_error": "oops" })))
    }
}

#[juniper::object(Context = Context)]
impl Mutation {
    fn echo(txt: String) -> FieldResult<String> {
        Ok(txt)
    }
}

fn handler(req: Request<String>) -> http::Result<Response<String>> {
    let ctx = ();
    let body = req.body();

    let (res, _errors) = juniper::execute(
        body,
        None,
        &RootNode::new(Query, Mutation),
        &Variables::new(),
        &ctx,
    )
    .unwrap();

    let content = res.as_object_value().unwrap();

    let content_str = serde_json::to_string_pretty(content).expect("Failed to serialize to JSON");

    let response = Response::builder()
        .status(StatusCode::OK)
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .body(content_str)
        .expect("failed to render response");

    Ok(response)
}

fn main() -> Result<(), Box<dyn Error>> {
    Ok(lambda!(handler))
}
