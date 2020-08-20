use super::{with_db, with_json_body, ApiDBError, IDParsingError};
use crate::db::Database;
use crate::model::ClassID;
use crate::Synced;
use serde::Deserialize;
use std::str::FromStr;
use std::sync::Arc;
use warp::Filter;

pub(super) fn class(
    db: &Synced<impl Database>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    get(Arc::clone(db)).or(put(Arc::clone(db)))
}

fn get(
    db: Synced<impl Database>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("classes" / String)
        .and(warp::get())
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

fn put(
    db: Synced<impl Database>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("classes" / String)
        .and(warp::put())
        .and(with_db(db))
        .and(with_json_body())
        .and_then(on_put)
}

#[derive(Deserialize)]
struct PutRequestBody {
    name: String,
}

async fn on_put(
    raw_id: String,
    db: Synced<impl Database>,
    body: PutRequestBody,
) -> Result<impl warp::Reply, warp::Rejection> {
    let id = ClassID::from_str(raw_id.as_str())
        .map_err(IDParsingError)
        .map_err(warp::reject::custom)?;

    db.lock()
        .await
        .rename_class(&id, body.name.as_str())
        .await
        .map_err(ApiDBError)
        .map_err(warp::reject::custom)?;

    Ok(warp::http::StatusCode::NO_CONTENT)
}
