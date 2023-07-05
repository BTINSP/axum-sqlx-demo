use sqlx::{Postgres, Pool};


pub async  fn database_pool_create() -> Pool<Postgres> {
    let database_url = "postgres://postgres:postgres@localhost:5432/database";
    sqlx::postgres::PgPoolOptions::new()
        .max_connections(10)
        .min_connections(5)
        .connect(database_url)
        .await
        .expect("链接超时")
}