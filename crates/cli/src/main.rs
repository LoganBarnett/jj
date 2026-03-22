mod cli;
mod config;
mod error;
mod follow;
mod jenkins;
mod logging;

use clap::Parser;
use cli::CliRaw;
use futures::TryFutureExt;
use hash_color_lib::{ColorizerOptions, HashColorizer};
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), error::AppError> {
  let cli = CliRaw::parse();
  let config = config::Config::from_cli_and_file(&cli)?;
  logging::init_logging(config.log_level, config.log_format);
  info!("Starting jj");
  let validated = cli::cli_validate(&cli, &config)?;
  if validated.follow {
    follow::follow(&validated).await?;
  } else if validated.follow_next {
    let code = follow::follow_next(&validated).await?;
    std::process::exit(code.0);
  } else {
    // This gives us something like this:
    // https://jenkins.foo/queue/item/590249/
    //
    // https://support.cloudbees.com/hc/en-us/articles/360028147532-Get-Build-Number-with-REST-API
    // The above documentation states that the queue item should be around for 5
    // minutes.  We can use that to query to see which build it has produced, and
    // then use that to poll/watch the build log.
    let colorizer = HashColorizer::new(ColorizerOptions::default());
    jenkins::build_enqueue(&validated)
      .and_then(|url| jenkins::build_queue_item_poll(&validated, url))
      .and_then(|(build_url, build_number)| {
        jenkins::build_log_stream(&validated, build_url, 0, build_number, &colorizer)
      })
      .await?;
    info!("Done!");
  }
  Ok(())
}
