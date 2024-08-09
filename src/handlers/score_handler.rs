use axum::{
    http::{Response, StatusCode},
    Json,
};
use serde::{Deserialize, Serialize};

use crate::handlers::utils::make_response;

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
        _ => "Euh, hello ...".to_string(),
    };

    make_response(StatusCode::OK, message, "application/json".to_string())
}

fn compute_attractiveness(description: IndiviualDescription) -> i8 {
    if description.salary > 10000 {
        1
    } else {
        0
    }
}
