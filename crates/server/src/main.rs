use api_router::api_router;
use axum::response::Redirect;
use axum::Router;
use tokio::net::TcpListener;
use web_router::web_router;

#[tokio::main]
async fn main() {
    // build our application with a single route
    let router = Router::new()
        .nest_service("/api", api_router())
        .nest_service("/web", web_router())
        .fallback(|| async { Redirect::temporary("/web") });

    // run it with hyper on localhost:3000
    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, router).await.unwrap();
}
