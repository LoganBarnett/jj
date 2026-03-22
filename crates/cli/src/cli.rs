use clap::{Parser, Subcommand};
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};
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
  #[arg(short, long, default_value = "default")]
  pub server: String,
  /// Log level (trace, debug, info, warn, error)
  #[arg(long, env = "LOG_LEVEL")]
  pub log_level: Option<String>,
  /// Log format (text, json)
  #[arg(long, env = "LOG_FORMAT")]
  pub log_format: Option<String>,
  #[command(subcommand)]
  pub command: CliCommand,
}

#[derive(Subcommand, Debug)]
pub enum CliCommand {
  /// Manage and run Jenkins jobs
  Job(JobArgs),
  /// Inspect Jenkins builds
  Build(BuildArgs),
}

#[derive(Parser, Debug)]
pub struct JobArgs {
  #[command(subcommand)]
  pub command: JobCommand,
}

#[derive(Subcommand, Debug)]
pub enum JobCommand {
  /// Enqueue a job run and stream its log to completion
  Run(JobRunArgs),
  /// Stream logs from active builds of a job
  Follow(JobFollowArgs),
}

#[derive(Parser, Debug)]
pub struct JobRunArgs {
  pub job: String,
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

#[derive(Parser, Debug)]
pub struct JobFollowArgs {
  /// Adopt the next build and exit with its result code instead of watching
  /// continuously.
  #[arg(long)]
  pub once: bool,
  pub job: String,
}

#[derive(Parser, Debug)]
pub struct BuildArgs {
  #[command(subcommand)]
  pub command: BuildCommand,
}

#[derive(Subcommand, Debug)]
pub enum BuildCommand {
  /// Show metadata and/or log for a specific build
  View(BuildViewArgs),
}

#[derive(Parser, Debug)]
pub struct BuildViewArgs {
  pub job: String,
  pub build_number: u64,
  /// Show build metadata only (default: show both)
  #[arg(long)]
  pub metadata: bool,
  /// Show build log only (default: show both)
  #[arg(long)]
  pub log: bool,
}

#[derive(Clone)]
pub struct CliJobRunValid {
  pub server: config::ConfigServer,
  pub client: ClientWithMiddleware,
  pub job: String,
  pub params: HashMap<String, String>,
}

#[derive(Clone)]
pub struct CliJobFollowValid {
  pub server: config::ConfigServer,
  pub client: ClientWithMiddleware,
  pub job: String,
  pub once: bool,
}

#[derive(Clone)]
pub struct CliBuildViewValid {
  pub server: config::ConfigServer,
  pub client: ClientWithMiddleware,
  pub job: String,
  pub build_number: u64,
  pub show_metadata: bool,
  pub show_log: bool,
}

fn resolve_server(
  server_name: &str,
  config: &config::Config,
) -> Result<config::ConfigServer, error::AppError> {
  let name = if server_name == "default" {
    config.default_server.clone()
  } else {
    server_name.to_string()
  };
  config
    .servers
    .get(&name)
    .cloned()
    .ok_or(error::AppError::CliConfigServerMissing(name))
}

fn build_client() -> ClientWithMiddleware {
  ClientBuilder::new(reqwest::Client::new()).build()
}

pub fn cli_job_run_validate(
  cli: &CliRaw,
  args: &JobRunArgs,
  config: &config::Config,
) -> Result<CliJobRunValid, error::AppError> {
  Ok(CliJobRunValid {
    server: resolve_server(&cli.server, config)?,
    client: build_client(),
    job: args.job.clone(),
    params: args.params.iter().cloned().collect(),
  })
}

pub fn cli_job_follow_validate(
  cli: &CliRaw,
  args: &JobFollowArgs,
  config: &config::Config,
) -> Result<CliJobFollowValid, error::AppError> {
  Ok(CliJobFollowValid {
    server: resolve_server(&cli.server, config)?,
    client: build_client(),
    job: args.job.clone(),
    once: args.once,
  })
}

pub fn cli_build_view_validate(
  cli: &CliRaw,
  args: &BuildViewArgs,
  config: &config::Config,
) -> Result<CliBuildViewValid, error::AppError> {
  // When neither flag is specified, show both metadata and log by default.
  let (show_metadata, show_log) = if !args.metadata && !args.log {
    (true, true)
  } else {
    (args.metadata, args.log)
  };
  Ok(CliBuildViewValid {
    server: resolve_server(&cli.server, config)?,
    client: build_client(),
    job: args.job.clone(),
    build_number: args.build_number,
    show_metadata,
    show_log,
  })
}
