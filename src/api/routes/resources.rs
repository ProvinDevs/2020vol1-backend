use super::{with_db, with_json_body, ApiDBError, IDParsingError};
use crate::db::Database;
use crate::model::Class;
use crate::model::ClassID;
use crate::Synced;
use std::str::FromStr;
use std::sync::Arc;
use warp::Filter;

pub(super) fn resources(
    db: &Synced<impl Database>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    get(Arc::clone(db))
}

fn get(
    db: Synced<impl Database>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("classes" / String / "resources")
        .and(warp::get())
        .and(with_db(db))
        .and_then(on_get)
}

async fn on_get(
    id: String,
    db: Synced<impl Database>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let id = ClassID::from_str(id.as_str())
        .map_err(IDParsingError)
        .map_err(warp::reject::custom)?;

    let resources = db
        .lock()
        .await
        .get_files(&id)
        .await
        .map_err(ApiDBError)
        .map_err(warp::reject::custom)?;

    Ok(warp::reply::json(&resources))
}
