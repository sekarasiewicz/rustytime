// src/services/task.rs
use crate::services::timeutil::*;
use sqlx::SqlitePool;
use uuid::Uuid;

#[derive(Debug)]
pub struct Task {
    pub id: String,
    pub project_id: String,
    pub name: String,
    pub description: Option<String>,
    pub archived: bool,
    pub created_at: String,
}

/// Add a new task under a project
pub async fn add(
    pool: &SqlitePool,
    project_id: &str,
    name: &str,
    desc: Option<&str>,
) -> anyhow::Result<String> {
    let id = Uuid::now_v7().to_string();
    let now = to_rfc3339(now_utc());

    sqlx::query!(
        "INSERT INTO tasks (id, project_id, name, description, archived, created_at)
         VALUES (?, ?, ?, ?, 0, ?)",
        id,
        project_id,
        name,
        desc,
        now
    )
    .execute(pool)
    .await?;

    Ok(id)
}

/// List all tasks for a project
pub async fn list(pool: &SqlitePool, project_id: &str) -> anyhow::Result<Vec<Task>> {
    let rows = sqlx::query!(
        r#"SELECT id, project_id, name, description, archived as "archived: bool", created_at
           FROM tasks WHERE project_id = ? ORDER BY created_at"#,
        project_id
    )
    .fetch_all(pool)
    .await?;

    Ok(rows
        .into_iter()
        .map(|r| Task {
            id: r.id,
            project_id: r.project_id,
            name: r.name,
            description: r.description,
            archived: r.archived,
            created_at: r.created_at,
        })
        .collect())
}

/// Edit a task's name/description
pub async fn edit(
    pool: &SqlitePool,
    id: &str,
    name: Option<&str>,
    desc: Option<&str>,
) -> anyhow::Result<()> {
    if name.is_none() && desc.is_none() {
        return Ok(()); // nothing to update
    }

    let mut query = String::from("UPDATE tasks SET ");
    let mut sets = vec![];
    if name.is_some() {
        sets.push("name = ?".to_string());
    }
    if desc.is_some() {
        sets.push("description = ?".to_string());
    }
    query.push_str(&sets.join(", "));
    query.push_str(" WHERE id = ?");

    let mut q = sqlx::query(&query);
    if let Some(n) = name {
        q = q.bind(n);
    }
    if let Some(d) = desc {
        q = q.bind(d);
    }
    q = q.bind(id);

    q.execute(pool).await?;
    Ok(())
}

/// Archive a task
pub async fn archive(pool: &SqlitePool, id: &str) -> anyhow::Result<()> {
    sqlx::query!("UPDATE tasks SET archived = 1 WHERE id = ?", id)
        .execute(pool)
        .await?;
    Ok(())
}

/// Delete a task (cascades to time_entries)
pub async fn delete(pool: &SqlitePool, id: &str) -> anyhow::Result<()> {
    sqlx::query!("DELETE FROM tasks WHERE id = ?", id)
        .execute(pool)
        .await?;
    Ok(())
}
