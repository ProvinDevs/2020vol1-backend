mod api;
mod model;

#[tokio::main]
async fn main() {
    api::serve().await;
}
