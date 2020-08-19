use crate::db::Database;
use crate::Synced;
use warp::Filter;

mod routes;

const CONTENT_LENGTH_LIMIT: u64 = 1024 * 16;

pub async fn serve(port: u16, db: Synced<impl Database>) {
    let route = routes::routes(db).with(warp::log("api"));

    warp::serve(route).run(([127, 0, 0, 1], port)).await;
}
