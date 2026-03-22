mod cli;
mod config;
mod error;
mod follow;
mod jenkins;
mod logging;
mod view;

use clap::Parser;
use cli::{BuildCommand, CliCommand, CliRaw, JobCommand};
use futures::TryFutureExt;
use hash_color_lib::{ColorizerOptions, HashColorizer};
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), error::AppError> {
  let cli = CliRaw::parse();
  let config = config::Config::from_cli_and_file(&cli)?;
  logging::init_logging(config.log_level, config.log_format);
  info!("Starting jj");
  match &cli.command {
    CliCommand::Job(job_args) => match &job_args.command {
      JobCommand::Run(args) => {
        let v = cli::cli_job_run_validate(&cli, args, &config)?;
        let colorizer = HashColorizer::new(ColorizerOptions::default());
        // https://support.cloudbees.com/hc/en-us/articles/360028147532-Get-Build-Number-with-REST-API
        // The above documentation states that the queue item should be around
        // for 5 minutes.  We can use that to query to see which build it has
        // produced, and then use that to poll/watch the build log.
        jenkins::build_enqueue(&v.client, &v.server, &v.job, &v.params)
          .and_then(|url| jenkins::build_queue_item_poll(&v.client, &v.server, url))
          .and_then(|(build_url, build_number)| {
            jenkins::build_log_stream(
              &v.client,
              &v.server,
              build_url,
              0,
              build_number,
              &colorizer,
            )
          })
          .await?;
        info!("Done!");
      }
      JobCommand::Follow(args) => {
        let v = cli::cli_job_follow_validate(&cli, args, &config)?;
        if v.once {
          let code = follow::follow_once(&v).await?;
          std::process::exit(code.0);
        } else {
          follow::follow(&v).await?;
        }
      }
    },
    CliCommand::Build(build_args) => match &build_args.command {
      BuildCommand::View(args) => {
        let v = cli::cli_build_view_validate(&cli, args, &config)?;
        view::view_build(&v).await?;
      }
    },
  }
  Ok(())
}
