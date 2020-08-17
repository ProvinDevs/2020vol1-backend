use warp::Filter;

const PORT: u16 = 3030;

pub async fn serve() {
    let routes = warp::any().map(|| "UNCHI");

    warp::serve(routes).run(([127, 0, 0, 1], PORT)).await;
}
