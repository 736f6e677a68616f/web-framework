use axum::{routing::get, Router};

#[tokio::main]
async fn main(){
    let app = Router::new().route("/", get(handler));
    let addr = "127.0.0.1:8080";
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn handler() -> &'static str {
    "hello world"
}