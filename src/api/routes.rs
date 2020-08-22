mod by_pass;
mod class;
mod classes;
mod resource;
mod resources;

use super::CONTENT_LENGTH_LIMIT;
use crate::db::{Database, DatabaseError};
use crate::Synced;
use serde::de::DeserializeOwned;
use std::sync::Arc;
use warp::Filter;

macro_rules! warp_err {
    ( $(struct $struct_name:ident($from:ty);)* ) => {
        $(
            #[derive(Debug)]
            pub(super) struct $struct_name(pub(super) $from);
            impl warp::reject::Reject for $struct_name {}
        )*
    };
}

warp_err! {
    struct ApiDBError(DatabaseError);
    struct IDParsingError(uuid::Error);
}

// returns filter that combined all filters in child modules.
pub(super) fn routes(
    db: Synced<impl Database>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    classes::classes(&db)
        .or(class::class(&db))
        .or(resources::resources(&db))
        .or(resource::resource(&db))
        .or(by_pass::by_pass(&db))
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
