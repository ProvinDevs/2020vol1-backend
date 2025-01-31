use crate::db::{Database, DatabaseError};
use crate::Synced;
use routes::{ApiDBError, IDParsingError};
use warp::http::Method;
use warp::Filter;

mod routes;

const CONTENT_LENGTH_LIMIT: u64 = 1024 * 16;

pub async fn serve(port: u16, db: Synced<impl Database>) {
    let cors = warp::cors::cors()
        .allow_any_origin()
        .allow_headers(vec!["content-type"])
        .allow_methods(&[
            Method::GET,
            Method::PUT,
            Method::DELETE,
            Method::POST,
            Method::OPTIONS,
        ]);

    let route = routes::routes(db)
        .recover(recover_error)
        .with(warp::log("api"))
        .with(cors);

    warp::serve(route).run(([0, 0, 0, 0], port)).await;
}

// warp::reject::custom()したやつはここで拾わないとwarpがエラー吐く
// それ以外(warpが用意してるやつ)はここで拾わず受け流せばwarpがいい感じにしてくれる
async fn recover_error(err: warp::Rejection) -> Result<impl warp::Reply, warp::Rejection> {
    if let Some(db_err) = err.find::<ApiDBError>() {
        return match db_err.0 {
            DatabaseError::ClassNotFound => Ok(warp::reply::with_status(
                "Not found such class id",
                warp::http::StatusCode::NOT_FOUND,
            )),

            DatabaseError::FileNotFound => Ok(warp::reply::with_status(
                "Not found such file id",
                warp::http::StatusCode::NOT_FOUND,
            )),

            _ => {
                log::error!("Database error occur: {:?}", db_err);

                Ok(warp::reply::with_status(
                    "Internal Server Error (Cannot retrieve data from database)",
                    warp::http::StatusCode::INTERNAL_SERVER_ERROR,
                ))
            }
        };
    }

    if err.find::<IDParsingError>().is_some() {
        return Ok(warp::reply::with_status(
            "Invalid id format",
            warp::http::StatusCode::BAD_REQUEST,
        ));
    }

    Err(err)
}
