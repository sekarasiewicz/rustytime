// src/db.rs
use sqlx::SqlitePool;
pub async fn open_db(db_path: &str) -> anyhow::Result<SqlitePool> {
    let pool = SqlitePool::connect(&format!("sqlite://{}", db_path)).await?;
    sqlx::migrate!().run(&pool).await?;
    Ok(pool)
}
