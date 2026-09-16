use axum::http::HeaderValue;
use tower_http::cors::{Any, CorsLayer};

pub fn cors_allow() -> CorsLayer {
    CorsLayer::new()
        .allow_origin(HeaderValue::from_static("http://localhost:5176"))
        .allow_methods(Any)
        .allow_headers(Any)
}
