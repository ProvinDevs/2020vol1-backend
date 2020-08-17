extern crate rand;
use crate::db::{Database, DatabaseError};
use chrono::{DateTime, Utc};
use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

const SEED_STR: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct ClassID(pub Uuid);

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct FileID(pub Uuid);

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
    pub async fn new(db: impl Database, name: String) -> Result<Self, DatabaseError> {
        let id: Uuid;
        loop {
            let tmp = Uuid::new_v4();
            if !db.check_existing_class_by_id(&ClassID(tmp)).await? {
                id = tmp;
                break;
            }
        }

        let pass_phrase: PassPhrase;
        loop {
            let generated_phrase = Self::generate_pass_phrase(6);
            if !db
                .check_existing_class_by_pass_phrase(&PassPhrase(generated_phrase.clone()))
                .await?
            {
                pass_phrase = PassPhrase(generated_phrase);
                break;
            }
        }

        Ok(Class {
            id: ClassID(id),
            pass_phrase: pass_phrase,
            name: name,
            files: vec![],
        })
    }

    fn generate_pass_phrase(size: usize) -> String {
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
        db: impl Database,
        marker_id: ArMarkerID,
        filename: String,
        created_at: DateTime<Utc>,
    ) -> Result<File, DatabaseError> {
        let id: Uuid;
        loop {
            let tmp = Uuid::new_v4();
            if !db.check_existing_class_by_id(&ClassID(tmp)).await? {
                id = tmp;
                break;
            }
        }

        Ok(File {
            id: FileID(id),
            marker_id: marker_id,
            resource_info: ResourceInfo {
                filename: filename,
                created_at: created_at,
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
