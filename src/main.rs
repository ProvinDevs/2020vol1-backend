mod api;
mod db;
mod model;

use crate::db::mem::MemoryDB;
use crate::db::mongo::MongoDB;
use crate::db::Database;
use crate::model::*;
use chrono::prelude::*;
use std::env;
use std::sync::Arc;
use tokio::sync::Mutex;

type Synced<D> = Arc<Mutex<D>>;

#[tokio::main]
async fn main() {
    // setup_logger();

    // let port = get_port();
    // let db = Arc::new(Mutex::new(MemoryDB::new()));

    // api::serve(port, db).await;

    let db = Arc::new(Mutex::new(
        MongoDB::new("mongodb://localhost").await.unwrap(),
    ));

    let classes = vec![
        Class::new(&db, "理科".to_string()).await.unwrap(),
        Class::new(&db, "社会".to_string()).await.unwrap(),
        Class::new(&db, "体育".to_string()).await.unwrap(),
    ];

    for class in &classes {
        db.lock().await.save_new_class(class).await.unwrap();
    }

    let marker = &["foo", "bar", "baz"]
        .iter()
        .map(|r| String::from(*r))
        .collect::<Vec<_>>();
    let filename = &["ffoo", "fbar", "fbaz"]
        .iter()
        .map(|r| String::from(*r))
        .collect::<Vec<_>>();

    for class in &classes {
        for (m, f) in marker.iter().cloned().zip(filename.iter().cloned()) {
            let file = File::new(&db, ArMarkerID(m), f, Utc::now()).await.unwrap();

            db.lock()
                .await
                .add_new_file(&class.id, &file)
                .await
                .unwrap();
        }
    }
}

fn setup_logger() {
    let dotenv_result = dotenv::dotenv();

    if let None = env::var_os("RUST_LOG") {
        env::set_var("RUST_LOG", "INFO");
    }

    env_logger::init();

    if let Err(e) = dotenv_result {
        if e.not_found() {
            return;
        }

        log::warn!("failed to load .env file: {}", e);
    }
}

fn get_port() -> u16 {
    env::var("PORT")
        .as_ref()
        .map(|e| e.as_str())
        .unwrap_or("3000")
        .parse()
        .expect("\"PORT\" should be valid port number (in range of u16).")
}
