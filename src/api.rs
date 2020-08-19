use crate::db::{Database, DatabaseError};
use crate::Synced;
use routes::ApiDBError;
use warp::Filter;

mod routes;

const CONTENT_LENGTH_LIMIT: u64 = 1024 * 16;

pub async fn serve(port: u16, db: Synced<impl Database>) {
    let route = routes::routes(db)
        .recover(recover_error)
        .with(warp::log("api"));

    warp::serve(route).run(([127, 0, 0, 1], port)).await;
}

// warp::reject::custom()したやつはここで拾わないとwarpがエラー吐く
// それ以外(warpが用意してるやつ)はここで拾わず受け流せばwarpがいい感じにしてくれる
async fn recover_error(err: warp::Rejection) -> Result<impl warp::Reply, warp::Rejection> {
    if let Some(db_err) = err.find::<ApiDBError>() {
        match db_err.0 {
            DatabaseError::ClassNotFound => {
                return Ok(warp::reply::with_status(
                    "Not found such class id",
                    warp::http::StatusCode::NOT_FOUND,
                ))
            }

            DatabaseError::FileNotFound => {
                return Ok(warp::reply::with_status(
                    "Not found such file id",
                    warp::http::StatusCode::NOT_FOUND,
                ))
            }

            _ => {
                log::error!("Database error occur: {:?}", db_err);

                return Ok(warp::reply::with_status(
                    "Internal Server Error (Cannot retrieve data from database)",
                    warp::http::StatusCode::INTERNAL_SERVER_ERROR,
                ));
            }
        };
    }

    Err(err)
}
