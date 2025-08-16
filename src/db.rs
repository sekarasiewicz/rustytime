// src/db.rs
use anyhow::{Context, Result};
use sqlx::SqlitePool;
use std::{
    env,
    path::{Path, PathBuf},
};

fn is_sqlite_url(s: &str) -> bool {
    s.starts_with("sqlite:")
}

fn url_to_fs_path(url: &str) -> Option<PathBuf> {
    // Accept forms like: sqlite://relative.db  or sqlite:///abs/path.db
    let trimmed = url.strip_prefix("sqlite://")?;
    // absolute if it starts with '/', else relative
    Some(PathBuf::from(trimmed))
}

fn ensure_parent_exists(p: &Path) -> Result<()> {
    if let Some(parent) = p.parent() {
        if !parent.as_os_str().is_empty() {
            std::fs::create_dir_all(parent)
                .with_context(|| format!("creating DB directory {:?}", parent))?;
        }
    }
    Ok(())
}

fn default_db_path() -> PathBuf {
    let base = dirs::data_dir()
        .unwrap_or(std::env::current_dir().expect("cwd"))
        .join("rustytime");
    base.join("rustytime.db")
}

pub fn resolve_db_url(cli_db: Option<&str>) -> (String, PathBuf) {
    // Priority: DATABASE_URL (if sqlite:*), then --db, then default path
    if let Ok(env_url) = env::var("DATABASE_URL") {
        if is_sqlite_url(&env_url) {
            let path = url_to_fs_path(&env_url).unwrap_or_else(|| default_db_path());
            return (env_url, path);
        }
    }
    if let Some(db_path) = cli_db {
        if is_sqlite_url(db_path) {
            let path = url_to_fs_path(db_path).unwrap_or_else(|| default_db_path());
            return (db_path.to_string(), path);
        } else {
            let p = PathBuf::from(db_path);
            return (format!("sqlite://{}", db_path), p);
        }
    }
    // fallback: ~/.local/share/rustytime/rustytime.db (on macOS: ~/Library/Application Support/…)
    let p = default_db_path();
    (format!("sqlite://{}", p.display()), p)
}

pub async fn open_db_resolved(db_url: &str, fs_path: &Path) -> Result<SqlitePool> {
    ensure_parent_exists(fs_path)?;
    let pool = SqlitePool::connect(db_url).await?;
    sqlx::migrate!().run(&pool).await?;
    Ok(pool)
}
