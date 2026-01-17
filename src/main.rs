use lambda_runtime::{LambdaEvent, service_fn, tracing};
use serde::Deserialize;
use serde_json::{Value, from_str};
use std::error::Error;

use crate::ask::{Request, ask};
pub mod ask;
pub mod constants;

#[derive(Deserialize)]
struct Event {
    body: String,
}
async fn handler(event: LambdaEvent<Event>) -> Result<Value, Box<dyn Error>> {
    let request: Request = from_str(&event.payload.body)?;
    ask(request).await
}
#[tokio::main]
async fn main() {
    tracing::init_default_subscriber();
    lambda_runtime::run(service_fn(handler)).await.unwrap();
}
