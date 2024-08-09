use axum::http::{HeaderMap, Response, StatusCode};

use crate::auth::jwt_auth::decode_jwt;
use crate::handlers::utils::make_response;

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
        Ok(user_claim) => make_response(
            StatusCode::OK,
            format!("Hello uuser {:?}", user_claim.email),
            "application/json".to_string(),
        ),

        Err(e) => make_response(
            StatusCode::UNAUTHORIZED,
            format!("Error : {}", e.to_string()),
            "application/json".to_string(),
        ),
    }
}
