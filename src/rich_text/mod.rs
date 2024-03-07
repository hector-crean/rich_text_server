pub mod post;
use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
enum RichText {
    String(String),
    Json(JSONContent),
    JsonArray(Vec<JSONContent>),
}

#[derive(Debug, Serialize, Deserialize)]
struct JSONContent {
    #[serde(rename = "type")]
    typ: Option<String>,
    attrs: Option<HashMap<String, serde_json::Value>>,
    content: Option<Vec<JSONContent>>,
    marks: Option<Vec<Mark>>,
    text: Option<String>,
    #[serde(flatten)]
    other_fields: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Mark {
    #[serde(rename = "type")]
    typ: String,
    attrs: Option<HashMap<String, serde_json::Value>>,
    #[serde(flatten)]
    other_fields: HashMap<String, serde_json::Value>,
}

fn main() {
    // Example usage
    let json_content = JSONContent {
        typ: Some("example".to_string()),
        attrs: Some(HashMap::new()),
        content: Some(vec![]),
        marks: Some(vec![Mark {
            typ: "bold".to_string(),
            attrs: Some(HashMap::new()),
            other_fields: HashMap::new(),
        }]),
        text: Some("Hello, World!".to_string()),
        other_fields: HashMap::new(),
    };

    println!("{:?}", json_content);
}
