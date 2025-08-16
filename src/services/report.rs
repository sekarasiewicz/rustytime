// src/services/report.rs
use sqlx::SqlitePool;
pub async fn daily(pool: &SqlitePool, date: Option<&str>) -> anyhow::Result<Vec<(String, i64)>> {
    if let Some(d) = date {
        let rows = sqlx::query!("SELECT local_date, SUM(duration_seconds) as total FROM time_entries WHERE end_time IS NOT NULL AND local_date=? GROUP BY local_date", d)
            .fetch_all(pool).await?;
        Ok(rows
            .into_iter()
            .map(|r| (r.local_date, r.total.unwrap_or(0)))
            .collect())
    } else {
        let rows = sqlx::query!("SELECT local_date, SUM(duration_seconds) as total FROM time_entries WHERE end_time IS NOT NULL GROUP BY local_date ORDER BY local_date DESC")
            .fetch_all(pool).await?;
        Ok(rows
            .into_iter()
            .map(|r| (r.local_date, r.total.unwrap_or(0)))
            .collect())
    }
}
