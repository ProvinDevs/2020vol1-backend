use super::{with_db, with_json_body, ApiDBError, IDParsingError};
use crate::db::Database;
use crate::model::{ArMarkerID, ClassID, File};
use crate::Synced;
use chrono::{TimeZone, Utc};
use serde::Deserialize;
use std::str::FromStr;
use std::sync::Arc;
use warp::Filter;

pub(super) fn resources(
    db: &Synced<impl Database>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    get(Arc::clone(db)).or(post(Arc::clone(db)))
}

fn get(
    db: Synced<impl Database>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("classes" / String / "files")
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

fn post(
    db: Synced<impl Database>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("classes" / String / "files")
        .and(warp::post())
        .and(with_db(db))
        .and(with_json_body())
        .and_then(on_post)
}

#[derive(Deserialize)]
struct PostRequestBody {
    #[serde(rename = "markerID")]
    marker_id: String,
    #[serde(rename = "resourceInfo")]
    resource_info: ResourceRequestBody,
}

#[derive(Deserialize)]
struct ResourceRequestBody {
    #[serde(rename = "fileName")]
    file_name: String,
    #[serde(rename = "createdAt")]
    created_at: i64,
}

async fn on_post(
    raw_id: String,
    db: Synced<impl Database>,
    body: PostRequestBody,
) -> Result<impl warp::Reply, warp::Rejection> {
    let class_id = ClassID::from_str(raw_id.as_str())
        .map_err(IDParsingError)
        .map_err(warp::reject::custom)?;

    let marker_id = ArMarkerID(body.marker_id);
    let created_at = Utc.timestamp(body.resource_info.created_at, 0);

    let file = File::new(&db, marker_id, body.resource_info.file_name, created_at)
        .await
        .map_err(ApiDBError)
        .map_err(warp::reject::custom)?;

    db.lock()
        .await
        .add_new_file(&class_id, &file)
        .await
        .map_err(ApiDBError)
        .map_err(warp::reject::custom)?;

    Ok(warp::reply::json(&file))
}
