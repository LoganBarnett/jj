use jj_lib::{LogFormat, LogLevel};
use tracing_subscriber::{
  fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter, Layer,
};

// CLIs write program output to stdout and all log output to stderr so that
// the two streams can be separated (e.g. piping output while watching logs).
pub fn init_logging(level: LogLevel, format: LogFormat) {
  let env_filter = EnvFilter::try_from_default_env()
    .unwrap_or_else(|_| EnvFilter::new(level.to_string()));

  // On Unix, attempt to route structured log events directly to the systemd
  // journal.  Falls back to stderr below when the journal socket is absent
  // (macOS, Docker without socket mount, etc.).
  #[cfg(unix)]
  if let Ok(journald) = tracing_journald::layer() {
    tracing_subscriber::registry()
      .with(journald.with_filter(env_filter))
      .init();
    return;
  }

  match format {
    LogFormat::Text => {
      tracing_subscriber::registry()
        .with(
          fmt::layer()
            .with_writer(std::io::stderr)
            .with_target(true)
            .with_line_number(true)
            .with_filter(env_filter),
        )
        .init();
    }
    LogFormat::Json => {
      tracing_subscriber::registry()
        .with(
          fmt::layer()
            .json()
            .with_writer(std::io::stderr)
            .with_target(true)
            .with_line_number(true)
            .with_filter(env_filter),
        )
        .init();
    }
  }
}
