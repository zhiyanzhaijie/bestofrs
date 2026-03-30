use app::app_error::{AppError, AppResult};
use infra::setup::init_app_container;
use tracing::warn;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> AppResult<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("warn")),
        )
        .with_target(false)
        .init();

    match run().await {
        Ok(()) => {}
        Err(err) => warn!("ingest daily snapshots failed: {err}"),
    }

    Ok(())
}

async fn run() -> AppResult<()> {
    let container = init_app_container().await?;
    let res = container.ingest_daily_snapshots.execute().await?;

    for failure in &res.failures {
        warn!(
            repo_id = %failure.repo_id,
            lookup_key = %failure.lookup_key,
            error = %failure.error,
            "ingest daily snapshot item failed"
        );
    }

    Ok(())
}
