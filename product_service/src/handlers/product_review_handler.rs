use std::sync::Arc;
use axum::{Extension, Json, Router};
use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{ post};
use uuid::Uuid;
use common::response::{BaseApiResponse, ErrorDetails};
use crate::{services, AppState};
use crate::domain::product::{CreateProductReview, ProductReview};
use crate::services::service::ProductReviewService;

pub fn routes() -> Router {
    Router::new()
        .route("/:id/review", post(product_add_review))

}

async fn product_add_review(Extension(state): Extension<Arc<AppState>>, Path(id): Path<Uuid>, Json(request): Json<CreateProductReview>) -> impl IntoResponse {
    let pool = &state.pg_pool;

    match services::product_review_service::ProductReviewServiceImpl.save(id, request.user_id, request.comment, request.rating,  pool).await {
        Ok(product_review) => {
            let response = BaseApiResponse::<ProductReview, ErrorDetails>::new(
                "success",
                "Product review created successfully!",
                Some(product_review),
                None
            );
            response.with_status_code(StatusCode::CREATED)
        }
        Err(error) => {
            let error_details = ErrorDetails {
                code: "BAD_REQUEST".to_string(),
                message: error.to_string(),
            };
            let response = BaseApiResponse::<ProductReview, ErrorDetails>::new(
                "error",
                "Failed to create product review",
                None,
                Some(error_details),
            );
            response.with_status_code(StatusCode::BAD_REQUEST)
        }
    }
}