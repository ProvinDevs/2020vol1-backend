use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct ClassID(pub Uuid);

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct FileID(pub Uuid);

#[derive(Serialize, Deserialize, Clone)]
pub struct PassPhrase(pub String);

#[derive(Serialize, Deserialize, Clone)]
pub struct ArMarkerID(pub String);

#[derive(Serialize, Deserialize, Clone)]
pub struct Class {
    pub name: String,
    pub id: ClassID,

    #[serde(rename = "passPhrase")]
    pub pass_phrase: PassPhrase,

    pub files: Vec<File>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct File {
    pub id: FileID,

    #[serde(rename = "markerID")]
    pub marker_id: ArMarkerID,

    #[serde(rename = "resourceInfo")]
    pub resource_info: ResourceInfo,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ResourceInfo {
    #[serde(rename = "fileName")]
    pub filename: String,

    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
}
