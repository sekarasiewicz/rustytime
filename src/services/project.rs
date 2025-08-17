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
    let rows = sqlx::query_as!(
        Project,
        r#"
        SELECT
          id as "id!",
          name as "name!",
          description,
          archived as "archived!: bool",
          created_at as "created_at!"
        FROM projects
        ORDER BY created_at
        "#
    )
    .fetch_all(pool)
    .await?;
    Ok(rows)
}

pub async fn archive(pool: &SqlitePool, id: &str) -> anyhow::Result<()> {
    sqlx::query!("UPDATE projects SET archived = 1 WHERE id = ?", id)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn delete(pool: &SqlitePool, id: &str) -> anyhow::Result<()> {
    sqlx::query!("DELETE FROM projects WHERE id=?", id)
        .execute(pool)
        .await?;
    Ok(())
}
