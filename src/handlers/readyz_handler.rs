use crate::handlers::utils::make_response;
use axum::{http::StatusCode, response::Response};

pub async fn readyz_handler() -> Response<String> {
    make_response(
        StatusCode::OK,
        "service is up".to_string(),
        "application/json".to_string(),
    )
}
