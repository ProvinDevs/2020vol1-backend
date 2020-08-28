use crate::db::{Database, DatabaseError};
use crate::Synced;
use rand::rngs::OsRng;
use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct ClassID(pub Uuid);

impl ClassID {
    pub async fn new(db: &Synced<impl Database>) -> Result<Self, DatabaseError> {
        loop {
            let generated_id = Self(Uuid::new_v4());
            if !db.lock().await.class_id_exists(&generated_id).await? {
                break Ok(generated_id);
            }
        }
    }
}

impl FromStr for ClassID {
    type Err = uuid::Error;
    fn from_str(raw_id: &str) -> Result<Self, Self::Err> {
        let id = Uuid::from_str(raw_id)?;
        Ok(ClassID(id))
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct FileID(pub Uuid);

impl FileID {
    pub async fn new(db: &Synced<impl Database>) -> Result<Self, DatabaseError> {
        loop {
            let generated_id = Self(Uuid::new_v4());
            if !db.lock().await.file_id_exists(&generated_id).await? {
                break Ok(generated_id);
            }
        }
    }
}

impl FromStr for FileID {
    type Err = uuid::Error;
    fn from_str(raw_id: &str) -> Result<Self, Self::Err> {
        let id = Uuid::from_str(raw_id)?;
        Ok(FileID(id))
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct PassPhrase(pub String);

impl PassPhrase {
    pub async fn new(db: &Synced<impl Database>) -> Result<Self, DatabaseError> {
        const SEED_STR: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
        const PASS_LEN: usize = 6;

        loop {
            let mut rng = OsRng;
            let text = String::from_utf8(
                SEED_STR
                    .as_bytes()
                    .choose_multiple(&mut rng, PASS_LEN)
                    .cloned()
                    .collect(),
            )
            .unwrap();

            let pass = Self(text);

            if !db.lock().await.pass_phrase_exists(&pass).await? {
                break Ok(pass);
            }
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct ArMarkerID(pub String);

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct EpochTime(pub i64);

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Class {
    pub name: String,
    pub id: ClassID,

    #[serde(rename = "passPhrase")]
    pub pass_phrase: PassPhrase,

    pub files: Vec<File>,
}

impl Class {
    pub async fn new(db: &Synced<impl Database>, name: String) -> Result<Self, DatabaseError> {
        let id = ClassID::new(db).await?;
        let pass_phrase = PassPhrase::new(db).await?;

        Ok(Class {
            id,
            pass_phrase,
            name: name.to_string(),
            files: vec![],
        })
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
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
        created_at: EpochTime,
    ) -> Result<File, DatabaseError> {
        let id = FileID::new(db).await?;

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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct ResourceInfo {
    #[serde(rename = "fileName")]
    pub filename: String,

    #[serde(rename = "createdAt")]
    pub created_at: EpochTime,
}
