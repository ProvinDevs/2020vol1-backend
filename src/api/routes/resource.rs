use super::{with_db, ApiDBError, IDParsingError};
use crate::db::Database;
use crate::model::FileID;
use crate::Synced;
use std::str::FromStr;
use std::sync::Arc;
use warp::Filter;

pub(super) fn resource(
    db: &Synced<impl Database>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    get(Arc::clone(db)).or(delete(Arc::clone(db)))
}

fn get(
    db: Synced<impl Database>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("classes" / String / "resources" / String)
        .and(warp::get())
        .and(with_db(db))
        .and_then(on_get)
}

async fn on_get(
    _: String,
    raw_resource_id: String,
    db: Synced<impl Database>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let resource_id = FileID::from_str(raw_resource_id.as_str())
        .map_err(IDParsingError)
        .map_err(warp::reject::custom)?;

    let resource = db
        .lock()
        .await
        .get_file_by_id(&resource_id)
        .await
        .map_err(ApiDBError)
        .map_err(warp::reject::custom)?;

    Ok(warp::reply::json(&resource))
}

fn delete(
    db: Synced<impl Database>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("classes" / String / "resources" / String)
        .and(warp::delete())
        .and(with_db(db))
        .and_then(on_delete)
}

async fn on_delete(
    _: String,
    raw_resource_id: String,
    db: Synced<impl Database>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let resource_id = FileID::from_str(raw_resource_id.as_str())
        .map_err(IDParsingError)
        .map_err(warp::reject::custom)?;

    let resource = db
        .lock()
        .await
        .delete_file(&resource_id)
        .await
        .map_err(ApiDBError)
        .map_err(warp::reject::custom)?;

    Ok(warp::reply::json(&resource))
}
