use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct ClassID(Uuid);

#[derive(Serialize, Deserialize)]
pub struct FileID(Uuid);

#[derive(Serialize, Deserialize)]
pub struct PassPhrase(String);

#[derive(Serialize, Deserialize)]
pub struct ArMarkerID(String);

#[derive(Serialize, Deserialize)]
pub struct Class {
    name: String,
    id: ClassID,

    #[serde(rename = "passPhrase")]
    pass_phrase: PassPhrase,

    files: Vec<File>,
}

#[derive(Serialize, Deserialize)]
pub struct File {
    id: FileID,

    #[serde(rename = "markerID")]
    marker_id: ArMarkerID,

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
