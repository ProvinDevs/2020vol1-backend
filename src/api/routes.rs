mod class;
mod classes;

use super::CONTENT_LENGTH_LIMIT;
use crate::db::{Database, DatabaseError};
use crate::Synced;
use serde::de::DeserializeOwned;
use std::sync::Arc;
use warp::Filter;

#[derive(Debug)]
struct ApiDBError(DatabaseError);
impl warp::reject::Reject for ApiDBError {}

#[derive(Debug)]
struct IDParsingError(uuid::Error);
impl warp::reject::Reject for IDParsingError {}

// returns filter that combined all filters in child modules.
pub(super) fn routes(
    db: Synced<impl Database>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    classes::classes(&db).or(class::class(&db))
}

fn with_json_body<T>() -> impl Filter<Extract = (T,), Error = warp::Rejection> + Clone
where
    T: DeserializeOwned + Send,
{
    warp::body::content_length_limit(CONTENT_LENGTH_LIMIT).and(warp::body::json())
}

fn with_db<D>(
    db: Synced<D>,
) -> impl Filter<Extract = (Synced<D>,), Error = std::convert::Infallible> + Clone
where
    D: Database,
{
    warp::any().map(move || Arc::clone(&db))
}
