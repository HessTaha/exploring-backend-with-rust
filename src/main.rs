mod auth;
mod handlers;
mod models;
mod routers;

use axum;

use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

#[derive(Clone)]
pub struct AppState {
    pub db: Pool<Postgres>,
}

#[tokio::main]
async fn main() {
    let database_url = "postgres://initexample:1234@localhost/test_db".to_string();
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("can't connect to database");
    let router = routers::init_router(AppState { db: pool });

    let listner = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listner, router).await.unwrap();
}
