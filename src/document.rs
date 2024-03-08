use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block {
    id: Uuid,
    #[serde(rename = "type")]
    ty: String,
    props: HashMap<String, serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Document {
    blocks: Vec<Block>,
}

impl Document {
    pub fn update_block_props(
        &mut self,
        block_id: Uuid,
        new_props: HashMap<String, serde_json::Value>,
    ) -> Result<(), String> {
        // Find the block with the matching ID
        if let Some(block) = self.blocks.iter_mut().find(|block| block.id == block_id) {
            // Update the props of the found block
            block.props = new_props;
            Ok(())
        } else {
            Err("Block not found".to_string())
        }
    }
}
