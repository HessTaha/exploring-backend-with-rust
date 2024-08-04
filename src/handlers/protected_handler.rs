use axum::http::{header::CONTENT_TYPE, HeaderMap, Response, StatusCode};
use serde_json::json;

use crate::auth::jwt_auth::decode_jwt;

pub async fn protected_route(header: HeaderMap) -> Response<String> {
    let header_token = header
        .get("Authorization")
        .unwrap()
        .to_str()
        .unwrap()
        .to_owned();

    let token = header_token.split(" ").nth(1).unwrap();
    let user = decode_jwt(&token);

    match user {
        Ok(user_claim) => Response::builder()
            .status(StatusCode::OK)
            .header(CONTENT_TYPE, "application/json")
            .body(
                json!({
                    "success": true,
                    "data":{
                            "message": format!("Hello uuser {:?}", user_claim.email)
                    }
                } )
                .to_string(),
            )
            .unwrap_or_default(),
        Err(e) => Response::builder()
            .status(StatusCode::UNAUTHORIZED)
            .header(CONTENT_TYPE, "application/json")
            .body(
                json!({
                    "sucess": false,
                    "data": {
                        "message" : format!("Error : {}", e.to_string())
                    }
                })
                .to_string(),
            )
            .unwrap_or_default(),
    }
}
