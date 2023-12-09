use axum::http::StatusCode;
use axum::response::IntoResponse;

pub(crate) async fn live() -> impl IntoResponse {
    StatusCode::OK
}

pub(crate) async fn ready() -> impl IntoResponse {
    StatusCode::OK
}
