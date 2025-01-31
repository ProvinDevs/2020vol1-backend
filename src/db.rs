pub mod mem;
pub mod mongo;

use crate::model::*;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct SimpleClassInfo {
    pub name: String,
    pub id: ClassID,

    #[serde(rename = "passPhrase")]
    pub pass_phrase: PassPhrase,
}

#[async_trait]
pub trait Database: Send + Sync + 'static {
    async fn get_all_classes(&self) -> Result<Vec<SimpleClassInfo>, DatabaseError>;
    async fn save_new_class(&mut self, _: &Class) -> Result<(), DatabaseError>;
    async fn get_class_by_id(&self, class_id: &ClassID) -> Result<Class, DatabaseError>;
    async fn get_class_by_pass_phrase(
        &self,
        pass_phrase: &PassPhrase,
    ) -> Result<Class, DatabaseError>;
    async fn rename_class(
        &mut self,
        class_id: &ClassID,
        new_name: &str,
    ) -> Result<(), DatabaseError>;
    async fn delete_class(&mut self, class_id: &ClassID) -> Result<Class, DatabaseError>;
    async fn class_id_exists(&self, class_id: &ClassID) -> Result<bool, DatabaseError>;
    async fn pass_phrase_exists(&self, pass_phrase: &PassPhrase) -> Result<bool, DatabaseError>;

    async fn get_files(&self, class_id: &ClassID) -> Result<Vec<File>, DatabaseError>;
    async fn add_new_file(&mut self, class_id: &ClassID, file: &File) -> Result<(), DatabaseError>;
    async fn get_file_by_id(&self, file_id: &FileID) -> Result<File, DatabaseError>;
    async fn delete_file(&mut self, file_id: &FileID) -> Result<File, DatabaseError>;
    async fn file_id_exists(&self, file_id: &FileID) -> Result<bool, DatabaseError>;
}

#[derive(Error, Debug, PartialEq, Eq)]
#[non_exhaustive]
#[allow(dead_code)]
pub enum DatabaseError {
    #[error("specified class id not found")]
    ClassNotFound,

    #[error("specified file id not found")]
    FileNotFound,

    #[error("connection error")]
    ConnectionError,

    #[error("serialize failed")]
    SerializeFailed,

    #[error("deserialize failed, There are invalid entries in database")]
    DeserializeFailed,
}
