
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Category {
    pub id: Uuid,
    pub name: String,
    pub created_at: i64,  // Epoch time
    pub updated_at: i64,  // Epoch time
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateCategory {
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateCategory {
    pub name: String,
}