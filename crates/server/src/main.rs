use api_router::api_router;
use axum::Router;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    // build our application with a single route
    let router = Router::new().nest_service("/api", api_router());

    // run it with hyper on localhost:3000
    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, router).await.unwrap();
}
