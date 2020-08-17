use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct ClassID(pub Uuid);

#[derive(Serialize, Deserialize)]
pub struct FileID(pub Uuid);

#[derive(Serialize, Deserialize)]
pub struct PassPhrase(pub String);

#[derive(Serialize, Deserialize)]
pub struct ArMarkerID(pub String);

#[derive(Serialize, Deserialize)]
pub struct Class {
    pub name: String,
    pub id: ClassID,

    #[serde(rename = "passPhrase")]
    pub pass_phrase: PassPhrase,

    pub files: Vec<File>,
}

#[derive(Serialize, Deserialize)]
pub struct File {
    pub id: FileID,

    #[serde(rename = "markerID")]
    pub marker_id: ArMarkerID,

    #[serde(rename = "resourceInfo")]
    pub resource_info: ResourceInfo,
}

#[derive(Serialize, Deserialize)]
pub struct ResourceInfo {
    #[serde(rename = "fileName")]
    pub filename: String,

    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
}
