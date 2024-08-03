use axum::{
    http::{header::CONTENT_TYPE, Response, StatusCode},
    Json,
};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize, Deserialize)]
pub struct IndiviualDescription {
    name: String,
    age: i8,
    job: String,
    salary: i16,
}

pub async fn score_handler(Json(description): Json<IndiviualDescription>) -> Response<String> {
    let attractivness = compute_attractiveness(description);
    let message = match attractivness {
        1 => "Hello Sexy".to_string(),
        _ => "Hum, hello ...".to_string(),
    };

    let response = Response::builder()
        .status(StatusCode::OK)
        .header(CONTENT_TYPE, "application/json")
        .body(
            json!({
                "success": true,
                "data": {
                    "message": message
                }
            })
            .to_string(),
        )
        .unwrap_or_default();

    response
}

fn compute_attractiveness(description: IndiviualDescription) -> i8 {
    if description.salary > 10000 {
        1
    } else {
        0
    }
}
