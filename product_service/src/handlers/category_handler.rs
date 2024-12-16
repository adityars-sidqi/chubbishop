use std::sync::Arc;
use axum::{
    extract::{Extension, Json, Path},
    response::IntoResponse,
    routing::{get,delete},
    http::StatusCode,
    Router,
};

use uuid::Uuid;
use common::response::{BaseApiResponse, ErrorDetails};
use crate::{services, AppState};
use crate::domain::category::{Category, CreateCategory, UpdateCategory};
use crate::services::service::CategoryService;

pub fn routes() -> Router {
    Router::new()
        .route("/", get(get_all).post(create))
        .route("/:id", get(get_by_id).put(update_data))
        .route("/:id", delete(delete_data))

}

async fn get_all(Extension(state): Extension<Arc<AppState>>) -> impl IntoResponse {
    let pool = &state.pg_pool;

    match services::category_service::CategoryServiceImpl.fetch_categories(pool).await {
        Ok(categories) => {
            let response = BaseApiResponse::<Vec<Category>, ErrorDetails>::new(
                "success",
                "Categories retrieved successfully!",
                Some(categories),
                None
            );
            response.with_status_code(StatusCode::OK)
        }
        Err(error) => {
            let error_details = ErrorDetails {
                code: "BAD_REQUEST".to_string(),
                message: error.to_string(),
            };
            let response = BaseApiResponse::<Vec<Category>, ErrorDetails>::new(
                "error",
                "Failed to fetch categories",
                None,
                Some(error_details),
            );
            response.with_status_code(StatusCode::BAD_REQUEST)
        }
    }
}

async fn get_by_id(Extension(state): Extension<Arc<AppState>>, Path(id): Path<Uuid>) -> impl IntoResponse {
    let pool = &state.pg_pool;

    match services::category_service::CategoryServiceImpl.fetch_by_id(id, pool).await {
        Ok(category) => {
            let response = BaseApiResponse::<Category, ErrorDetails>::new(
                "success",
                "Category retrieved successfully!",
                Some(category),
                None
            );
            response.with_status_code(StatusCode::OK)
        }
        Err(error) => {
            let error_details = ErrorDetails {
                code: "NOT_FOUND".to_string(),
                message: error.to_string(),
            };
            let response = BaseApiResponse::<Category, ErrorDetails>::new(
                "error",
                "Failed to fetch category",
                None,
                Some(error_details),
            );
            response.with_status_code(StatusCode::NOT_FOUND)
        }
    }
}

async fn create(Extension(state): Extension<Arc<AppState>>, Json(request): Json<CreateCategory>) -> impl IntoResponse {
    let pool = &state.pg_pool;

    match services::category_service::CategoryServiceImpl.save(request.name, pool).await {
        Ok(category) => {
            let response = BaseApiResponse::<Category, ErrorDetails>::new(
                "success",
                "Category created successfully!",
                Some(category),
                None
            );
            response.with_status_code(StatusCode::CREATED)
        }
        Err(error) => {
            let error_details = ErrorDetails {
                code: "BAD_REQUEST".to_string(),
                message: error.to_string(),
            };
            let response = BaseApiResponse::<Vec<Category>, ErrorDetails>::new(
                "error",
                "Failed to create category",
                None,
                Some(error_details),
            );
            response.with_status_code(StatusCode::BAD_REQUEST)
        }
    }
}

async fn update_data(Extension(state): Extension<Arc<AppState>>, Path(id): Path<Uuid>, Json(request): Json<UpdateCategory>) -> impl IntoResponse {
    let pool = &state.pg_pool;

    match services::category_service::CategoryServiceImpl.update(id, request.name, pool).await {
        Ok(category) => {
            let response = BaseApiResponse::<Category, ErrorDetails>::new(
                "success",
                "Category updated successfully!",
                Some(category),
                None
            );
            response.with_status_code(StatusCode::OK)
        }
        Err(error) => {
            let error_details = ErrorDetails {
                code: "BAD_REQUEST".to_string(),
                message: error.to_string(),
            };
            let response = BaseApiResponse::<Vec<Category>, ErrorDetails>::new(
                "error",
                "Failed to update category",
                None,
                Some(error_details),
            );
            response.with_status_code(StatusCode::BAD_REQUEST)
        }
    }
}

async fn delete_data(Extension(state): Extension<Arc<AppState>>, Path(id): Path<Uuid>) -> impl IntoResponse {
    let pool = &state.pg_pool;
    let result = services::category_service::CategoryServiceImpl.delete(id, pool).await;

    match result {
        Ok(()) => {
            let response = BaseApiResponse::<Category,ErrorDetails>::new(
                "success",
                "Category deleted successfully!",
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
            let response = BaseApiResponse::<Vec<Category>, ErrorDetails>::new(
                "error",
                "Failed to delete category",
                None,
                Some(error_details),
            );
            response.with_status_code(StatusCode::NOT_FOUND)
        }
    }
}