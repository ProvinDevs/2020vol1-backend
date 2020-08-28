mod api;
mod db;
mod model;

use crate::db::mem::MemoryDB;
use crate::db::mongo::MongoDB;
use std::env;
use std::sync::Arc;
use tokio::sync::Mutex;

type Synced<D> = Arc<Mutex<D>>;

#[tokio::main]
async fn main() {
    setup_logger();

    let port = get_port();

    match env::var("DATABASE").ok() {
        Some(db_name) => match db_name.as_str() {
            "memory" => use_memory_db(port).await,
            "mongo" => use_mongo_db(port).await,

            _ => panic!("Set DATABASE env var to \"memory\" or \"mongo\""),
        },

        None => {
            log::warn!(
                "DATABASE env var not set. fallbacking to memory DB, data lost occurs on restart!"
            );

            use_memory_db(port).await
        }
    }
}

async fn use_memory_db(port: u16) {
    let db = Arc::new(Mutex::new(MemoryDB::new()));
    api::serve(port, db).await;
}

async fn use_mongo_db(port: u16) {
    let url = env::var("MONGO_URL").expect("Set MONGO_URL to MongoDB URL");
    let db = MongoDB::new(&url).await.expect("Failed to connect MongoDB");
    let db = Arc::new(Mutex::new(db));

    api::serve(port, db).await;
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
