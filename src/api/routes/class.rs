use super::{with_db, ApiDBError, IDParsingError};
use crate::db::Database;
use crate::model::ClassID;
use crate::Synced;
use std::str::FromStr;
use std::sync::Arc;
use warp::Filter;

pub(super) fn class(
    db: &Synced<impl Database>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    get(Arc::clone(db))
}

fn get(
    db: Synced<impl Database>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("classes" / String)
        .and(warp::post())
        .and(with_db(db))
        .and_then(on_get)
}

async fn on_get(
    raw_id: String,
    db: Synced<impl Database>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let id = ClassID::from_str(raw_id.as_str())
        .map_err(IDParsingError)
        .map_err(warp::reject::custom)?;
    let class = db
        .lock()
        .await
        .get_class_by_id(&id)
        .await
        .map_err(ApiDBError)
        .map_err(warp::reject::custom)?;

    Ok(warp::reply::json(&class))
}
