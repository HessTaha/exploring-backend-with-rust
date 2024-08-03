use axum::{
    extract::State,
    http::{header::CONTENT_TYPE, StatusCode},
    response::Response,
    Json,
};
use serde_json::json;
use sqlx::query;

use crate::models::users::UserData;
use crate::AppState;

pub async fn get_users_data(
    State(state): State<AppState>,
    Json(data): Json<UserData>,
) -> Response<String> {
    let query_result = query!("SELECT * FROM users WHERE name = $1", data.name)
        .fetch_one(&state.db)
        .await
        .expect("Cannot find data");

    let user = UserData {
        name: query_result.name.unwrap(),
        surname: query_result.surname.unwrap(),
        id: query_result.id.to_string(),
        age: query_result.age.expect("Missing age"),
        profession: query_result.profession.unwrap(),
    };
    let response = Response::builder()
        .status(StatusCode::OK)
        .header(CONTENT_TYPE, "appplication/json")
        .body(json!(user).to_string())
        .unwrap_or_default();

    response
}
