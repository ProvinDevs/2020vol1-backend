use warp::Filter;

mod handler;
mod router;

const PORT: u16 = 3030;

pub async fn serve() {
    let routes = warp::any().map(|| "UNCHI");

    warp::serve(routes).run(([127, 0, 0, 1], PORT)).await;
}
