use axum::Router;

mod blockchains;
mod utils;

#[tokio::main]
async fn main() {
    let app = Router::new();

    let listener = tokio::net::TcpListener::bind("").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
