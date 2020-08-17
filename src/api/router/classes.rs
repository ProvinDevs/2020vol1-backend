use super::{with_db, Synced};
use crate::api::handler::classes;
use crate::db::Database;
use std::sync::Arc;
use warp::Filter;

pub fn classes<D>(
    db: Synced<D>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
where
    D: Database, // here?
{
    classes_list(Arc::clone(&db)).or(create_class(Arc::clone(&db)))
}

fn classes_list<D>(
    db: Synced<D>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
where
    D: Database, // here?
{
    warp::path!("classes")
        .and(warp::get())
        .and(with_db(db))
        .and_then(classes::classes_list)
}

fn create_class<D>(
    db: Synced<D>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
where
    D: Database, // here?
{
    warp::path!("classes").and(warp::post())
}
