use axum::extract::Path;
use axum::http::{header, HeaderMap, HeaderValue};
use axum::response::IntoResponse;
use axum::routing::get;
use axum::Router;
use lib_web::{get_file_data, get_index_data};

async fn web_route(request_headers: HeaderMap, path: Option<Path<String>>) -> impl IntoResponse {
    let path_string = match path {
        Some(Path(path)) => path,
        None => "index.html".to_string(),
    };

    let file_data = get_file_data(path_string.as_str()).unwrap_or(get_index_data());

    let mut headers = HeaderMap::new();
    if !file_data.mime_type.is_empty() {
        headers.insert(
            header::CONTENT_TYPE,
            HeaderValue::from_static(file_data.mime_type),
        );
    }

    let mut use_compressed_data = false;
    if let Some(encoding_header) = request_headers.get("Accept-Encoding") {
        if let Ok(s) = encoding_header.to_str() {
            use_compressed_data = s.contains("gzip");
        }
    }

    if use_compressed_data {
        headers.insert(header::CONTENT_ENCODING, HeaderValue::from_static("gzip"));

        (headers, file_data.data_gzip).into_response()
    } else {
        (headers, file_data.data_uncompressed).into_response()
    }
}

pub fn web_router() -> Router {
    Router::new()
        .route("/*path", get(web_route))
        .route("/", get(web_route))
}
