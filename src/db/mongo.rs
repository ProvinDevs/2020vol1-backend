use crate::db::{Database, DatabaseError, SimpleClassInfo};
use crate::model::*;
use async_trait::async_trait;
use mongodb::bson::{self, doc, Bson, Document};
use mongodb::error::Error as MongoDBError;
use mongodb::options::ClientOptions;
use mongodb::{Client, Collection};
use serde::de::DeserializeOwned;
use serde::Deserialize;
use std::time::Duration;
use tokio::stream::StreamExt;

pub struct MongoDB {
    inner: Collection,
}

// (Log Error)
fn le<E, OE>(error: E) -> impl FnOnce(OE) -> E
where
    OE: std::fmt::Debug,
{
    move |o| {
        log::error!("MongoDB Error: {:?}", &o);
        error
    }
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

    async fn search_by_doc<T>(
        &self,
        doc: impl Into<Option<Document>>,
    ) -> Result<Option<T>, DatabaseError>
    where
        T: DeserializeOwned,
    {
        self.inner
            .find_one(doc, None)
            .await
            .map_err(le(DatabaseError::ConnectionError))?
            .map(bson::from_document)
            .map(|e| e.map_err(le(DatabaseError::DeserializeFailed)))
            .transpose()
    }

    async fn aggregate_one_and_parse<T>(
        &self,
        pipeline: Vec<Document>,
    ) -> Result<Option<T>, DatabaseError>
    where
        T: DeserializeOwned,
    {
        self.inner
            .aggregate(pipeline, None)
            .await
            .map_err(le(DatabaseError::ConnectionError))?
            .next()
            .await
            .transpose()
            .map_err(le(DatabaseError::ConnectionError))?
            .map(bson::from_document)
            .map(|e| e.map_err(le(DatabaseError::DeserializeFailed)))
            .transpose()
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
            .await?
            .ok_or_else(|| DatabaseError::ClassNotFound)
    }

    async fn get_class_by_pass_phrase(
        &self,
        pass_phrase: &PassPhrase,
    ) -> Result<Class, DatabaseError> {
        self.search_by_doc(doc! { "passPhrase": &pass_phrase.0 })
            .await?
            .ok_or_else(|| DatabaseError::ClassNotFound)
    }

    async fn rename_class(
        &mut self,
        class_id: &ClassID,
        new_name: &str,
    ) -> Result<(), DatabaseError> {
        self.inner
            .update_one(
                doc! { "id": class_id.0.to_string() },
                doc! { "$set": { "name": new_name } },
                None,
            )
            .await
            .map_err(le(DatabaseError::ConnectionError))?;

        Ok(())
    }

    async fn delete_class(&mut self, class_id: &ClassID) -> Result<Class, DatabaseError> {
        let class = self
            .search_by_doc(doc! { "id": class_id.0.to_string() })
            .await?
            .ok_or_else(|| DatabaseError::ClassNotFound)?;

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
            .find_one(doc! { "passPhrase": &pass_phrase.0 }, None)
            .await
            .map_err(le(DatabaseError::ConnectionError))?;

        Ok(result.is_some())
    }

    async fn get_files(&self, class_id: &ClassID) -> Result<Vec<File>, DatabaseError> {
        #[derive(Deserialize)]
        struct DBResponse {
            files: Vec<File>,
        }

        let response = self
            .aggregate_one_and_parse::<DBResponse>(vec![
                doc! {
                    "$match": {
                        "id": class_id.0.to_string()
                    }
                },
                doc! {
                    "$project": {
                        "files": true
                    }
                },
            ])
            .await?
            .ok_or_else(|| DatabaseError::ClassNotFound)?;

        Ok(response.files)
    }

    async fn add_new_file(&mut self, class_id: &ClassID, file: &File) -> Result<(), DatabaseError> {
        let file_doc = bson::to_document(file).map_err(le(DatabaseError::SerializeFailed))?;

        let update_result = self
            .inner
            .update_one(
                doc! { "id": class_id.0.to_string() },
                doc! { "$push": { "files": file_doc }},
                None,
            )
            .await
            .map_err(le(DatabaseError::ConnectionError))?;

        if update_result.matched_count == 1 && update_result.modified_count == 1 {
            Ok(())
        } else {
            log::error!(
                "couldn't update: match: {}, mod: {}",
                update_result.matched_count,
                update_result.modified_count
            );
            Err(DatabaseError::ClassNotFound)
        }
    }

    async fn get_file_by_id(&self, file_id: &FileID) -> Result<File, DatabaseError> {
        #[derive(Deserialize)]
        struct DBResponse {
            files: File,
        }

        self.aggregate_one_and_parse::<DBResponse>(vec![
            doc! {
                "$project": {
                    "files": true
                }
            },
            doc! {
                "$unwind": {
                    "path": "$files"
                }
            },
            doc! {
                "$match": {
                    "files.id": file_id.0.to_string()
                }
            },
        ])
        .await?
        .map(|e| e.files)
        .ok_or_else(|| DatabaseError::FileNotFound)
    }

    async fn delete_file(&mut self, file_id: &FileID) -> Result<File, DatabaseError> {
        let file = self.get_file_by_id(file_id).await?;

        let result = self
            .inner
            .update_many(
                doc! {},
                doc! { "$pull": { "files": { "id": file_id.0.to_string() } } },
                None,
            )
            .await
            .map_err(le(DatabaseError::ConnectionError))?;

        assert!(result.modified_count == 1);

        Ok(file)
    }

    async fn file_id_exists(&self, file_id: &FileID) -> Result<bool, DatabaseError> {
        let result = self
            .inner
            .aggregate(
                vec![
                    doc! {
                        "$project": {
                            "files": true
                        }
                    },
                    doc! {
                        "$unwind": {
                            "path": "$files"
                        }
                    },
                    doc! {
                        "$match": {
                            "files.id": file_id.0.to_string()
                        }
                    },
                ],
                None,
            )
            .await
            .map_err(le(DatabaseError::ConnectionError))?
            .next()
            .await
            .is_some();

        Ok(result)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::sync::Arc;
    use tokio::runtime::Builder;
    use tokio::sync::Mutex;

    // requires mongodb on localhost.
    #[test]
    fn mongo_test() {
        env_logger::init();
        async fn test() {
            let db = MongoDB::new("mongodb://localhost")
                .await
                .expect("failed to create mongodb handle");

            let db = Arc::new(Mutex::new(db));

            let mut classes = vec![
                Class::new(&db, "理科".into())
                    .await
                    .expect("failed to create class"),
                Class::new(&db, "社会".into())
                    .await
                    .expect("failed to create class"),
            ];

            // save_new_class, get_all_classes
            {
                for class in &classes {
                    db.lock()
                        .await
                        .save_new_class(&class)
                        .await
                        .expect("failed to save class");
                }

                let got_response = db
                    .lock()
                    .await
                    .get_all_classes()
                    .await
                    .expect("failed to retrieve classes");

                let models = classes.iter().map(|class| SimpleClassInfo {
                    name: class.name.clone(),
                    id: class.id.clone(),
                    pass_phrase: class.pass_phrase.clone(),
                });

                for model in models {
                    assert!(got_response.contains(&model));
                }
            }

            // get_class_by_id
            {
                let got_response = db
                    .lock()
                    .await
                    .get_class_by_id(&classes[0].id)
                    .await
                    .expect("failed to retrieve class");

                assert_eq!(got_response, classes[0])
            }

            // get_class_by_passphrase
            {
                let got_response = db
                    .lock()
                    .await
                    .get_class_by_pass_phrase(&classes[1].pass_phrase)
                    .await
                    .expect("failed to retrieve class");

                assert_eq!(got_response, classes[1])
            }

            // class_id_exists
            {
                let res = db
                    .lock()
                    .await
                    .class_id_exists(&classes[1].id)
                    .await
                    .expect("failed to check whether class id exists");

                assert!(res);

                let res = {
                    let not_exist_id = ClassID::new(&db)
                        .await
                        .expect("failed to generate class id");

                    db.lock()
                        .await
                        .class_id_exists(&not_exist_id)
                        .await
                        .expect("failed to check whether class id exists")
                };

                assert!(!res);
            }

            // pass_phrase_exists
            {
                let res = db
                    .lock()
                    .await
                    .pass_phrase_exists(&classes[0].pass_phrase)
                    .await
                    .expect("failed to check whether pass phrase exists");
                assert!(res);

                let res = {
                    let not_exist_pass = PassPhrase::new(&db)
                        .await
                        .expect("failed to generate class id");

                    db.lock()
                        .await
                        .pass_phrase_exists(&not_exist_pass)
                        .await
                        .expect("failed to check whether class id exists")
                };
                assert!(!res);
            }

            // rename_class
            {
                db.lock()
                    .await
                    .rename_class(&classes[1].id, "英語")
                    .await
                    .expect("failed to rename class");

                classes[1].name = "英語".into();

                let after0 = db
                    .lock()
                    .await
                    .get_class_by_id(&classes[0].id)
                    .await
                    .expect("failed to get class");

                let after1 = db
                    .lock()
                    .await
                    .get_class_by_id(&classes[1].id)
                    .await
                    .expect("failed to get class");

                assert_eq!(classes[0], after0);
                assert_eq!(classes[1], after1);
            }

            let file_test_class = &mut classes[0];
            let mut files = vec![
                File::new(
                    &db,
                    ArMarkerID("foo_marker".into()),
                    "foo.png".into(),
                    chrono::Utc::now(),
                )
                .await
                .expect("failed to create new file"),
                File::new(
                    &db,
                    ArMarkerID("bar_marker".into()),
                    "bar.png".into(),
                    chrono::Utc::now(),
                )
                .await
                .expect("failed to create new file"),
            ];

            // add_new_file, get_files
            {
                for file in &files {
                    db.lock()
                        .await
                        .add_new_file(&file_test_class.id, file)
                        .await
                        .expect("failed to add new file");

                    file_test_class.files.push(file.clone());
                }

                let res = db
                    .lock()
                    .await
                    .get_class_by_id(&file_test_class.id)
                    .await
                    .expect("failed to get class");

                let res_files = db
                    .lock()
                    .await
                    .get_files(&file_test_class.id)
                    .await
                    .expect("failed to get files");

                for file in &res_files {
                    assert!(res.files.contains(file));
                }

                for file in &files {
                    assert!(res.files.contains(file));
                }
            }

            // file_id_exists
            {
                let res = db
                    .lock()
                    .await
                    .file_id_exists(&files[0].id)
                    .await
                    .expect("failed to check whether file id exists");

                assert!(res);

                let res = {
                    let not_exist_id = FileID::new(&db)
                        .await
                        .expect("failed to create new file id");

                    db.lock()
                        .await
                        .file_id_exists(&not_exist_id)
                        .await
                        .expect("failed to check whether file id exists")
                };

                assert!(!res);
            }

            // get_file_by_id
            {
                let res = db
                    .lock()
                    .await
                    .get_file_by_id(&files[1].id)
                    .await
                    .expect("failed to get file");

                assert_eq!(res, files[1]);
            }

            // delete_file
            {
                let deleted = db
                    .lock()
                    .await
                    .delete_file(&files[0].id)
                    .await
                    .expect("failed to delete file");

                assert_eq!(files[0], deleted);
                files.remove(0);

                let res_files = db
                    .lock()
                    .await
                    .get_files(&file_test_class.id)
                    .await
                    .expect("failed to get files");

                assert_eq!(res_files, files);

                file_test_class.files = res_files;
            }

            // delete_class
            {
                let deleted = db
                    .lock()
                    .await
                    .delete_class(&classes[0].id)
                    .await
                    .expect("failed to delete file");

                assert_eq!(classes[0], deleted);
                classes.remove(0);

                let res_classes = db.lock().await.get_class_by_id(&deleted.id).await;
                assert_eq!(res_classes, Err(DatabaseError::ClassNotFound));

                let res_classes = db
                    .lock()
                    .await
                    .get_class_by_id(&classes[0].id)
                    .await
                    .expect("expected to not deleted this one");
            }
        }

        Builder::new()
            .enable_all()
            .threaded_scheduler()
            .build()
            .unwrap()
            .block_on(test());
    }
}
