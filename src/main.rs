mod api;
mod db;
mod model;

#[tokio::main]
async fn main() {
    api::serve().await;
}
