use std::sync::Arc;
use axum::{Extension, Json, Router};
use axum::extract::{Path, Query};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{get};
use serde::Deserialize;
use uuid::Uuid;
use common::response::{BaseApiResponse, ErrorDetails};
use crate::{services, AppState};
use crate::domain::product::{CreateProduct, CreateProductReview, Product, ProductWithReviews, UpdateProduct};
use crate::services::service::ProductService;

pub fn routes() -> Router {
    Router::new()
        .route("/", get(get_all).post(create))
        .route("/:id", get(get_by_id).put(update_data).delete(delete_data))

}

#[derive(Debug, Deserialize)]
struct WithReviews {
    with_reviews: Option<bool>, // Parameter query untuk menentukan apakah ulasan disertakan
}

async fn get_all(Extension(state): Extension<Arc<AppState>>) -> impl IntoResponse {
    let pool = &state.pg_pool;

    match services::product_service::ProductServiceImpl.fetch_all(pool).await {
        Ok(products) => {
            let response = BaseApiResponse::<Vec<Product>, ErrorDetails>::new(
                "success",
                "Products retrieved successfully!",
                Some(products),
                None
            );
            response.with_status_code(StatusCode::OK)
        }
        Err(error) => {
            let error_details = ErrorDetails {
                code: "BAD_REQUEST".to_string(),
                message: error.to_string(),
            };
            let response = BaseApiResponse::<Vec<Product>, ErrorDetails>::new(
                "error",
                "Failed to fetch products",
                None,
                Some(error_details),
            );
            response.with_status_code(StatusCode::BAD_REQUEST)
        }
    }
}

async fn get_by_id(Extension(state): Extension<Arc<AppState>>, Path(id): Path<Uuid>, Query(with_reviews): Query<WithReviews>) -> impl IntoResponse {
    let pool = &state.pg_pool;

    match with_reviews.with_reviews {
        Some(true) => {
            match services::product_service::ProductServiceImpl.fetch_by_id_with_reviews(id, pool).await {
                Ok(product) => {
                    let response = BaseApiResponse::<ProductWithReviews, ErrorDetails>::new(
                        "success",
                        "Product retrieved successfully!",
                        Some(product),
                        None
                    );
                    response.with_status_code(StatusCode::OK)
                }
                Err(error) => {
                    let error_details = ErrorDetails {
                        code: "NOT_FOUND".to_string(),
                        message: error.to_string(),
                    };
                    let response = BaseApiResponse::<ProductWithReviews, ErrorDetails>::new(
                        "error",
                        "Failed to fetch product",
                        None,
                        Some(error_details),
                    );
                    response.with_status_code(StatusCode::NOT_FOUND)
                }
            }
        }
        None | Some(false) => {
            match services::product_service::ProductServiceImpl.fetch_by_id(id, pool).await {
                Ok(product) => {
                    let response = BaseApiResponse::<Product, ErrorDetails>::new(
                        "success",
                        "Product retrieved successfully!",
                        Some(product),
                        None
                    );
                    response.with_status_code(StatusCode::OK)
                }
                Err(error) => {
                    let error_details = ErrorDetails {
                        code: "NOT_FOUND".to_string(),
                        message: error.to_string(),
                    };
                    let response = BaseApiResponse::<Product, ErrorDetails>::new(
                        "error",
                        "Failed to fetch product",
                        None,
                        Some(error_details),
                    );
                    response.with_status_code(StatusCode::NOT_FOUND)
                }
            }
        }
    }

}

async fn create(Extension(state): Extension<Arc<AppState>>, Json(request): Json<CreateProduct>) -> impl IntoResponse {
    let pool = &state.pg_pool;

    match services::product_service::ProductServiceImpl.save(request.name, request.description, request.price, request.stock, request.category_id, pool).await {
        Ok(product) => {
            let response = BaseApiResponse::<Product, ErrorDetails>::new(
                "success",
                "Product created successfully!",
                Some(product),
                None
            );
            response.with_status_code(StatusCode::CREATED)
        }
        Err(error) => {
            let error_details = ErrorDetails {
                code: "BAD_REQUEST".to_string(),
                message: error.to_string(),
            };
            let response = BaseApiResponse::<Product, ErrorDetails>::new(
                "error",
                "Failed to create product",
                None,
                Some(error_details),
            );
            response.with_status_code(StatusCode::BAD_REQUEST)
        }
    }
}

async fn update_data(Extension(state): Extension<Arc<AppState>>, Path(id): Path<Uuid>, Json(request): Json<UpdateProduct>) -> impl IntoResponse {
    let pool = &state.pg_pool;

    match services::product_service::ProductServiceImpl.update(id, request.name, request.description, request.price, request.stock, pool).await {
        Ok(product) => {
            let response = BaseApiResponse::<Product, ErrorDetails>::new(
                "success",
                "Product updated successfully!",
                Some(product),
                None
            );
            response.with_status_code(StatusCode::OK)
        }
        Err(error) => {
            let error_details = ErrorDetails {
                code: "BAD_REQUEST".to_string(),
                message: error.to_string(),
            };
            let response = BaseApiResponse::<Product, ErrorDetails>::new(
                "error",
                "Failed to update product",
                None,
                Some(error_details),
            );
            response.with_status_code(StatusCode::BAD_REQUEST)
        }
    }
}

async fn delete_data(Extension(state): Extension<Arc<AppState>>, Path(id): Path<Uuid>) -> impl IntoResponse {
    let pool = &state.pg_pool;
    let result = services::product_service::ProductServiceImpl.delete(id, pool).await;

    match result {
        Ok(()) => {
            let response = BaseApiResponse::<Product,ErrorDetails>::new(
                "success",
                "Product deleted successfully!",
                None,
                None
            );
            response.with_status_code(StatusCode::OK)
        }
        Err(error) => {
            let error_details = ErrorDetails {
                code: "NOT_FOUND".to_string(),
                message: error.to_string(),
            };
            let response = BaseApiResponse::<Product, ErrorDetails>::new(
                "error",
                "Failed to delete product",
                None,
                Some(error_details),
            );
            response.with_status_code(StatusCode::NOT_FOUND)
        }
    }
}
