//! CLI configuration.
//!
//! `Config` is turned into a foundation-managed CLI/file merge by the
//! `MergeConfig` derive, which generates `CliRaw` (clap), `ConfigFileRaw`
//! (serde), `ConfigError`, and `from_cli_and_file`.  jj's server registry is
//! application-specific: it is resolved by `resolve_registry`, a `skip`-field
//! resolver that loads the same config file foundation discovers and evaluates
//! each server's token through the shell.

use jj_lib::{LogFormat, LogLevel};
use rust_template_foundation::config::{find_config_file, load_toml};
use rust_template_foundation::MergeConfig;
use serde::Deserialize;
use std::collections::HashMap;
use std::process::Command;
use thiserror::Error;

use crate::cli::CliCommand;

#[derive(Debug, Clone, MergeConfig)]
#[merge_config(app_name = "jj", extra_error = "ServerConfigError")]
pub struct Config {
  #[merge_config(common)]
  pub log_level: LogLevel,
  #[merge_config(common)]
  pub log_format: LogFormat,
  /// Server to target, by name from the config file.  The literal "default"
  /// resolves to the file's `default_server`.
  #[merge_config(short, default = "\"default\".to_string()")]
  pub server: String,
  /// Server registry loaded from the config file, with each server's token
  /// evaluated.  Populated by [`Config::resolve_registry`].
  #[merge_config(skip)]
  pub registry: ServerRegistry,
  #[merge_config(subcommand)]
  pub command: CliCommand,
}

impl Config {
  // The `skip` field contract has the derive call
  // `Self::resolve_registry(&cli, &file)` before the merged fields move out of
  // `cli`/`file`.  jj re-reads the discovered config file rather than routing
  // its bare-table server format through the generated `ConfigFileRaw`, which
  // would collide with foundation's flattened common fields.
  fn resolve_registry(
    cli: &CliRaw,
    _file: &ConfigFileRaw,
  ) -> Result<ServerRegistry, ConfigError> {
    let Some(path) = find_config_file("jj", cli.config.as_deref()) else {
      return Ok(ServerRegistry::default());
    };
    let raw: ServerConfigFile = load_toml(&path)?;
    Ok(ServerRegistry {
      default_server: raw.default_server,
      servers: raw
        .servers
        .into_iter()
        .map(|(name, server)| {
          let username = server.username.map_or_else(
            || std::env::var("USER").map_err(ServerConfigError::UserVar),
            Ok,
          )?;
          Ok((
            name.clone(),
            ConfigServer {
              name,
              host_url: server.host_url,
              token: token_eval(&server.token_eval)?,
              username,
              build_with_parameters_additional_types: server
                .build_with_parameters_additional_types,
            },
          ))
        })
        .collect::<Result<HashMap<String, ConfigServer>, ServerConfigError>>(
        )?,
    })
  }
}

/// Server registry resolved from the config file.
#[derive(Clone, Debug, Default)]
pub struct ServerRegistry {
  pub default_server: String,
  pub servers: HashMap<String, ConfigServer>,
}

/// A fully-resolved server whose token has already been evaluated.
#[derive(Clone, Debug)]
pub struct ConfigServer {
  // The server's registry key, kept for identification; not read today.
  #[allow(dead_code)]
  pub name: String,
  pub host_url: String,
  pub username: String,
  pub token: String,
  /// Extra Jenkins parameter `_class` names this server may submit through
  /// `buildWithParameters`; see
  /// [`ConfigServerFileRaw::build_with_parameters_additional_types`].
  pub build_with_parameters_additional_types: Vec<String>,
}

/// Config-file shape for jj's server registry: `default_server` alongside a
/// flattened set of bare `[<name>]` server tables.
#[derive(Debug, Default, Deserialize)]
pub struct ServerConfigFile {
  #[serde(default)]
  pub default_server: String,
  #[serde(flatten)]
  pub servers: HashMap<String, ConfigServerFileRaw>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ConfigServerFileRaw {
  pub host_url: String,
  // Shell expression evaluated to produce the token.  To embed a literal
  // token, wrap it in single quotes: "'my-token'".
  pub token_eval: String,
  pub username: Option<String>,
  /// Jenkins parameter `_class` names — in addition to jj's built-in core
  /// types — that jj may submit through `buildWithParameters`, its race-free
  /// queue path.  A run that sets a parameter of any other type falls back to
  /// the `/build` form (which carries every type but cannot hand back a queue
  /// handle).  List a plugin's parameter type here once you have confirmed
  /// buildWithParameters carries its value on your Jenkins.
  #[serde(default)]
  pub build_with_parameters_additional_types: Vec<String>,
}

/// jj-specific configuration failures, surfaced through the derive's
/// `ConfigError::Extra` variant.
#[derive(Debug, Error)]
pub enum ServerConfigError {
  #[error("Failed to evaluate token command: {0}")]
  TokenEval(#[source] std::io::Error),
  #[error("Failed to read token command output: {0}")]
  TokenRead(#[source] std::string::FromUtf8Error),
  #[error("Missing USER environment variable: {0}")]
  UserVar(#[source] std::env::VarError),
}

fn token_eval(token_code: &str) -> Result<String, ServerConfigError> {
  // Beware that sh could be a shell you don't expect in your environment.
  Command::new("sh")
    .args(["-c", token_code])
    .output()
    .map_err(ServerConfigError::TokenEval)
    .and_then(|output| {
      String::from_utf8(output.stdout).map_err(ServerConfigError::TokenRead)
    })
}
