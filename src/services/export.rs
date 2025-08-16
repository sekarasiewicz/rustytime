// src/services/export.rs
use serde::Serialize;
use sqlx::SqlitePool;

#[derive(Serialize)]
struct Entry {
    id: String,
    task_id: String,
    start_time: String,
    end_time: Option<String>,
    duration_seconds: Option<i64>,
    local_date: String,
}

pub async fn export_json(
    pool: &SqlitePool,
    out: &str,
    from: Option<&str>,
    to: Option<&str>,
) -> anyhow::Result<()> {
    let rows = sqlx::query!(
        r#"SELECT id, task_id, start_time, end_time, duration_seconds, local_date
       FROM time_entries
       WHERE (? IS NULL OR local_date >= ?)
         AND (? IS NULL OR local_date <= ?)
       ORDER BY start_time"#,
        from,
        from,
        to,
        to
    )
    .fetch_all(pool)
    .await?;

    let entries: Vec<Entry> = rows
        .into_iter()
        .map(|row| Entry {
            id: row.id.unwrap(),
            task_id: row.task_id,
            start_time: row.start_time,
            end_time: row.end_time,
            duration_seconds: row.duration_seconds,
            local_date: row.local_date,
        })
        .collect();
    std::fs::write(out, serde_json::to_vec_pretty(&entries)?)?;
    Ok(())
}
