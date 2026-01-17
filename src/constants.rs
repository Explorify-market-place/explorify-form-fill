use aws_config::{BehaviorVersion, SdkConfig};
use serde_json::{Value, json};
use tokio::sync::OnceCell;
use std::sync::LazyLock;

pub static AWS_CONFIG: OnceCell<SdkConfig> = OnceCell::const_new();
pub async fn get_aws_config() -> &'static SdkConfig {
    AWS_CONFIG.get_or_init(|| async {
        aws_config::defaults(BehaviorVersion::latest()).load().await
    }).await
}

pub const PDF_SYS_PROMPT: &str = "You are an itinerary PDF reader. Read the given pdf and output the required based on that, in JSON format.";
pub const MARKDOWN_SYS_PROMPT: &str = "You are an itinerary reader. Read the given text and output the required based on that, in JSON format.";
pub static OUTPUT_SCHEMA: LazyLock<Value> = LazyLock::new(|| {
    json!({
      "type": "object",
      "properties": {
        "name": {
          "type": "string"
        },
        "description": {
          "type": "string"
        },
        "route": {
          "type": "array",
          "description": "The starting places at first, ending place at end and all intermediate site seen in between.",
          "items": {
            "type": "string"
          }
        },
        "pricing": {
          "type": "string"
        }
      },
      "required": [
        "name",
        "description",
        "route",
      ]
    })
});
