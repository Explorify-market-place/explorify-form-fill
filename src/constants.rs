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

pub const PDF_SYS_PROMPT: &str = "You are an expert travel itinerary analyzer. Extract comprehensive trip details from the PDF.

Guidelines:
- ALWAYS provide name, description, and price - make educated guesses if needed
  * Name: Use title, heading, or create from main destination (e.g., 'Varanasi Evening Tour')
  * Description: Infer from content even if not explicitly stated (2-3 sentences)
  * Price: Extract if mentioned, otherwise estimate based on trip type and duration
- Extract other information ONLY if explicitly stated in the PDF
- For duration: only include if explicitly stated with units (hours/days/nights)
- For stops: preserve order, extract activities mentioned, skip duration unless explicitly stated in minutes
- Categories: infer trip type (e.g., 'Cultural experiences', 'Adventure', 'Spiritual tours', 'Wildlife safaris')
- Interests: infer appeal (e.g., 'Photography', 'Temples', 'Food & drink', 'History', 'Nature')
- Languages: infer from guide language mentions or PDF content language
- Skip optional fields if uncertain to avoid vendor cleanup burden

Output in JSON format matching the provided schema.";
pub const MARKDOWN_SYS_PROMPT: &str = "You are an expert travel itinerary analyzer. Extract comprehensive trip details from the text.

Guidelines:
- ALWAYS provide name, description, and price - make educated guesses if needed
  * Name: Use title, heading, or create from main destination
  * Description: Infer from content even if not explicitly stated (2-3 sentences)
  * Price: Extract if mentioned, otherwise estimate based on trip type and duration
- Extract other information ONLY if explicitly stated in the content
- For duration: only include if explicitly stated with units (hours/days/nights)
- For stops: preserve order, extract activities mentioned, skip duration unless explicitly stated
- Categories: infer trip type (e.g., 'Cultural experiences', 'Adventure', 'Day trips')
- Interests: infer appeal (e.g., 'Photography', 'Beaches', 'Local culture', 'Wildlife')
- Languages: infer from context or content language
- Skip optional fields if uncertain

Output in JSON format matching the provided schema.";
pub static OUTPUT_SCHEMA: LazyLock<Value> = LazyLock::new(|| {
    json!({
      "type": "object",
      "properties": {
        "name": {
          "type": "string",
          "description": "Trip name/title from the itinerary"
        },
        "description": {
          "type": "string",
          "description": "Short summary (2-3 sentences) inferred from the trip content"
        },
        "fullDescription": {
          "type": "string",
          "description": "Detailed description extracted from the PDF, multiple paragraphs if available"
        },
        "price": {
          "type": "number",
          "description": "Per-person price as a number (e.g., 25000 for ₹25,000). If range, use any value. Extract only if explicitly mentioned."
        },
        "duration": {
          "type": "object",
          "description": "Trip duration - ONLY include if explicitly stated with units in the PDF",
          "properties": {
            "value": {
              "type": "number",
              "description": "Duration number (e.g., 8, 3, 12)"
            },
            "unit": {
              "type": "string",
              "enum": ["hours", "days", "nights"],
              "description": "Duration unit"
            }
          },
          "required": ["value", "unit"]
        },
        "startingPoint": {
          "type": "string",
          "description": "Starting location or pickup point if explicitly mentioned (e.g., 'Hotel pickup', 'Delhi Airport')"
        },
        "endingPoint": {
          "type": "string",
          "description": "Ending location or drop-off point if explicitly mentioned"
        },
        "meetingPoint": {
          "type": "string",
          "description": "Meeting/pickup instructions if explicitly mentioned (e.g., 'Pickup From Hotel/Airport')"
        },
        "stops": {
          "type": "array",
          "description": "Itinerary stops in sequential order. Extract locations, activities, and order from the PDF.",
          "items": {
            "type": "object",
            "properties": {
              "name": {
                "type": "string",
                "description": "Stop name (e.g., 'Ganges River', 'Taj Mahal', 'Manikarnika Ghat')"
              },
              "description": {
                "type": "string",
                "description": "Stop details if mentioned in PDF, can infer brief context if clear (e.g., 'Ancient temple', 'Scenic viewpoint')"
              },
              "activities": {
                "type": "array",
                "description": "Activities at this stop explicitly mentioned (e.g., 'Boat cruise', 'Guided tour', 'Photo opportunity')",
                "items": {
                  "type": "string"
                }
              },
              "duration": {
                "type": "number",
                "description": "Minutes at this stop - ONLY include if explicitly stated in PDF (e.g., '30 minutes at temple')"
              },
              "order": {
                "type": "number",
                "description": "Sequential order in itinerary (1-indexed)"
              }
            },
            "required": ["name", "activities", "order"]
          }
        },
        "highlights": {
          "type": "array",
          "description": "Key selling points or highlights explicitly mentioned in PDF (e.g., 'Private transportation', 'Hotel pickup included', 'Professional guide')",
          "items": {
            "type": "string"
          }
        },
        "included": {
          "type": "array",
          "description": "What's included in the trip - extract from 'Included' or 'What's Included' section",
          "items": {
            "type": "string"
          }
        },
        "excluded": {
          "type": "array",
          "description": "What's NOT included - extract from 'Excluded' or 'Not Included' section",
          "items": {
            "type": "string"
          }
        },
        "whatToBring": {
          "type": "array",
          "description": "Items to bring - extract only if explicitly listed in PDF (e.g., 'Passport', 'Comfortable shoes', 'Sunscreen')",
          "items": {
            "type": "string"
          }
        },
        "notAllowed": {
          "type": "array",
          "description": "Prohibited items - extract only if explicitly listed (e.g., 'Drones', 'Alcohol', 'Pets')",
          "items": {
            "type": "string"
          }
        },
        "notSuitableFor": {
          "type": "array",
          "description": "Who should avoid this trip - extract only if explicitly mentioned (e.g., 'Pregnant women', 'People with mobility issues')",
          "items": {
            "type": "string"
          }
        },
        "knowBeforeYouGo": {
          "type": "array",
          "description": "Important information before booking - extract from 'Important Info' or 'Know Before You Go' sections",
          "items": {
            "type": "string"
          }
        },
        "categories": {
          "type": "array",
          "description": "Infer trip categories from content. Select all applicable categories from the predefined list.",
          "items": {
            "type": "string",
            "format": "enum",
            "enum": [
              "Water activities",
              "Guided tours",
              "Day trips",
              "Adventures",
              "Cultural experiences",
              "Spiritual tours",
              "Wildlife safaris",
              "Mountain treks",
              "City tours",
              "Food tours"
            ]
          }
        },
        "interests": {
          "type": "array",
          "description": "Infer interests/appeals from activities and locations. Select all applicable interests from the predefined list.",
          "items": {
            "type": "string",
            "format": "enum",
            "enum": [
              "Boat cruises",
              "Food & drink",
              "Photography",
              "Nightlife",
              "Temples",
              "Mountains",
              "Beaches",
              "History",
              "Architecture",
              "Local culture",
              "Wildlife"
            ]
          }
        },
        "languages": {
          "type": "array",
          "description": "Guide languages - infer from explicit mentions (e.g., 'English-speaking guide') or PDF language",
          "items": {
            "type": "string"
          }
        }
      },
      "required": [
        "name",
        "description",
        "stops",
        "highlights",
        "included",
        "excluded",
        "categories",
        "interests"
      ]
    })
});
