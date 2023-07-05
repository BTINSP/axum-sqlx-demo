use sqlx::{Postgres, Pool};


#[derive(Clone)]
pub struct AppState {
    pub pool: Pool<Postgres>
}