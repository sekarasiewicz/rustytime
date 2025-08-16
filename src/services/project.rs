// src/services/project.rs
use crate::{models::Project, services::timeutil::*};
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

pub async fn list(pool: &SqlitePool) -> anyhow::Result<Vec<Project>> {
    let rows = sqlx::query!("SELECT id, name, description, archived, created_at FROM projects")
        .fetch_all(pool)
        .await?;
    Ok(rows
        .into_iter()
        .map(|r| Project {
            id: r.id.unwrap(),
            name: r.name,
            description: r.description,
            archived: r.archived == 1,
            created_at: r.created_at,
        })
        .collect())
}
