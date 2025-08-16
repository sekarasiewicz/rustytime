// src/services/project.rs
use crate::services::timeutil::*;
use sqlx::SqlitePool;
use uuid::Uuid;

pub async fn add(pool: &SqlitePool, name: &str, desc: Option<&str>) -> anyhow::Result<String> {
    let id = Uuid::now_v7().to_string();
    let now = to_rfc3339(now_utc());
    sqlx::query("INSERT INTO projects(id,name,description,archived,created_at) VALUES(?,?,?,?,?)")
        .bind(&id)
        .bind(name)
        .bind(desc)
        .bind(0)
        .bind(now)
        .execute(pool)
        .await?;
    Ok(id)
}
