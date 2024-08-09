use axum::{extract::State, http::StatusCode, response::Response, Json};
use sqlx::query_as;

use crate::auth::jwt_auth::get_jwt;
use crate::handlers::utils::make_response;
use crate::models::users::LoginData;
use crate::AppState;

pub async fn login_handler(
    State(state): State<AppState>,
    Json(data): Json<LoginData>,
) -> Response<String> {
    let query_result = query_as!(
        LoginData,
        "select email from users where email=$1",
        data.email
    )
    .fetch_one(&state.db)
    .await;

    match query_result {
        Ok(user) => {
            let token = get_jwt(user);
            make_response(
                StatusCode::OK,
                token.unwrap(),
                "application/json".to_string(),
            )
        }
        Err(error) => make_response(
            StatusCode::NOT_FOUND,
            format!("Error : {}", error.to_string()),
            "application/json".to_string(),
        ),
    }
}
