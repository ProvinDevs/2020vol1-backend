use crate::db::{Database, DatabaseError};
use crate::Synced;
use chrono::{DateTime, Utc};
use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct ClassID(pub Uuid);

impl FromStr for ClassID {
    type Err = uuid::Error;
    fn from_str(raw_id: &str) -> Result<Self, Self::Err> {
        let id = Uuid::from_str(raw_id)?;
        Ok(ClassID(id))
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct FileID(pub Uuid);

impl FromStr for FileID {
    type Err = uuid::Error;
    fn from_str(raw_id: &str) -> Result<Self, Self::Err> {
        let id = Uuid::from_str(raw_id)?;
        Ok(FileID(id))
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq)]
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

impl Class {
    pub async fn new(db: &Synced<impl Database>, name: String) -> Result<Self, DatabaseError> {
        let id = loop {
            let generated_id = ClassID(Uuid::new_v4());
            if !db.lock().await.class_id_exists(&generated_id).await? {
                break generated_id;
            }
        };

        let pass_phrase = loop {
            let generated_phrase = PassPhrase(Self::generate_pass_phrase(6));
            if !db
                .lock()
                .await
                .pass_phrase_exists(&generated_phrase)
                .await?
            {
                break generated_phrase;
            }
        };

        Ok(Class {
            id,
            pass_phrase,
            name,
            files: vec![],
        })
    }

    #[inline]
    fn generate_pass_phrase(size: usize) -> String {
        const SEED_STR: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";

        let mut rng = &mut rand::thread_rng();
        String::from_utf8(
            SEED_STR
                .as_bytes()
                .choose_multiple(&mut rng, size)
                .cloned()
                .collect(),
        )
        .unwrap()
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct File {
    pub id: FileID,

    #[serde(rename = "markerID")]
    pub marker_id: ArMarkerID,

    #[serde(rename = "resourceInfo")]
    pub resource_info: ResourceInfo,
}

impl File {
    pub async fn new(
        db: &Synced<impl Database>,
        marker_id: ArMarkerID,
        filename: String,
        created_at: DateTime<Utc>,
    ) -> Result<File, DatabaseError> {
        let id = loop {
            let generated_id = FileID(Uuid::new_v4());
            if !db.lock().await.file_id_exists(&generated_id).await? {
                break generated_id;
            }
        };

        Ok(File {
            id,
            marker_id,
            resource_info: ResourceInfo {
                filename,
                created_at,
            },
        })
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ResourceInfo {
    #[serde(rename = "fileName")]
    pub filename: String,

    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
}
