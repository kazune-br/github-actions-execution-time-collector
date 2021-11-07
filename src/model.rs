use chrono::prelude::*;
use chrono::DateTime;
use serde::{Deserialize, Serialize};
use serde_query::Deserialize as SQDeserialize;
use std::fs::File;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Workflows {
    values: Vec<Workflow>,
}

impl Workflows {
    pub fn get_workflows(&self) -> Vec<Workflow> {
        self.values.to_vec()
    }
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Workflow {
    id: i64,
    name: String,
}

impl Workflow {
    pub fn get_id(&self) -> i64 {
        self.id
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct WorkflowRuns {
    workflow_runs: Vec<WorkflowRun>,
}

impl WorkflowRuns {
    pub fn empty() -> Self {
        Self {
            workflow_runs: Vec::new(),
        }
    }

    pub fn get_workflow_runs(&self) -> Vec<WorkflowRun> {
        self.workflow_runs.to_vec()
    }

    pub fn get_length(&self) -> usize {
        self.workflow_runs.len()
    }

    pub fn exclude_runs_exceeding_date_limit(
        &mut self,
        from_date: DateTime<Utc>,
        to_date: DateTime<Utc>,
    ) -> Self {
        Self {
            workflow_runs: self
                .workflow_runs
                .clone()
                .into_iter()
                .filter(|wr| wr.is_execution_date_equally_greater_than(from_date))
                .filter(|wr| wr.is_execution_date_equally_smaller_than(to_date))
                .collect::<Vec<WorkflowRun>>(),
        }
    }

    pub fn add_workflow_runs(&mut self, workflow_runs: WorkflowRuns) {
        workflow_runs
            .workflow_runs
            .into_iter()
            .for_each(|wr| self.workflow_runs.push(wr))
    }

    pub fn is_reached_at_from_date(&self, from_date: DateTime<Utc>) -> bool {
        self.workflow_runs
            .iter()
            .any(|wr| wr.is_execution_date_smaller_than(from_date))
    }
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct WorkflowRun {
    id: i64,
    created_at: String,
    status: String,
}

impl WorkflowRun {
    pub fn get_id(&self) -> i64 {
        self.id
    }

    pub fn get_created_at(&self) -> String {
        self.created_at.clone()
    }

    pub fn get_status(&self) -> String {
        self.status.clone()
    }

    /*
        ex:
        execution_date is 2021/09/4(120) && date_limit is 2021/09/03(110) => false
        execution_date is 2021/09/3(110) && date_limit is 2021/09/03(110) => false
        execution_date is 2021/09/2(100) && date_limit is 2021/09/03(110) => true
    */
    fn is_execution_date_smaller_than(&self, date: DateTime<Utc>) -> bool {
        DateTime::parse_from_rfc3339(&self.created_at)
            .expect("failed to parse")
            .timestamp()
            < date.timestamp()
    }

    /*
        ex:
        execution_date is 2021/09/4(120) && date_limit is 2021/09/03(110) => false
        execution_date is 2021/09/3(110) && date_limit is 2021/09/03(110) => true
        execution_date is 2021/09/2(100) && date_limit is 2021/09/03(110) => true
    */
    fn is_execution_date_equally_smaller_than(&self, date: DateTime<Utc>) -> bool {
        DateTime::parse_from_rfc3339(&self.created_at)
            .expect("failed to parse")
            .timestamp()
            <= date.timestamp()
    }

    /*
        ex:
        execution_date is 2021/09/4(120) && date_limit is 2021/09/03(110) => true
        execution_date is 2021/09/3(110) && date_limit is 2021/09/03(110) => true
        execution_date is 2021/09/2(100) && date_limit is 2021/09/03(110) => false
    */
    fn is_execution_date_equally_greater_than(&self, date: DateTime<Utc>) -> bool {
        DateTime::parse_from_rfc3339(&self.created_at)
            .expect("failed to parse")
            .timestamp()
            >= date.timestamp()
    }
}

#[derive(Serialize, SQDeserialize, Debug, Default)]
pub struct Timing {
    #[query(r#".billable.["UBUNTU"].["total_ms"]"#)]
    billable_total_ms: Option<i64>,
    #[query(r#".["run_duration_ms"]"#)]
    run_duration_ms: Option<i64>,
}

impl Timing {
    pub fn get_billable_total_ms(&self) -> Option<i64> {
        self.billable_total_ms
    }

    pub fn get_run_duration_ms(&self) -> Option<i64> {
        self.run_duration_ms
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Timings {
    values: Vec<Timing>,
}

impl Timings {
    pub fn new(values: Vec<Timing>) -> Self {
        Self { values }
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct WorkflowSummary {
    values: Vec<WorkflowRunSummary>,
}

impl WorkflowSummary {
    pub fn new(
        repository_name: String,
        workflow: Workflow,
        workflow_runs: WorkflowRuns,
        timings: Timings,
    ) -> Self {
        let mut reports: Vec<WorkflowRunSummary> = Vec::new();
        for (t, wr) in timings
            .values
            .iter()
            .zip(workflow_runs.get_workflow_runs().iter())
        {
            reports.push(WorkflowRunSummary {
                repository_name: repository_name.clone(),
                workflow_id: workflow.get_id(),
                workflow_name: workflow.get_name(),
                workflow_run_id: wr.get_id(),
                workflow_run_created_at: wr.get_created_at(),
                workflow_run_status: wr.get_status(),
                billable_total_ms: t.get_billable_total_ms(),
                run_duration_ms: t.get_run_duration_ms(),
            })
        }
        Self { values: reports }
    }

    pub fn to_csv(&self, name: i64) {
        if self.get_length() == 0 {
            return;
        }

        let file = File::create(format!("{}.csv", name)).expect("failed to create file");
        let mut wtr = csv::Writer::from_writer(file);
        self.values
            .iter()
            .for_each(|item| wtr.serialize(item).expect("failed to write"));
        wtr.flush().expect("failed to flush");
    }

    pub fn get_length(&self) -> usize {
        self.values.len()
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct WorkflowRunSummary {
    repository_name: String,
    workflow_id: i64,
    workflow_name: String,
    workflow_run_id: i64,
    workflow_run_created_at: String,
    workflow_run_status: String,
    billable_total_ms: Option<i64>,
    run_duration_ms: Option<i64>,
}
