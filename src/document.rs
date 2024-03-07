use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

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

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
enum RichText {
    String(String),
    Json(JSONContent),
    JsonArray(Vec<JSONContent>),
}

#[derive(Serialize, Deserialize)]
pub struct RichTextProps {
    text: RichText,
}

enum Node {
    RichTextNode(RenderableNode),
    Unknown {
        id: Uuid,
        props: HashMap<String, serde_json::Value>,
    },
}

#[derive(Serialize, Deserialize, Debug)]
struct Block<T, P> {
    id: Uuid,
    #[serde(rename = "type")]
    typ: T,
    props: P,
}

type RenderableNode<T, P> = Block<T, P>;

#[derive(Serialize, Deserialize, Debug)]
struct Document<T: Into<String>, P: Into<HashMap<String, serde_json::Value>>> {
    blocks: Vec<Block<T, P>>,
}
