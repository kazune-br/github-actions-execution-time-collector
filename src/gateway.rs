use crate::model::{Timing, WorkflowRuns, Workflows};
use crate::repository::IGithubRepository;

use anyhow::Result;
use async_trait::async_trait;
use chrono::prelude::*;
use reqwest::Client;
use serde_json::Value;

use regex::Regex;

pub struct GithubGateway {
    client: Client,
    owner_name: String,
    repository_name: String,
}

impl GithubGateway {
    pub fn new(client: Client, owner_name: String, repository_name: String) -> Self {
        Self {
            client,
            owner_name,
            repository_name,
        }
    }
}

#[async_trait]
impl IGithubRepository for GithubGateway {
    async fn find_workflows(&self) -> Result<Workflows> {
        let endpoint = format!(
            "https://api.github.com/repos/{}/{}/actions/workflows",
            self.owner_name, self.repository_name
        );
        Ok(self
            .client
            .get(endpoint)
            .send()
            .await?
            .json::<Workflows>()
            .await?)
    }

    async fn find_workflow_runs(
        &self,
        workflow_id: i64,
        from_date: DateTime<Utc>,
        to_date: DateTime<Utc>,
    ) -> Result<WorkflowRuns> {
        let max_count = self.find_workflow_runs_max_page_counts(workflow_id).await?;
        let mut workflow_runs = WorkflowRuns::empty();
        for i in 0..max_count {
            let endpoint = format!(
                "https://api.github.com/repos/{}/{}/actions/workflows/{}/runs?per_page=100&page={}",
                self.owner_name, self.repository_name, workflow_id, i
            );

            let mut runs = self
                .client
                .get(endpoint.clone())
                .send()
                .await?
                .json::<WorkflowRuns>()
                .await?;
            let reached = runs.is_reached_at_from_date(from_date);
            runs = runs.exclude_runs_exceeding_date_limit(from_date, to_date);
            workflow_runs.add_workflow_runs(runs);

            if reached {
                break;
            }
        }
        Ok(workflow_runs)
    }

    async fn find_workflow_runs_max_page_counts(&self, workflow_id: i64) -> Result<i32> {
        let endpoint = format!(
            "https://api.github.com/repos/{}/{}/actions/workflows/{}/runs?per_page=100&page=1",
            self.owner_name, self.repository_name, workflow_id,
        );
        let res = self.client.get(endpoint).send().await?;
        if !res.headers().contains_key("link") {
            return Ok(1);
        }
        Ok(extract_max_count(
            res.headers()
                .get("link")
                .expect("failed to get link")
                .to_str()
                .expect("failed to convert into str"),
        ))
    }

    async fn find_timing_from_workflow_run(&self, run_id: i64) -> Result<Timing> {
        let endpoint = format!(
            "https://api.github.com/repos/{}/{}/actions/runs/{}/timing",
            self.owner_name, self.repository_name, run_id
        );
        let json_string = self.client.get(endpoint).send().await?.text().await?;
        let value: Value = serde_json::from_str(&json_string)?;
        Ok(Timing::from_value(value))
    }
}

fn extract_max_count(text: &str) -> i32 {
    // parameter `text` example:
    // text="<https://api.github.com/repositories/*****/actions/workflows/*****/runs?per_page=100&page=2>; rel=\"next\", <https://api.github.com/repositories/*****/actions/workflows/*****/runs?per_page=100&page=3>; rel=\"last\"
    let re = Regex::new(r"&page=\d+").expect("failed to init regex");
    text.split(',')
        .collect::<Vec<&str>>()
        .into_iter()
        // 0: <https://api.github.com/repositories/*****/actions/workflows/*****/runs?per_page=100&page=2>; rel=\"next\"
        // 1:  <https://api.github.com/repositories/*****/actions/workflows/*****/runs?per_page=100&page=3>; rel=\"last\"
        .filter(|link_and_rel_text| link_and_rel_text.contains("last"))
        // 1:  <https://api.github.com/repositories/*****/actions/workflows/*****/runs?per_page=100&page=3>; rel=\"last\"
        .map(|link_text| -> i32 {
            let caps = re.captures(link_text).expect("failed to apply regex");
            let ret: Vec<&str> = caps
                .get(0)
                .expect("failed to get value")
                .as_str()
                .split('=')
                .collect();
            // ret=["&page=", "3"]
            ret[1].parse::<i32>().expect("must be parsed as number")
        })
        .collect::<Vec<i32>>()
        .pop()
        .expect("must have some number")
}
