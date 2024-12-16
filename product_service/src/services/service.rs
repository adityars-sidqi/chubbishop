use bigdecimal::BigDecimal;
use sqlx::{Error, Pool, Postgres};
use uuid::Uuid;
use crate::domain::category::{Category};
use crate::domain::product::{Product, ProductReview, ProductWithReviews};

pub trait CategoryService {
    async fn fetch_categories(&self, pool: &Pool<Postgres>) -> Result<Vec<Category>, Error>;
    async fn fetch_by_id(&self, id: Uuid, pool: &Pool<Postgres>) -> Result<Category, Error>;
    async fn save(&self, name: String, pool: &Pool<Postgres>) -> Result<Category, Error>;
    async fn update(&self, id: Uuid, name: String, pool: &Pool<Postgres>) -> Result<Category, Error>;
    async fn delete(&self, id: Uuid, pool: &Pool<Postgres>) -> Result<(), Error>;
}

pub trait ProductService {
    async fn fetch_all(&self, pool: &Pool<Postgres>) -> Result<Vec<Product>, Error>;
    async fn fetch_by_id(&self, id: Uuid, pool: &Pool<Postgres>) -> Result<Product, Error>;
    async fn fetch_by_id_with_reviews(&self, id: Uuid, pool: &Pool<Postgres>) -> Result<ProductWithReviews, Error>;
    async fn save(&self, name: String, description: Option<String>, price: BigDecimal, stock: i32, category_id: Uuid, pool: &Pool<Postgres>) -> Result<Product, Error>;
    async fn update(&self, id: Uuid, name: Option<String>, description: Option<String>, price: Option<BigDecimal>, stock: Option<i32>, pool: &Pool<Postgres>) -> Result<Product, Error>;
    async fn delete(&self, id: Uuid, pool: &Pool<Postgres>) -> Result<(), Error>;

}

pub trait ProductReviewService {
    async fn save(&self, product_id: Uuid, user_id: Uuid, comment: Option<String>, rating: Option<i32>, pool: &Pool<Postgres>) -> Result<ProductReview, Error>;

}