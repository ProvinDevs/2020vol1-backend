use std::sync::{Arc, Mutex};
use warp::Filter;

use crate::db::{Database, DatabaseError};

pub mod classes;

type Synced<T> = Arc<Mutex<T>>;

fn with_db<D>(
    db: Synced<D>,
) -> impl Filter<Extract = (Synced<D>,), Error = std::convert::Infallible> + Clone
where
    D: Database, // here?
{
    warp::any().map(move || Arc::clone(&db))
}
