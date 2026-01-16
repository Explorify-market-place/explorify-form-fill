use lambda_runtime::{Error, LambdaEvent, service_fn, tracing};
use serde::Deserialize;
pub mod ask;

#[derive(Deserialize)]
struct Event {
    body: String,
}
async fn handler(event: LambdaEvent<Event>)->Result<(), Error> {
    todo!()
}
#[tokio::main]
async fn main() {
    tracing::init_default_subscriber();
    lambda_runtime::run(service_fn(handler)).await.unwrap();
}
