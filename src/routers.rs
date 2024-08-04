use axum::{
    routing::{get, post},
    Router,
};

use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::handlers::{
    login_handler::login_handler, protected_handler::protected_route,
    readyz_handler::readyz_handler, score_handler::score_handler, users_handler::get_users_data,
};
use crate::AppState;

pub fn init_router(app_state: AppState) -> Router {
    //TODO understand better tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                "my-first-api-project=debug,tower_http=debug,axum::rejection=trace".into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
    Router::new()
        .route("/readyz", get(readyz_handler))
        .route("/score", post(score_handler))
        .route("/users", post(get_users_data))
        .route("/login", post(login_handler))
        .route("/protected", post(protected_route))
        .with_state(app_state)
        .layer(TraceLayer::new_for_http())
}
