use crate::init_db_pool;
use anyhow::Result;
use sqlx::PgPool;

pub async fn init_db() -> PgPool {
    init_db_pool().await.unwrap()
}

pub async fn init_db_result() -> Result<PgPool> {
    init_db_pool().await
}
