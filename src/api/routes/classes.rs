use super::{with_db, ApiDBError};
use crate::db::Database;
use crate::Synced;
use std::sync::Arc;
use warp::Filter;

pub(super) fn classes(
    db: &Synced<impl Database>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    get(Arc::clone(db))
}

fn get(
    db: Synced<impl Database>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("classes")
        .and(warp::get())
        .and(with_db(db))
        .and_then(on_get)
}

async fn on_get(db: Synced<impl Database>) -> Result<impl warp::Reply, warp::Rejection> {
    let classes = db
        .lock()
        .await
        .get_all_classes()
        .await
        .map_err(ApiDBError)
        .map_err(warp::reject::custom)?;

    Ok(warp::reply::json(&classes))
}
