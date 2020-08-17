use serde::de::DeserializeOwned;
use warp::Filter;

const CONTENT_LENGTH_LIMIT: u64 = 1024 * 16;

fn with_json_body<T>() -> impl Filter<Extract = (T,), Error = warp::Rejection> + Clone
where
    T: DeserializeOwned + Send,
{
    warp::body::content_length_limit(CONTENT_LENGTH_LIMIT).and(warp::body::json())
}
