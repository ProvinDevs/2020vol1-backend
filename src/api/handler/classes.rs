use super::ApiDBError;
use crate::db::Database;
use std::sync::Arc;
use tokio::sync::Mutex;

type Synced<T> = Arc<Mutex<T>>;

pub async fn classes_list<D>(db: Synced<D>) -> Result<impl warp::Reply, warp::Rejection>
where
    D: Database,
{
    let classes = db
        .lock()
        .await
        .get_all_classes()
        .await
        .map_err(ApiDBError)
        .map_err(warp::reject::custom)?;
    Ok(warp::reply::json(&classes))
}
