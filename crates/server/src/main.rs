use axum::response::Redirect;
use axum::Router;
use lib_api_router::api_router;
use tokio::net::TcpListener;

#[cfg(not(feature = "debug-web"))]
use lib_web_router::web_router;

#[cfg(not(feature = "debug-web"))]
fn router() -> Router {
    Router::new()
        .nest_service("/api", api_router())
        .nest_service("/web", web_router())
        .fallback(|| async { Redirect::temporary("/web") })
}

#[cfg(feature = "debug-web")]
fn router() -> Router {
    Router::new()
        .nest_service("/api", api_router())
        .fallback(|| async { Redirect::temporary("http://127.0.0.1:4000/web") })
}

#[tokio::main]
async fn main() {
    // run it with hyper on localhost:3000
    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, router()).await.unwrap();
}
