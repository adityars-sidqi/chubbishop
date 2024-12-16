use bigdecimal::BigDecimal;
use sqlx::{Error, Pool, Postgres, QueryBuilder};
use sqlx::types::chrono::Utc;
use uuid::Uuid;
use crate::domain::product::{Product, ProductWithReviews, Review};
use crate::services::category_service;
use crate::services::service::{CategoryService, ProductService};

pub struct ProductServiceImpl;

impl ProductService for ProductServiceImpl {
    async fn fetch_all(&self, pool: &Pool<Postgres>) -> Result<Vec<Product>, Error> {
        let products = sqlx::query_as!(Product, "SELECT products.id, products.name, description, price, stock, \
        products.created_at, products.updated_at, categories.name as category_name \
        FROM products INNER JOIN categories ON categories.id = products.category_id")
            .fetch_all(pool)
            .await?;
        Ok(products)
    }

    async fn fetch_by_id(&self, id: Uuid, pool: &Pool<Postgres>) -> Result<Product, Error> {
        let product = sqlx::query_as!(Product, "SELECT products.id, products.name, description, price, stock, \
        products.created_at, products.updated_at, categories.name as category_name \
        FROM products INNER JOIN categories ON categories.id = products.category_id WHERE products.id = $1", id)
            .fetch_one(pool)
            .await?;
        Ok(product)
    }

    async fn fetch_by_id_with_reviews(&self, id: Uuid, pool: &Pool<Postgres>) -> Result<ProductWithReviews, Error> {

        let rows = sqlx::query!(r#"
                SELECT products.id AS product_id, products.name, description, price, stock, products.created_at, products.updated_at,
                categories.name as category_name,
                product_reviews.product_id as review_product_id, product_reviews.user_id, product_reviews.comment, product_reviews.rating, product_reviews.created_at as review_created_at
                FROM products
                LEFT JOIN categories ON products.category_id = categories.id
                LEFT JOIN product_reviews  ON products.id = product_reviews.product_id
                WHERE products.id = $1"#, id)
            .fetch_all(pool)
            .await?;

        // Memastikan ada hasil produk
        if rows.is_empty() {
            return Err(Error::RowNotFound); // Atau penanganan kesalahan lain yang sesuai
        }

        // Memetakan hasil ke dalam ProductWithReviews
        let mut product = ProductWithReviews {
            id: rows[0].product_id, // Ambil dari baris pertama
            name: rows[0].name.clone(),
            description: rows[0].description.clone(),
            price: rows[0].price.clone(),
            stock: rows[0].stock,
            category_name: rows[0].category_name.clone(),
            reviews: None, // Inisialisasi sebagai None
            created_at: rows[0].created_at,
            updated_at: rows[0].updated_at,
        };
        // Mengumpulkan ulasan
        let mut reviews: Vec<Review> = Vec::new();
        for row in rows {
            if let Some(user_id) = row.user_id {
                reviews.push(Review {
                    user_id: row.user_id,
                    comment: row.comment.clone(),
                    rating: row.rating,
                    created_at: Some(row.review_created_at),
                });
            }
        }

        // Jika ada ulasan, set ke dalam product
        if !reviews.is_empty() {
            product.reviews = Some(reviews);
        }

        Ok(product)
    }

    async fn save(&self, name: String, description: Option<String>, price: BigDecimal, stock: i32, category_id: Uuid, pool: &Pool<Postgres>) -> Result<Product, Error> {
        let category = category_service::CategoryServiceImpl.fetch_by_id(category_id, pool).await?;

        let now = Utc::now().timestamp_millis();
        let product = Product {
            id: Uuid::new_v4(),
            name,
            description,
            price,
            stock,
            category_name: category.name,
            created_at: now,
            updated_at: now,
        };

        sqlx::query!(
        "INSERT INTO products (id, name, description, price, stock, category_id, created_at, updated_at, version) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)",
            product.id,
            product.name,
            product.description,
            product.price,
            product.stock,
            category.id,
            product.created_at,
            product.updated_at,
            0
        ).execute(pool).await?;

        Ok(product)
    }

    async fn update(&self, id: Uuid, name: Option<String>, description: Option<String>, price: Option<BigDecimal>, stock: Option<i32>, pool: &Pool<Postgres>) -> Result<Product, Error> {
        let mut product = self.fetch_by_id(id, pool).await?;


        let mut query_builder = QueryBuilder::<Postgres>::new("UPDATE products SET ");
        let mut first = true;
        if let Some(name) = &name {
            if !first { query_builder.push(", ");}
            query_builder.push("name = ");
            query_builder.push_bind(name);
            product.name = name.clone();
            first = false;
        }
        if let Some(description) = &description {
            if !first { query_builder.push(", ");}
            query_builder.push("description = ");
            query_builder.push_bind(description);
            product.description = Some(description.clone());
            first = false;
        }
        if let Some(price) = &price {
            if !first { query_builder.push(", ");}
            query_builder.push("price = ");
            query_builder.push_bind(price);
            product.price = price.clone();
            first = false;
        }
        if let Some(stock) = &stock {
            if !first { query_builder.push(", ");}
            query_builder.push("stock = ");
            query_builder.push_bind(stock);
            product.stock = stock.clone();
            first = false;
        }

        if first {
            // Tidak ada data yang diupdate
            return Ok(product);
        }

        query_builder.push(" WHERE id = ");
        query_builder.push_bind(id);

        let query = query_builder.build();

        // Melakukan update di database
        query.execute(pool).await?;
        Ok(product)
    }

    async fn delete(&self, id: Uuid, pool: &Pool<Postgres>) -> Result<(), Error> {
        let delete = sqlx::query!("DELETE FROM products WHERE id = $1", id)
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