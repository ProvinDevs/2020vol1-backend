use crate::db::{Database, DatabaseError, SimpleClassInfo};
use crate::model::*;
use async_trait::async_trait;
use mongodb::bson::{self, doc, Bson, Document};
use mongodb::error::Error as MongoDBError;
use mongodb::options::ClientOptions;
use mongodb::{Client, Collection};
use serde::de::DeserializeOwned;
use std::time::Duration;
use tokio::stream::StreamExt;

pub struct MongoDB {
    inner: Collection,
}

impl MongoDB {
    pub async fn new(url: &str) -> Result<MongoDB, MongoDBError> {
        let mut client_options = ClientOptions::parse(url).await?;

        client_options.app_name = Some("Blackboard".into());
        client_options.min_pool_size = Some(0);
        client_options.max_pool_size = Some(1);
        client_options.max_idle_time = Some(Duration::from_secs(15));

        let database = Client::with_options(client_options)?.database("blackboard");
        let entries = database.collection("classes");

        Ok(MongoDB { inner: entries })
    }

    async fn search_by_doc<T>(&self, doc: impl Into<Option<Document>>) -> Result<T, DatabaseError>
    where
        T: DeserializeOwned,
    {
        let doc = self
            .inner
            .find_one(doc, None)
            .await
            .map_err(le(DatabaseError::ConnectionError))?
            .ok_or_else(|| DatabaseError::ClassNotFound)?;

        bson::from_document(doc).map_err(le(DatabaseError::DeserializeFailed))
    }
}

// (Log Error)
fn le<E, OE>(error: E) -> impl FnOnce(OE) -> E
where
    OE: std::fmt::Debug,
{
    move |o: _| {
        log::error!("MongoDB Error: {:?}", &o);
        error
    }
}

#[async_trait]
impl Database for MongoDB {
    async fn get_all_classes(&self) -> Result<Vec<SimpleClassInfo>, DatabaseError> {
        self.inner
            .aggregate(
                vec![doc! {
                    "$project": {
                        "files": false
                    }
                }],
                None,
            )
            .await
            .map_err(le(DatabaseError::ConnectionError))?
            .map(|d| d.map(bson::from_document::<SimpleClassInfo>))
            .map(|d| d.map(|s| s.map_err(le(DatabaseError::DeserializeFailed))))
            .collect::<Result<Result<Vec<_>, _>, _>>()
            .await
            .map_err(le(DatabaseError::ConnectionError))?
    }

    async fn save_new_class(&mut self, class: &Class) -> Result<(), DatabaseError> {
        let doc = bson::to_document(class).map_err(le(DatabaseError::SerializeFailed))?;

        self.inner
            .insert_one(doc, None)
            .await
            .map_err(le(DatabaseError::ConnectionError))?;

        Ok(())
    }

    async fn get_class_by_id(&self, class_id: &ClassID) -> Result<Class, DatabaseError> {
        self.search_by_doc(doc! { "id": class_id.0.to_string() })
            .await
    }

    async fn get_class_by_pass_phrase(
        &self,
        pass_phrase: &PassPhrase,
    ) -> Result<Class, DatabaseError> {
        self.search_by_doc(doc! { "pass_phrase": &pass_phrase.0 })
            .await
    }

    async fn rename_class(
        &mut self,
        class_id: &ClassID,
        new_name: &str,
    ) -> Result<(), DatabaseError> {
        self.inner
            .update_one(
                doc! { "id": class_id.0.to_string() },
                doc! { "name": new_name },
                None,
            )
            .await
            .map_err(le(DatabaseError::ConnectionError))?;

        Ok(())
    }

    async fn delete_class(&mut self, class_id: &ClassID) -> Result<Class, DatabaseError> {
        let class = self
            .search_by_doc(doc! { "id": class_id.0.to_string() })
            .await?;

        self.inner
            .delete_one(doc! { "id": class_id.0.to_string() }, None)
            .await
            .map_err(le(DatabaseError::ConnectionError))?;

        Ok(class)
    }

    async fn class_id_exists(&self, class_id: &ClassID) -> Result<bool, DatabaseError> {
        let result = self
            .inner
            .find_one(doc! { "id": class_id.0.to_string() }, None)
            .await
            .map_err(le(DatabaseError::ConnectionError))?;

        Ok(result.is_some())
    }

    async fn pass_phrase_exists(&self, pass_phrase: &PassPhrase) -> Result<bool, DatabaseError> {
        let result = self
            .inner
            .find_one(doc! { "pass_phrase": &pass_phrase.0 }, None)
            .await
            .map_err(le(DatabaseError::ConnectionError))?;

        Ok(result.is_some())
    }

    async fn get_files(&self, class_id: &ClassID) -> Result<Vec<File>, DatabaseError> {
        struct DBResponse {
            files: Vec<File>,
        }

        self.inner
            .aggregate(vec![doc! {}], None)
            .await
            .map_err(le(DatabaseError::ConnectionError))?
            .next()
            .await
            .ok_or_else(|| DatabaseError::FileNotFound);

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
