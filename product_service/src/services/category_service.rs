use sqlx::{Error, Pool, Postgres};
use sqlx::types::chrono::Utc;
use uuid::Uuid;
use crate::domain::category::{Category};
use crate::services::service::CategoryService;

pub struct CategoryServiceImpl;


impl CategoryService for CategoryServiceImpl {
    async fn fetch_categories(&self, pool: &Pool<Postgres>) -> Result<Vec<Category>, Error> {
        let categories = sqlx::query_as!(Category, "SELECT id, name, created_at, updated_at FROM categories")
            .fetch_all(pool)
            .await?;
        Ok(categories)
    }

    async fn fetch_by_id(&self, id: Uuid, pool: &Pool<Postgres>) -> Result<Category, Error> {
        let category = sqlx::query_as!(Category, "SELECT id, name, created_at, updated_at FROM categories WHERE id = $1", id)
            .fetch_one(pool)
            .await?;
        Ok(category)
    }

    async fn save(&self, name: String, pool: &Pool<Postgres>) -> Result<Category, Error> {
        let now = Utc::now().timestamp_millis();
        let category = Category {
            id: Uuid::new_v4(),
            name,
            created_at: now,
            updated_at: now,
        };

        sqlx::query!(
        "INSERT INTO categories (id, name, created_at, updated_at, version) VALUES ($1, $2, $3, $4, $5)",
        category.id,
        category.name,
        category.created_at,
        category.updated_at,
            0
        )
            .execute(pool)
            .await?;

        Ok(category)
    }

    async fn update(&self, id: Uuid, name: String, pool: &Pool<Postgres>) -> Result<Category, Error> {
        let mut category = sqlx::query_as!(Category, "SELECT id, name, created_at, updated_at FROM categories WHERE id = $1", id)
            .fetch_one(pool)
            .await?;

        category.name = name;
        category.updated_at = Utc::now().timestamp_millis();

        // Melakukan update di database
        sqlx::query!(
            "UPDATE categories SET name = $1, updated_at = $2 WHERE id = $3",
            category.name,
            category.updated_at,
            id
        )
            .execute(pool)
            .await?;

        Ok(category)
    }

    async fn delete(&self, id: Uuid, pool: &Pool<Postgres>) -> Result<(), Error> {
        let delete = sqlx::query!("DELETE FROM categories WHERE id = $1", id)
            .execute(pool)
            .await;

        match delete.unwrap_or_default().rows_affected() {
            1 => Ok(()),
            _ => {
                Err(Error::RowNotFound)
            }
        }
    }
}