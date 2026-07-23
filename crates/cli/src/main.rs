//! jj — run Jenkins jobs from the command line.
//!
//! The `#[foundation_main]` macro handles CLI parsing, config resolution, and
//! logging init.  This file dispatches the resolved subcommand to the Jenkins
//! client, follow modes, and build view.

mod cli;
mod config;
mod error;
mod follow;
mod jenkins;
mod view;

use cli::{BuildCommand, CliCommand, JobCommand};
use config::Config;
use hash_color_lib::{ColorizerOptions, HashColorizer};
use rust_template_foundation::main as foundation_main;
use std::process::ExitCode;
use tracing::info;

#[foundation_main]
pub async fn main(config: Config) -> Result<ExitCode, error::AppError> {
  info!("Starting jj");
  match &config.command {
    CliCommand::Job(job_args) => match &job_args.command {
      JobCommand::Run(args) => {
        let v = cli::cli_job_run_validate(&config, args)?;
        let colorizer = HashColorizer::new(ColorizerOptions::default());
        let (build_url, build_number) =
          jenkins::build_start(&v.client, &v.server, &v.job, &v.params).await?;
        jenkins::build_log_stream(
          &v.client,
          &v.server,
          build_url,
          0,
          build_number,
          &colorizer,
        )
        .await?;
        info!("Done!");
        Ok(ExitCode::SUCCESS)
      }
      JobCommand::Follow(args) => {
        let v = cli::cli_job_follow_validate(&config, args)?;
        if v.once {
          let code = follow::follow_once(&v).await?;
          Ok(ExitCode::from(code.0 as u8))
        } else {
          follow::follow(&v).await?;
          Ok(ExitCode::SUCCESS)
        }
      }
    },
    CliCommand::Build(build_args) => match &build_args.command {
      BuildCommand::View(args) => {
        let v = cli::cli_build_view_validate(&config, args)?;
        view::view_build(&v).await?;
        Ok(ExitCode::SUCCESS)
      }
    },
  }
}
