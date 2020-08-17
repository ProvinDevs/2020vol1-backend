use crate::model::*;
use serde::Serialize;
use std::fmt::Debug;

#[derive(Serialize)]
pub struct SimpleClassInfo {
    name: String,
    id: ClassID,

    #[serde(rename = "passPhrase")]
    pass_phrase: PassPhrase,
}

pub trait Database: Send {
    type Error: Debug + Send;

    fn get_all_classes(&self) -> Result<Vec<SimpleClassInfo>, Self::Error>;
    fn save_new_class(&mut self, _: &Class) -> Result<(), Self::Error>;
    fn get_class_by_id(&self, class_id: &str) -> Result<Class, Self::Error>;
    fn rename_class(&mut self, class_id: &str, new_name: &str) -> Result<(), Self::Error>;
    fn delete_class(&mut self, class_id: &str) -> Result<Class, Self::Error>;

    fn get_files(&self, class_id: &str) -> Result<Vec<File>, Self::Error>;
    fn add_new_file(&mut self, class_id: &str, file: File) -> Result<(), Self::Error>;
    fn get_file_by_id(&self, file_id: &str) -> Result<File, Self::Error>;
    fn delete_file(&mut self, file_id: &str) -> Result<File, Self::Error>;
}
