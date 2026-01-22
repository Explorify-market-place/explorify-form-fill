use base64::engine::{Engine, general_purpose};
use crate::constants::get_aws_config;
use std::error::Error;

const BUCKET_NAME: &str = "explorify-trips-ap-south-1";

pub async fn fetch_pdf_base64(key: &str) -> Result<String, Box<dyn Error>> {
    let client = aws_sdk_s3::Client::new(get_aws_config().await);
    let response = client
        .get_object()
        .bucket(BUCKET_NAME)
        .key(key)
        .send()
        .await?;
    let data = response.body.collect().await?.into_bytes();

    Ok(general_purpose::STANDARD.encode(data))
}
