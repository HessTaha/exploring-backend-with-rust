use axum::{
    http::{header::CONTENT_TYPE, StatusCode},
    response::Response,
};
use serde_json::json;

pub fn make_response(
    status_code: StatusCode,
    message: String,
    content_type: String,
) -> Response<String> {
    let success = match status_code {
        StatusCode::OK => true,
        _ => false,
    };

    let json_template = json!(
        {
            "data": {
                "message": message
            },
            "success": success
        }

    );

    Response::builder()
        .status(status_code)
        .header(CONTENT_TYPE, content_type)
        .body(json_template.to_string())
        .unwrap_or_default()
}
