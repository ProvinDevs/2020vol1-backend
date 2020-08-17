use crate::db::{Database, DatabaseError, SimpleClassInfo};
use crate::model::*;
use async_trait::async_trait;

pub struct MemoryDB {
    inner: Vec<Class>,
}

#[async_trait]
impl Database for MemoryDB {
    async fn get_all_classes(&self) -> Result<Vec<SimpleClassInfo>, DatabaseError> {
        let infos = self
            .inner
            .iter()
            .map(|c| SimpleClassInfo {
                name: c.name.clone(),
                id: c.id.clone(),
                pass_phrase: c.pass_phrase.clone(),
            })
            .collect();

        Ok(infos)
    }

    async fn save_new_class(&mut self, c: &Class) -> Result<(), DatabaseError> {
        self.inner.push(c.clone());
        Ok(())
    }

    async fn get_class_by_id(&self, class_id: &ClassID) -> Result<Class, DatabaseError> {
        self.inner
            .iter()
            .find(|c| c.id == *class_id)
            .map_or_else(|| Err(DatabaseError::ClassNotFound), |c| Ok(c.clone()))
    }

    async fn rename_class(
        &mut self,
        class_id: &ClassID,
        new_name: &str,
    ) -> Result<(), DatabaseError> {
        self.inner
            .iter_mut()
            .find(|c| c.id == *class_id)
            .ok_or_else(|| DatabaseError::ClassNotFound)?
            .name = new_name.to_string();

        Ok(())
    }

    async fn delete_class(&mut self, class_id: &ClassID) -> Result<Class, DatabaseError> {
        let index = self
            .inner
            .iter()
            .position(|c| c.id == *class_id)
            .ok_or_else(|| DatabaseError::ClassNotFound)?;

        Ok(self.inner.remove(index))
    }

    async fn get_files(&self, class_id: &ClassID) -> Result<Vec<File>, DatabaseError> {
        Ok(self.get_class_by_id(class_id).await?.files)
    }

    async fn add_new_file(&mut self, class_id: &ClassID, file: &File) -> Result<(), DatabaseError> {
        self.inner
            .iter_mut()
            .find(|c| c.id == *class_id)
            .ok_or_else(|| DatabaseError::ClassNotFound)?
            .files
            .push(file.clone());

        Ok(())
    }

    async fn get_file_by_id(&self, file_id: &FileID) -> Result<File, DatabaseError> {
        self.inner
            .iter()
            .flat_map(|c| c.files.iter())
            .find(|f| f.id == *file_id)
            .ok_or_else(|| DatabaseError::FileNotFound)
            .map(|f| f.clone())
    }

    async fn delete_file(&mut self, file_id: &FileID) -> Result<File, DatabaseError> {
        for class in &mut self.inner {
            if let Some(index) = class.files.iter().position(|f| f.id == *file_id) {
                return Ok(class.files.remove(index));
            }
        }

        Err(DatabaseError::FileNotFound)
    }

    async fn get_class_by_pass_phrase(
        &self,
        pass_phrase: &PassPhrase,
    ) -> Result<Class, DatabaseError> {
        self.inner
            .iter()
            .find(|c| c.pass_phrase == *pass_phrase)
            .map_or_else(|| Err(DatabaseError::ClassNotFound), |c| Ok(c.clone()))
    }

    async fn check_existing_class_by_id(&self, id: &ClassID) -> Result<bool, DatabaseError> {
        Ok(self.inner.iter().any(|c| c.id == *id))
    }

    async fn check_existing_class_by_pass_phrase(
        &self,
        pass_phrase: &PassPhrase,
    ) -> Result<bool, DatabaseError> {
        Ok(self.inner.iter().any(|c| c.pass_phrase == *pass_phrase))
    }
}
