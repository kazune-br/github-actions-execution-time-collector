use anyhow::Result;
use chrono::prelude::*;
use clap::{Arg, ArgAction, ArgMatches, Command};
use indicatif::{ProgressBar, ProgressStyle};

#[derive(Default, Debug)]
pub struct Cli {
    matches: ArgMatches,
}

impl Cli {
    pub fn new() -> Self {
        Self {
            matches: Command::new("github-actions-execution-time-collector")
                .arg(Arg::new("owner_name").long("o"))
                .arg(Arg::new("repository_name").long("r"))
                .arg(Arg::new("from_date").long("from"))
                .arg(Arg::new("to_date").long("to"))
                .get_matches(),
        }
    }

    pub fn extract_args(&self) -> (String, String, DateTime<Utc>, DateTime<Utc>) {
        println!("hello");
        let owner_name = self
            .matches
            .get_one::<String>("owner_name")
            .expect("owner_name must be given");
        let repository_name = self
            .matches
            .get_one::<String>("repository_name")
            .expect("repository_name must be given");
        let from_date = self.convert_into_datetime("from_date");
        let to_date = self.convert_into_datetime("to_date");
        (
            owner_name.to_string(),
            repository_name.to_string(),
            from_date,
            to_date,
        )
    }

    fn convert_into_datetime(&self, arg_name: &str) -> DateTime<Utc> {
        let date_parsed = self
            .matches
            .get_one::<String>(arg_name)
            .expect("from_date must be given")
            .split('-')
            .collect::<Vec<&str>>()
            .into_iter()
            .map(|s| {
                s.parse::<u32>()
                    .expect("failed to parse from str to int, but must be parsed")
            })
            .collect::<Vec<u32>>();
        Utc.with_ymd_and_hms(date_parsed[0] as i32, date_parsed[1], date_parsed[2], 0, 0, 0).unwrap()
    }
}

pub fn new_progress_bar(bar_length: u64, bar_name: String) -> Result<ProgressBar> {
    let pb = ProgressBar::new(bar_length);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{prefix:>12.cyan.bold} [{bar:57}] {pos}/{len} {wide_msg}")?
            .progress_chars("=> "),
    );
    pb.set_prefix(bar_name);
    Ok(pb)
}
