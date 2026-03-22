use clap::Parser;
use std::collections::HashMap;
use std::error::Error;

use crate::config;
use crate::error;

// Shameful rip from:
// https://github.com/clap-rs/clap/blob/master/examples/typed-derive.rs#L24-L26
// and
// https://github.com/clap-rs/clap/blob/master/examples/typed-derive.rs#L47-L59
fn parse_key_val<T, U>(
  s: &str,
) -> Result<(T, U), Box<dyn Error + Send + Sync + 'static>>
where
  T: std::str::FromStr,
  T::Err: Error + Send + Sync + 'static,
  U: std::str::FromStr,
  U::Err: Error + Send + Sync + 'static,
{
  let pos = s
    .find('=')
    .ok_or_else(|| format!("invalid KEY=value: no `=` found in `{}`", s))?;
  Ok((s[..pos].parse()?, s[pos + 1..].parse()?))
}

#[derive(Parser, Debug)]
#[command(name = "jj", about = "Run Jenkins jobs from the command line.")]
pub struct CliRaw {
  pub job: String,
  #[arg(short, long, default_value = "default")]
  pub server: String,
  /// Log level (trace, debug, info, warn, error)
  #[arg(long, env = "LOG_LEVEL")]
  pub log_level: Option<String>,
  /// Log format (text, json)
  #[arg(long, env = "LOG_FORMAT")]
  pub log_format: Option<String>,
  // number_of_values = 1 means --param must be repeated for each pair.
  // Values must be provided with an equals sign separating them.  See
  // https://github.com/clap-rs/clap_derive/blob/master/examples/keyvalue.rs
  // for examples.
  #[arg(
    long = "param",
    short = 'P',
    value_parser = parse_key_val::<String, String>,
    number_of_values = 1
  )]
  pub params: Vec<(String, String)>,
}

pub struct CliValid {
  pub job: String,
  pub params: HashMap<String, String>,
  pub server: config::ConfigServer,
}

pub fn cli_validate(
  cli: &CliRaw,
  config: &config::Config,
) -> Result<CliValid, error::AppError> {
  let server_name = if cli.server == "default" {
    config.default_server.clone()
  } else {
    cli.server.clone()
  };
  match config.servers.get(&server_name) {
    Some(server) => Ok(CliValid {
      job: cli.job.clone(),
      params: cli.params.iter().cloned().collect(),
      server: server.clone(),
    }),
    None => Err(error::AppError::CliConfigServerMissing(server_name)),
  }
}
