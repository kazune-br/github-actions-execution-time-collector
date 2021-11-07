mod cli;
mod gateway;
mod infrastructure;
mod model;
mod repository;

use cli::{new_progress_bar, Cli};
use gateway::GithubGateway;
use infrastructure::new_api_client;
use model::{Timing, Timings, Workflow, WorkflowRuns, WorkflowSummary};
use repository::IGithubRepository;

use anyhow::Result;
use futures::future::try_join_all;
use std::env;
use std::sync::Arc;
use tokio::sync::mpsc;

#[tokio::main]
async fn main() -> Result<()> {
    let (owner_name, repository_name, from_date, to_date) = Cli::new().extract_args();
    let token = env::var("GITHUB_TOKEN")?;
    let client = new_api_client(token)?;
    let gateway = Arc::new(GithubGateway::new(
        client,
        owner_name,
        repository_name.clone(),
    ));

    let workflows = gateway.find_workflows().await?;
    let workflow_runs_tasks = workflows
        .get_workflows()
        .into_iter()
        .map(|w| {
            let gateway = gateway.clone();
            tokio::spawn(async move {
                gateway
                    .find_workflow_runs(w.get_id(), from_date, to_date)
                    .await
            })
        })
        .collect::<Vec<_>>();
    let workflow_runs_lst = try_join_all(workflow_runs_tasks)
        .await?
        .into_iter()
        .collect::<Result<Vec<WorkflowRuns>>>()?;

    let (tx, mut rx) = mpsc::channel(1);
    tokio::spawn(async move {
        for (wr, w) in workflow_runs_lst
            .into_iter()
            .zip(workflows.get_workflows().into_iter())
        {
            let name = repository_name.clone();
            let id = w.get_id();
            let g = gateway.clone();
            let summary = collect_summary(g, name, w, wr).await;
            if tx.send((summary, id)).await.is_err() {
                println!("receiver dropped");
                return;
            }
        }
    });
    while let Some((summary, id)) = rx.recv().await {
        summary.to_csv(id);
    }
    Ok(())
}

async fn collect_summary(
    gateway: Arc<GithubGateway>,
    repository_name: String,
    workflow: Workflow,
    workflow_runs: WorkflowRuns,
) -> WorkflowSummary {
    let pb = new_progress_bar(workflow_runs.get_length() as u64, workflow.get_name());

    let (tx, mut rx) = mpsc::channel(1);
    for run in workflow_runs.get_workflow_runs().into_iter() {
        let tx = tx.clone();
        let gateway = gateway.clone();
        tokio::spawn(async move {
            match gateway.find_timing_from_workflow_run(run.get_id()).await {
                Ok(timing) => {
                    if tx.send(timing).await.is_err() {
                        println!("receiver dropped");
                    }
                }
                Err(msg) => println!("failure: {}", msg),
            }
        });
    }
    // Note: drop the last sender to stop `rx` waiting for message
    drop(tx);

    let mut timing_lst: Vec<Timing> = Vec::new();
    while let Some(t) = rx.recv().await {
        timing_lst.push(t);
        pb.inc(1);
    }
    pb.finish();

    let timings = Timings::new(timing_lst);
    WorkflowSummary::new(repository_name, workflow, workflow_runs, timings)
}
