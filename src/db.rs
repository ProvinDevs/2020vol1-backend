use crate::model::*;
use async_trait::async_trait;
use serde::Serialize;
use thiserror::Error;

#[derive(Serialize)]
pub struct SimpleClassInfo {
    pub name: String,
    pub id: ClassID,

    #[serde(rename = "passPhrase")]
    pub pass_phrase: PassPhrase,
}

#[async_trait]
pub trait Database: Send + Sync {
    async fn get_all_classes(&self) -> Result<Vec<SimpleClassInfo>, DatabaseError>;
    async fn save_new_class(&mut self, _: &Class) -> Result<(), DatabaseError>;
    async fn get_class_by_id(&self, class_id: &str) -> Result<Class, DatabaseError>;
    async fn rename_class(&mut self, class_id: &str, new_name: &str) -> Result<(), DatabaseError>;
    async fn delete_class(&mut self, class_id: &str) -> Result<Class, DatabaseError>;

    async fn get_files(&self, class_id: &str) -> Result<Vec<File>, DatabaseError>;
    async fn add_new_file(&mut self, class_id: &str, file: File) -> Result<(), DatabaseError>;
    async fn get_file_by_id(&self, file_id: &str) -> Result<File, DatabaseError>;
    async fn delete_file(&mut self, file_id: &str) -> Result<File, DatabaseError>;
}

#[derive(Error, Debug)]
#[non_exhaustive]
pub enum DatabaseError {
    #[error("specified class id not found")]
    ClassNotFound,

    #[error("specified file id not found")]
    FileNotFound,
}
