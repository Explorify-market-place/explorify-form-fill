use crate::constants::{MARKDOWN_SYS_PROMPT, OUTPUT_SCHEMA, PDF_SYS_PROMPT, get_aws_config};
use aws_sdk_lambda::{Client, primitives::Blob};
use gemini_client_api::gemini::{
    ask::Gemini,
    types::{
        request::{InlineData, Part},
        sessions::Session,
    },
};
use serde::Deserialize;
use serde_json::{Value, from_str, json, to_vec};
use std::{
    env,
    str::from_utf8,
};

#[derive(Deserialize)]
pub struct Request {
    pdf: Option<String>,
    link: Option<String>,
}

async fn link_to_markdown(url: &str) -> String {
    let request = json!({
            "body": {
                "url":url,
                "images":true,
                "limit":1
        }
    });
    let request = Blob::new(to_vec(&request).unwrap());

    let client = Client::new(get_aws_config().await);
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

pub async fn ask(request: Request) -> Value {
    let is_link: bool;
    let parts = if let Some(link) = request.link {
        is_link = true;
        vec![Part::text(link_to_markdown(&link).await.into())]
    } else {
        is_link = false;
        let pdf = request
            .pdf
            .expect("Neither link nor pdf field was present in request");
        vec![Part::inline_data(InlineData::new(
            mime::APPLICATION_PDF,
            pdf,
        ))]
    };

    let response = Gemini::new(
        env::var("GEMINI_API_KEY").unwrap(),
        "gemini-2.5-flash",
        Some(
            if is_link {
                MARKDOWN_SYS_PROMPT
            } else {
                PDF_SYS_PROMPT
            }
            .into(),
        ),
    )
    .set_json_mode(OUTPUT_SCHEMA.clone())
    .ask(Session::new(2).ask(parts))
    .await
    .unwrap();

    println!(
        "{}",
        from_str::<Value>(&response.get_chat().get_text_no_think("\n")).unwrap()
    );
    response.get_json().unwrap()
}
#[tokio::test]
pub async fn ask_link_test() {
    dbg!(
        ask(Request {
            pdf: None,
            link: Some("https://traveltechindia.netlify.app/Details/Pachmarhi".to_string())
        })
        .await
    );
}
#[tokio::test]
pub async fn ask_pdf_test() {
    use base64::{Engine as _, engine::general_purpose};

    let pdf = general_purpose::STANDARD
        .encode(std::fs::read("Manali Kasol 4N-5D Ex Delhi.pdf").unwrap());
    dbg!(
        ask(Request {
            pdf: Some(pdf),
            link: None
        })
        .await
    );
}
