use crate::services::timeutil::*;
use sqlx::SqlitePool;
use uuid::Uuid;

pub async fn start(pool: &SqlitePool, task_id: &str) -> anyhow::Result<()> {
    let mut tx = pool.begin().await?;
    // If active, stop it
    if let Some(id) = sqlx::query_scalar::<_, Option<String>>(
        "SELECT time_entry_id FROM active_timer WHERE singleton=1",
    )
    .fetch_one(&mut *tx)
    .await?
    {
        stop_tx(&mut tx, &id).await?;
    }

    let id = Uuid::now_v7().to_string();
    let now = now_utc();
    let date = local_date_warsaw(now);
    let now_s = to_rfc3339(now);

    sqlx::query("INSERT INTO time_entries(id, task_id, start_time, end_time, duration_seconds, local_date, created_at)
                 VALUES(?, ?, ?, NULL, NULL, ?, ?)")
        .bind(&id).bind(task_id).bind(&now_s).bind(&date).bind(&now_s)
        .execute(&mut *tx).await?;

    sqlx::query("UPDATE active_timer SET time_entry_id=? WHERE singleton=1")
        .bind(&id)
        .execute(&mut *tx)
        .await?;

    tx.commit().await?;
    Ok(())
}

pub async fn stop(pool: &SqlitePool) -> anyhow::Result<()> {
    let mut tx = pool.begin().await?;
    let active: Option<String> =
        sqlx::query_scalar("SELECT time_entry_id FROM active_timer WHERE singleton=1")
            .fetch_one(&mut *tx)
            .await?;
    if let Some(id) = active {
        stop_tx(&mut tx, &id).await?;
    }
    tx.commit().await?;
    Ok(())
}

async fn stop_tx<'a>(tx: &mut sqlx::Transaction<'a, sqlx::Sqlite>, id: &str) -> anyhow::Result<()> {
    let start: String = sqlx::query_scalar("SELECT start_time FROM time_entries WHERE id=?")
        .bind(id)
        .fetch_one(&mut **tx)
        .await?;
    let start_t =
        time::OffsetDateTime::parse(&start, &time::format_description::well_known::Rfc3339)?;
    let now = now_utc();
    let dur = (now - start_t).whole_seconds().max(0);
    let now_s = to_rfc3339(now);

    sqlx::query("UPDATE time_entries SET end_time=?, duration_seconds=? WHERE id=?")
        .bind(&now_s)
        .bind(dur)
        .bind(id)
        .execute(&mut **tx)
        .await?;

    sqlx::query("UPDATE active_timer SET time_entry_id=NULL WHERE singleton=1")
        .execute(&mut **tx)
        .await?;
    Ok(())
}
