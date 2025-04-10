use sqlx::postgres::PgPool;
use anyhow::Result;

pub async fn run_migrations(database_url: &str) -> Result<()> {
    let pool = PgPool::connect(database_url).await?;
    
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS devices (
            id TEXT PRIMARY KEY,
            connected_at TIMESTAMPTZ NOT NULL,
            last_seen TIMESTAMPTZ NOT NULL,
            telemetry JSONB
        )
        "#
    )
    .execute(&pool)
    .await?;

    Ok(())
} 