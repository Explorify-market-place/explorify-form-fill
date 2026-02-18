use crate::{
    constants::{MARKDOWN_SYS_PROMPT, OUTPUT_SCHEMA, PDF_SYS_PROMPT, get_aws_config},
    fetch::fetch_pdf_base64,
};
use aws_sdk_lambda::{Client, primitives::Blob};
use gemini_client_api::gemini::{
    ask::Gemini,
    types::{
        request::{InlineData, Part},
        sessions::Session,
    },
};
use serde::Deserialize;
use serde_json::{Value, json, to_vec};
use std::{env, error::Error, str::from_utf8};

#[derive(Deserialize)]
pub struct Request {
    pdf: Option<String>,
    link: Option<String>,
    pub secret: String,
}

async fn link_to_markdown(url: &str) -> Result<String, Box<dyn Error>> {
    let request = json!({
            "body": {
                "url":url,
                "images":true,
                "limit":1
        }
    });
    let request = Blob::new(to_vec(&request)?);

    let client = Client::new(get_aws_config().await);
    let response = client
        .invoke()
        .function_name("webscrap")
        .invocation_type(aws_sdk_lambda::types::InvocationType::RequestResponse)
        .payload(request)
        .send()
        .await?;

    Ok(from_utf8(response.payload.unwrap().as_ref())?.to_string())
}
#[tokio::test]
pub async fn link_to_markdown_test() {
    let response = link_to_markdown("https://vinaiak.com").await;
    dbg!(response.unwrap());
}

pub async fn ask(request: Request) -> Result<Value, Box<dyn Error>> {
    let is_link: bool;
    let parts = if let Some(link) = request.link {
        is_link = true;
        vec![
            Part::text("# Text\n".into()),
            Part::text(link_to_markdown(&link).await?.into()),
        ]
    } else {
        is_link = false;
        let pdf = request
            .pdf
            .expect("Neither link nor pdf field was present in request");
        vec![
            Part::text("# PDF\n".into()),
            Part::inline_data(InlineData::new(
                mime::APPLICATION_PDF,
                fetch_pdf_base64(&pdf).await?,
            )),
        ]
    };

    let response = Gemini::new(
        env::var("GEMINI_API_KEY").unwrap(),
        "gemini-3-flash-preview",
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
    .await?;

    Ok(response.get_json()?)
}
#[tokio::test]
pub async fn ask_link_test() {
    dbg!(
        ask(Request {
            pdf: None,
            link: Some("https://traveltechindia.netlify.app/Details/Pachmarhi".to_string()),
            secret: env::var("API_SECRET").unwrap()
        })
        .await
        .unwrap()
    );
}
#[tokio::test]
pub async fn ask_pdf_test() {
    dbg!(
        ask(Request {
            pdf: Some(
                "itineraries/temp-ec705976-8753-40a3-ad11-80a92909bae1-1769014010233-spiti.pdf"
                    .into()
            ),
            link: None,
            secret: env::var("API_SECRET").unwrap()
        })
        .await
        .unwrap()
    );
}
