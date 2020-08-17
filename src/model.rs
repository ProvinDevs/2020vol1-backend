use chrono::{DateTime, Utc};
use uuid::Uuid;

pub struct Class {
    name: String,
    id: Uuid,
    files: Vec<File>,
}

pub struct File {
    id: Uuid,
    marker_id: String,
    resource_info: ResourceInfo,
}

pub struct ResourceInfo {
    filename: String,
    created_at: DateTime<Utc>,
}
