use aws_config::BehaviorVersion;
use aws_sdk_lambda::{Client, primitives::Blob};
use serde_json::{json, to_vec};
use std::str::from_utf8;

pub async fn link_to_markdown(url: &str) -> String {
    let request = json!({
            "body": {
                "url":url,
                "images":true,
                "limit":1
        }
    });
    let request = Blob::new(to_vec(&request).unwrap());

    let client = Client::new(&aws_config::defaults(BehaviorVersion::latest()).load().await);
    let response = client
        .invoke()
        .function_name("webscrap")
        .invocation_type(aws_sdk_lambda::types::InvocationType::RequestResponse)
        .payload(request)
        .send()
        .await
        .expect("Failed to webscrap function");

    from_utf8(response.payload.unwrap().as_ref())
        .unwrap()
        .to_string()
}

#[tokio::test]
pub async fn link_to_markdown_test() {
    let response = link_to_markdown("https://vinaiak.com").await;
    dbg!(response);
}
