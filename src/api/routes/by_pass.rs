use crate::api::routes::with_db;
use crate::api::ApiDBError;
use crate::db::Database;
use crate::model::PassPhrase;
use crate::Synced;
use std::sync::Arc;
use warp::Filter;

pub(super) fn by_pass(
    db: &Synced<impl Database>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    get(Arc::clone(db))
}

fn get(
    db: Synced<impl Database>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("class" / "by-pass" / String)
        .and(warp::get())
        .and(with_db(db))
        .and_then(on_get)
}

async fn on_get(
    pass: String,
    db: Synced<impl Database>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let pass = PassPhrase(pass);

    let class = db
        .lock()
        .await
        .get_class_by_pass_phrase(&pass)
        .await
        .map_err(ApiDBError)
        .map_err(warp::reject::custom)?;

    Ok(warp::reply::json(&class))
}
