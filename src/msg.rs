use crate::state::File;
use crate::state::Metadata;
use crate::state::MetadataSchema;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InitMsg {
    // add InitMsg parameters here
    pub file: File,
    pub metadata: Metadata,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum HandleMsg {
    // add HandleMsg types here
    AddFile {
        path: String,
        content: Vec<u8>,
        mode: String,
        time: u64, 
        content_type: String,
    },
    AddMetadata {
        data: MetadataSchema,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    // add QueryMsg types here
    GetFile {
        cid: String,
    },
    GetMetadata {
        cid: String,
    },
}

/// Responses from handle function
#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum HandleAnswer {
    // add HandleMsg response types here
    AddFile { 
        cid: String,
    },
    AddMetadata { cid: String },
}

/// Responses from query function
#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryAnswer {
    GetFile {
        data: Vec<u8>,
        content_type: String,
        path: String,
    },
    GetMetadata {
        data: Vec<u8>,
    },
}
