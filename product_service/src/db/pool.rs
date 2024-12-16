use sqlx::{PgPool, Pool, Postgres};

pub async fn create_pool(database_url: &str)
                         -> Pool<Postgres> {

   PgPool::connect(database_url).await.unwrap()

}