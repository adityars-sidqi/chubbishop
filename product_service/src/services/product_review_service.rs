use sqlx::{Error, Pool, Postgres};
use sqlx::types::chrono::Utc;
use uuid::Uuid;
use crate::domain::product::ProductReview;
use crate::services::product_service;
use crate::services::service::{ProductReviewService, ProductService};

pub struct ProductReviewServiceImpl;

impl ProductReviewService for ProductReviewServiceImpl {
    async fn save(&self, product_id: Uuid, user_id: Uuid, comment: Option<String>, rating: Option<i32>, pool: &Pool<Postgres>) -> Result<ProductReview, Error> {
        let product = product_service::ProductServiceImpl.fetch_by_id(product_id, pool).await?;

        let now = Utc::now().timestamp_millis();
        let product_review = ProductReview {
            id: Some(Uuid::new_v4()),
            product_id: Some(product.id),
            user_id: Some(user_id),
            comment,
            rating,
            created_at: Some(now),
            updated_at: Some(now),
        };

        sqlx::query!(
        "INSERT INTO product_reviews (id, user_id, comment, rating, product_id, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, $6, $7)",
            product_review.id,
            product_review.user_id,
            product_review.comment,
            product_review.rating,
            product_review.product_id,
            product_review.created_at,
            product_review.updated_at
        ).execute(pool).await?;

        Ok(product_review)
    }
}