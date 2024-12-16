use axum::Router;
use crate::handlers;

pub fn create_routes() -> Router {
    Router::new()
        .nest("/categories", handlers::category_handler::routes())
        .nest("/products", handlers::product_handler::routes())
        .nest("/products", handlers::product_review_handler::routes())
}