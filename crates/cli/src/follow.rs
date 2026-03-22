use std::collections::HashSet;
use std::time::Duration;

use hash_color_lib::{ColorizerOptions, HashColorizer};
use jj_lib::build::BuildExitCode;
use tokio::{signal, task::JoinSet, time};
use tracing::{error, info};

use crate::cli::CliValid;
use crate::error::AppError;
use crate::jenkins;

const POLL_INTERVAL: Duration = Duration::from_secs(5);

fn build_colorizer() -> HashColorizer {
  HashColorizer::new(ColorizerOptions::default())
}

// Adopts the highest-numbered currently-running build, or waits for the next
// build to start.  Streams its log to completion, then exits with the build's
// result code.
pub async fn follow_next(config: &CliValid) -> Result<BuildExitCode, AppError> {
  let colorizer = build_colorizer();

  let builds = jenkins::jenkins_job_builds(config).await?;
  let adopted_info = builds
    .builds
    .iter()
    .filter(|b| b.building)
    .max_by_key(|b| b.number)
    .map(|b| (b.number, b.url.clone()));
  let baseline = if adopted_info.is_none() {
    builds.builds.iter().map(|b| b.number).max().unwrap_or(0)
  } else {
    0
  };

  let (build_number, build_url) = if let Some(info) = adopted_info {
    info
  } else {
    loop {
      time::sleep(POLL_INTERVAL).await;
      let new_builds = jenkins::jenkins_job_builds(config).await?;
      let candidate = new_builds
        .builds
        .iter()
        .filter(|b| b.building && b.number > baseline)
        .max_by_key(|b| b.number)
        .map(|b| (b.number, b.url.clone()));
      if let Some(info) = candidate {
        break info;
      }
    }
  };

  info!(build_number, "Streaming build log");
  jenkins::build_log_stream(config, build_url, 0, build_number, &colorizer).await?;

  let final_builds = jenkins::jenkins_job_builds(config).await?;
  let result_str = final_builds
    .builds
    .iter()
    .find(|b| b.number == build_number)
    .and_then(|b| b.result.as_deref());
  Ok(jenkins::jenkins_result_to_status(result_str).exit_code())
}

// Watches the job continuously, streaming logs from every active build until
// cancelled with Ctrl+C.
pub async fn follow(config: &CliValid) -> Result<(), AppError> {
  let mut seen: HashSet<u64> = HashSet::new();
  let mut tasks: JoinSet<()> = JoinSet::new();
  let mut interval = time::interval(POLL_INTERVAL);

  loop {
    tokio::select! {
      _ = signal::ctrl_c() => break,
      _ = interval.tick() => {
        match jenkins::jenkins_job_builds(config).await {
          Ok(builds) => {
            for build in builds.builds {
              if build.building && !seen.contains(&build.number) {
                seen.insert(build.number);
                let config_clone = config.clone();
                let build_number = build.number;
                let build_url = build.url.clone();
                tasks.spawn(async move {
                  let col = build_colorizer();
                  info!(build_number, "Streaming build log");
                  match jenkins::build_log_stream(
                    &config_clone,
                    build_url,
                    0,
                    build_number,
                    &col,
                  )
                  .await
                  {
                    Ok(()) => info!(build_number, "Build stream complete"),
                    Err(e) => {
                      error!(build_number, error = %e, "Build stream error")
                    }
                  }
                });
              }
            }
          }
          Err(e) => error!(error = %e, "Failed to query job builds"),
        }
      }
      Some(result) = tasks.join_next() => {
        if let Err(e) = result {
          error!(error = %e, "Build stream task panicked");
        }
      }
    }
  }

  tasks.abort_all();
  Ok(())
}
