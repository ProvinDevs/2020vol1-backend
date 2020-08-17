use warp::Filter;

pub async fn serve() {
    let routes = warp::any().map(|| "UNCHI");

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
