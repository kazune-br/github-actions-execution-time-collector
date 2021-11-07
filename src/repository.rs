use crate::model::{Timing, WorkflowRuns, Workflows};

use anyhow::Result;
use async_trait::async_trait;
use chrono::prelude::*;

#[async_trait]
pub trait IGithubRepository {
    async fn find_workflows(&self) -> Result<Workflows>;
    async fn find_workflow_runs(
        &self,
        workflow_id: i64,
        from_date: DateTime<Utc>,
        to_date: DateTime<Utc>,
    ) -> Result<WorkflowRuns>;
    async fn find_workflow_runs_max_page_counts(&self, workflow_id: i64) -> Result<i32>;
    async fn find_timing_from_workflow_run(&self, run_id: i64) -> Result<Timing>;
}
