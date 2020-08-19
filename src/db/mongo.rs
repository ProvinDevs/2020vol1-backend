use crate::db::{Database, DatabaseError, SimpleClassInfo};
use crate::model::*;
use async_trait::async_trait;
use mongodb::Collection;

pub struct MongoDB {
    inner: Collection,
}

impl MongoDB {
    fn new() -> Self {
        todo!()
    }
}

#[async_trait]
impl Database for MongoDB {
    async fn get_all_classes(&self) -> Result<Vec<SimpleClassInfo>, DatabaseError> {
        todo!()
    }

    async fn save_new_class(&mut self, _: &Class) -> Result<(), DatabaseError> {
        todo!()
    }

    async fn get_class_by_id(&self, class_id: &ClassID) -> Result<Class, DatabaseError> {
        todo!()
    }

    async fn get_class_by_pass_phrase(
        &self,
        pass_phrase: &PassPhrase,
    ) -> Result<Class, DatabaseError> {
        todo!()
    }

    async fn rename_class(
        &mut self,
        class_id: &ClassID,
        new_name: &str,
    ) -> Result<(), DatabaseError> {
        todo!()
    }

    async fn delete_class(&mut self, class_id: &ClassID) -> Result<Class, DatabaseError> {
        todo!()
    }

    async fn class_id_exists(&self, class_id: &ClassID) -> Result<bool, DatabaseError> {
        todo!()
    }

    async fn pass_phrase_exists(&self, class_id: &PassPhrase) -> Result<bool, DatabaseError> {
        todo!()
    }

    async fn get_files(&self, class_id: &ClassID) -> Result<Vec<File>, DatabaseError> {
        todo!()
    }

    async fn add_new_file(&mut self, class_id: &ClassID, file: &File) -> Result<(), DatabaseError> {
        todo!()
    }

    async fn get_file_by_id(&self, file_id: &FileID) -> Result<File, DatabaseError> {
        todo!()
    }

    async fn delete_file(&mut self, file_id: &FileID) -> Result<File, DatabaseError> {
        todo!()
    }

    async fn file_id_exists(&self, file_id: &FileID) -> Result<bool, DatabaseError> {
        todo!()
    }
}
