use std::io::Write;

use crate::cli::CliBuildViewValid;
use crate::error::AppError;
use crate::jenkins;

// Converts a days-since-Unix-epoch count to (year, month, day) using the
// Gregorian calendar algorithm from Howard Hinnant's date library.
fn days_to_ymd(days_since_epoch: u64) -> (u32, u32, u32) {
  let z = days_since_epoch as i64 + 719468;
  let era = if z >= 0 { z / 146097 } else { (z - 146096) / 146097 };
  let doe = (z - era * 146097) as u64;
  let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
  let y = yoe as i64 + era * 400;
  let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
  let mp = (5 * doy + 2) / 153;
  let d = doy - (153 * mp + 2) / 5 + 1;
  let m = if mp < 10 { mp + 3 } else { mp - 9 };
  let y = if m <= 2 { y + 1 } else { y };
  (y as u32, m as u32, d as u32)
}

fn format_timestamp_ms(ms: u64) -> String {
  let total_secs = ms / 1000;
  let second = total_secs % 60;
  let total_mins = total_secs / 60;
  let minute = total_mins % 60;
  let total_hours = total_mins / 60;
  let hour = total_hours % 24;
  let days = total_hours / 24;
  let (year, month, day) = days_to_ymd(days);
  format!(
    "{:04}-{:02}-{:02} {:02}:{:02}:{:02} UTC",
    year, month, day, hour, minute, second
  )
}

fn format_duration_ms(ms: u64) -> String {
  let total_secs = ms / 1000;
  let mins = total_secs / 60;
  let secs = total_secs % 60;
  if mins > 0 {
    format!("{}m {}s", mins, secs)
  } else {
    format!("{}s", secs)
  }
}

pub async fn view_build(config: &CliBuildViewValid) -> Result<(), AppError> {
  if config.show_metadata {
    let detail = jenkins::build_detail_get(
      &config.client,
      &config.server,
      &config.job,
      config.build_number,
    )
    .await?;

    let status_str = detail
      .result
      .as_deref()
      .unwrap_or(if detail.building { "BUILDING" } else { "UNKNOWN" });

    let cause = detail
      .actions
      .iter()
      .filter_map(|a| a.causes.as_ref())
      .flatten()
      .next()
      .and_then(|c| c.short_description.as_deref())
      .unwrap_or("unknown");

    let revision = detail
      .actions
      .iter()
      .filter_map(|a| a.last_built_revision.as_ref())
      .next();

    let commit_str = revision
      .map(|r| {
        let sha = r.sha1.as_deref().unwrap_or("unknown");
        let short_sha = if sha.len() >= 8 { &sha[..8] } else { sha };
        let branch = r
          .branch
          .as_ref()
          .and_then(|bs| bs.first())
          .and_then(|b| b.name.as_deref())
          .unwrap_or("unknown");
        format!("{} on {}", short_sha, branch)
      })
      .unwrap_or_else(|| "unknown".to_string());

    let stdout = std::io::stdout();
    let mut out = stdout.lock();
    writeln!(out, "Build #{}     [{}]", detail.number, status_str)
      .map_err(AppError::JenkinsBuildOutput)?;
    writeln!(
      out,
      "Started:   {}",
      format_timestamp_ms(detail.timestamp)
    )
    .map_err(AppError::JenkinsBuildOutput)?;
    writeln!(out, "Duration:  {}", format_duration_ms(detail.duration))
      .map_err(AppError::JenkinsBuildOutput)?;
    writeln!(out, "Cause:     {}", cause)
      .map_err(AppError::JenkinsBuildOutput)?;
    writeln!(out, "Commit:    {}", commit_str)
      .map_err(AppError::JenkinsBuildOutput)?;
    writeln!(out, "URL:       {}", detail.url)
      .map_err(AppError::JenkinsBuildOutput)?;
    if config.show_log {
      writeln!(out).map_err(AppError::JenkinsBuildOutput)?;
    }
  }

  if config.show_log {
    let log = jenkins::build_log_fetch(
      &config.client,
      &config.server,
      &config.job,
      config.build_number,
    )
    .await?;
    print!("{}", log);
  }

  Ok(())
}
