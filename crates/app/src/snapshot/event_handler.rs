use std::collections::HashMap;
use std::sync::Arc;

use domain::{RepoId, SnapshotRecorded};

use crate::app_error::AppResult;
use crate::snapshot::SnapshotDeltaRepo;

#[derive(Clone)]
pub struct SnapshotEventHandler {
    delta_store: Arc<dyn SnapshotDeltaRepo>,
}

impl SnapshotEventHandler {
    pub fn new(delta_store: Arc<dyn SnapshotDeltaRepo>) -> Self {
        Self { delta_store }
    }

    pub async fn handle_snapshot_recorded(&self, event: SnapshotRecorded) -> AppResult<()> {
        self.handle_snapshots_recorded(&[event]).await
    }

    pub async fn handle_snapshots_recorded(&self, events: &[SnapshotRecorded]) -> AppResult<()> {
        if events.is_empty() {
            return Ok(());
        }

        let mut by_date: HashMap<chrono::NaiveDate, Vec<RepoId>> = HashMap::new();
        for event in events {
            by_date
                .entry(event.snapshot_date)
                .or_default()
                .push(event.repo_id.clone());
        }

        for (date, repo_ids) in by_date {
            self.delta_store.upsert_for_date(&repo_ids, date).await?;
        }

        Ok(())
    }
}
