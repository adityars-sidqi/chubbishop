use std::sync::Arc;
use axum::Extension;
use dotenvy::dotenv;
use sqlx::{Pool, Postgres};

mod handlers;
mod db;
mod redis;
mod services;
mod routes;
mod domain;


#[derive(Clone)]
pub struct AppState {
    pub pg_pool: Pool<Postgres>,
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    tracing_subscriber::fmt().init();

    let pg_pool = db::pool::create_pool(&std::env::var("DATABASE_URL").unwrap_or_default()).await;
    // let redis_pool = redis::pool::create_redis_pool(&std::env::var("REDIS_URL").unwrap());

    let app_state = Arc::new(AppState { pg_pool });

    let app = routes::create_routes().layer(Extension(app_state));

    // let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    // Server::bind(&addr).serve(app.into_make_service()).await.unwrap();
    axum::serve(listener, app.into_make_service()).await.unwrap();
}
