use std::env;

use lambda_runtime::{LambdaEvent, service_fn, tracing};
use serde::Deserialize;
use serde_json::{Value, from_str, json};

use crate::ask::{Request, ask};
pub mod ask;
pub mod constants;
pub mod fetch;

#[derive(Deserialize)]
struct Event {
    body: String,
}
async fn handler(event: LambdaEvent<Event>) -> Result<Value, lambda_runtime::Error> {
    let request: Request = match from_str::<Request>(&event.payload.body) {
        Ok(request) => {
            if request.secret == env::var("API_SECRET").unwrap() {
                request
            } else {
                print!("Authentication failed");
                return Ok(json!({
                    "statusCode":401,
                    "body":"Authentication failed"
                }));
            }
        }
        Err(e) => {
            return Ok(json!({
                "statusCode":400,
                "body":format!("Invalid fomat. Failed to parse\n{e}")
            }));
        }
    };

    let response = ask(request).await;
    match response {
        Ok(response) => {
            println!("Processed request.\n{response}");
            Ok(json!({
                "statusCode":200,
                "body":response,
            }
            ))
        }
        Err(e) => {
            eprintln!("Failed to process request: {e}");
            Ok(json!({
                "statusCode":400,
                "body":e.to_string(),
            }
            ))
        }
    }
}
#[tokio::main]
async fn main() {
    tracing::init_default_subscriber();
    lambda_runtime::run(service_fn(handler)).await.unwrap();
}
