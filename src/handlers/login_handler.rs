use axum::{
    extract::State,
    http::{header::CONTENT_TYPE, StatusCode},
    response::Response,
    Json,
};
use serde_json::json;
use sqlx::query_as;

use crate::auth::jwt_auth::get_jwt;
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
            Response::builder()
                .status(StatusCode::OK)
                .header(CONTENT_TYPE, "application/json")
                .body(
                    json!(
                        {
                            "data": {
                                "message": token.unwrap()
                            },
                            "success": true
                        }

                    )
                    .to_string(),
                )
                .unwrap_or_default()
        }

        Err(error) => Response::builder()
            .status(StatusCode::OK)
            .header(CONTENT_TYPE, "application/json")
            .body(
                json!(
                    {
                        "data": {
                            "message": "cannot generate token"
                        },
                        "success": false
                    }

                )
                .to_string(),
            )
            .unwrap_or_default(),
    }
}
