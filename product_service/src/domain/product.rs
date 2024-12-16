use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Product {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub price: BigDecimal,
    pub stock: i32,
    pub category_name: String,
    pub created_at: i64,  // Epoch time
    pub updated_at: i64,  // Epoch time
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateProduct {
    pub name: String,
    pub description: Option<String>,
    pub price: BigDecimal,
    pub stock: i32,
    pub category_id: Uuid
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateProduct {
    pub name: Option<String>,
    pub description: Option<String>,
    pub price: Option<BigDecimal>,
    pub stock: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductWithReviews {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub price: BigDecimal,
    pub stock: i32,
    pub category_name: String,
    pub reviews: Option<Vec<Review>>,
    pub created_at: i64,  // Epoch time
    pub updated_at: i64,  // Epoch time
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Review {
    pub user_id: Option<Uuid>,
    pub comment: Option<String>,
    pub rating: Option<i32>,
    pub created_at: Option<i64>
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct ProductReview {
    pub id: Option<Uuid>,
    pub product_id: Option<Uuid>,
    pub user_id: Option<Uuid>,
    pub comment: Option<String>,
    pub rating: Option<i32>,
    pub created_at: Option<i64>,  // Epoch time
    pub updated_at: Option<i64>,  // Epoch time
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateProductReview {
    pub user_id: Uuid,
    pub comment: Option<String>,
    pub rating: Option<i32>,
}