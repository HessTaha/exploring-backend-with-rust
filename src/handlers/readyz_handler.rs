use axum::{
    http::{header, StatusCode},
    response::Response,
};
use serde_json::json;

pub async fn readyz_handler() -> Response<String> {
    let response = Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "application/json")
        .body(
            json!(
                {
                    "data": {
                        "message": "service is up"
                    },
                    "success": true
                }
            )
            .to_string(),
        )
        .unwrap_or_default();

    response
}
