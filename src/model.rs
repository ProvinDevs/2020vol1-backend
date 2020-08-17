use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct Class {
    name: String,
    id: Uuid,

    #[serde(rename = "passPhrase")]
    pass_phrase: String,

    files: Vec<File>,
}

#[derive(Serialize, Deserialize)]
pub struct File {
    id: Uuid,

    #[serde(rename = "markerID")]
    marker_id: String,

    #[serde(rename = "resourceInfo")]
    resource_info: ResourceInfo,
}

#[derive(Serialize, Deserialize)]
pub struct ResourceInfo {
    #[serde(rename = "fileName")]
    filename: String,

    #[serde(rename = "createdAt")]
    created_at: DateTime<Utc>,
}
