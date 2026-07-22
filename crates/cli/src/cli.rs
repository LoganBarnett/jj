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

#[derive(Subcommand, Debug, Clone)]
pub enum CliCommand {
  /// Manage and run Jenkins jobs
  Job(JobArgs),
  /// Inspect Jenkins builds
  Build(BuildArgs),
}

#[derive(Parser, Debug, Clone)]
pub struct JobArgs {
  #[command(subcommand)]
  pub command: JobCommand,
}

#[derive(Subcommand, Debug, Clone)]
pub enum JobCommand {
  /// Enqueue a job run and stream its log to completion
  Run(JobRunArgs),
  /// Stream logs from active builds of a job
  Follow(JobFollowArgs),
}

#[derive(Parser, Debug, Clone)]
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

#[derive(Parser, Debug, Clone)]
pub struct JobFollowArgs {
  /// Adopt the next build and exit with its result code instead of watching
  /// continuously.
  #[arg(long)]
  pub once: bool,
  pub job: String,
}

#[derive(Parser, Debug, Clone)]
pub struct BuildArgs {
  #[command(subcommand)]
  pub command: BuildCommand,
}

#[derive(Subcommand, Debug, Clone)]
pub enum BuildCommand {
  /// Show metadata and/or log for a specific build
  View(BuildViewArgs),
}

#[derive(Parser, Debug, Clone)]
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
    config.registry.default_server.clone()
  } else {
    server_name.to_string()
  };
  config
    .registry
    .servers
    .get(&name)
    .cloned()
    .ok_or(error::AppError::CliConfigServerMissing(name))
}

fn build_client() -> ClientWithMiddleware {
  ClientBuilder::new(reqwest::Client::new()).build()
}

pub fn cli_job_run_validate(
  config: &config::Config,
  args: &JobRunArgs,
) -> Result<CliJobRunValid, error::AppError> {
  Ok(CliJobRunValid {
    server: resolve_server(&config.server, config)?,
    client: build_client(),
    job: args.job.clone(),
    params: args.params.iter().cloned().collect(),
  })
}

pub fn cli_job_follow_validate(
  config: &config::Config,
  args: &JobFollowArgs,
) -> Result<CliJobFollowValid, error::AppError> {
  Ok(CliJobFollowValid {
    server: resolve_server(&config.server, config)?,
    client: build_client(),
    job: args.job.clone(),
    once: args.once,
  })
}

pub fn cli_build_view_validate(
  config: &config::Config,
  args: &BuildViewArgs,
) -> Result<CliBuildViewValid, error::AppError> {
  // When neither flag is specified, show both metadata and log by default.
  let (show_metadata, show_log) = if !args.metadata && !args.log {
    (true, true)
  } else {
    (args.metadata, args.log)
  };
  Ok(CliBuildViewValid {
    server: resolve_server(&config.server, config)?,
    client: build_client(),
    job: args.job.clone(),
    build_number: args.build_number,
    show_metadata,
    show_log,
  })
}
