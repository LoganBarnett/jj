use jj_lib::{LogFormat, LogLevel};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::process::Command;
use thiserror::Error;

use crate::cli::CliRaw;

#[derive(Debug, Error)]
pub enum ConfigError {
  #[error("Failed to read config at {path:?}: {source}")]
  FileRead {
    path: PathBuf,
    #[source]
    source: std::io::Error,
  },
  #[error("Failed to parse config at {path:?}: {source}")]
  Parse {
    path: PathBuf,
    #[source]
    source: toml::de::Error,
  },
  #[error("Server '{0}' not found in configuration")]
  ServerNotFound(String),
  #[error("Failed to evaluate token command: {0}")]
  TokenEval(std::io::Error),
  #[error("Failed to read token command output: {0}")]
  TokenRead(std::string::FromUtf8Error),
  #[error("Missing HOME environment variable: {0}")]
  HomeVar(std::env::VarError),
  #[error("Configuration validation failed: {0}")]
  Validation(String),
}

#[derive(Serialize, Deserialize)]
pub struct ConfigFileRaw {
  pub default_server: String,
  #[serde(flatten)]
  pub servers: HashMap<String, ConfigServerFileRaw>,
}

// See https://serde.rs/lifetimes.html for details regarding how to make this a
// lifetime deserialization.
#[derive(Clone, Serialize, Deserialize)]
pub struct ConfigServerFileRaw {
  pub host_url: String,
  // The string to be evaluated using the shell which will provide the token.
  // To embed a literal token, wrap it in single quotes: "'my-token'".
  pub token_eval: String,
  pub username: String,
}

#[derive(Clone)]
pub struct Config {
  pub log_level: LogLevel,
  pub log_format: LogFormat,
  pub default_server: String,
  pub servers: HashMap<String, ConfigServer>,
}

#[derive(Clone)]
pub struct ConfigServer {
  pub name: String,
  pub host_url: String,
  pub username: String,
  pub token: String,
}

impl Config {
  pub fn from_cli_and_file(cli: &CliRaw) -> Result<Self, ConfigError> {
    let home = std::env::var("HOME").map_err(ConfigError::HomeVar)?;
    let path: PathBuf = [&home, ".config", "jj", "config.toml"]
      .iter()
      .collect();
    let contents =
      std::fs::read_to_string(&path).map_err(|source| ConfigError::FileRead {
        path: path.clone(),
        source,
      })?;
    let raw: ConfigFileRaw =
      toml::from_str(&contents).map_err(|source| ConfigError::Parse {
        path: path.clone(),
        source,
      })?;

    let log_level = cli
      .log_level
      .as_deref()
      .unwrap_or("info")
      .parse::<LogLevel>()
      .map_err(|e| ConfigError::Validation(e.to_string()))?;

    let log_format = cli
      .log_format
      .as_deref()
      .unwrap_or("text")
      .parse::<LogFormat>()
      .map_err(|e| ConfigError::Validation(e.to_string()))?;

    let servers = raw
      .servers
      .into_iter()
      .map(|(k, v)| {
        Ok((
          k.clone(),
          ConfigServer {
            name: k,
            host_url: v.host_url,
            token: token_eval(v.token_eval)?,
            username: v.username,
          },
        ))
      })
      .collect::<Result<HashMap<String, ConfigServer>, ConfigError>>()?;

    Ok(Config {
      log_level,
      log_format,
      default_server: raw.default_server,
      servers,
    })
  }
}

fn token_eval(token_code: String) -> Result<String, ConfigError> {
  // Beware that sh could be a shell you don't expect in your environment.
  Command::new("sh")
    .args(["-c", &token_code])
    .output()
    .map_err(ConfigError::TokenEval)
    .and_then(|x| String::from_utf8(x.stdout).map_err(ConfigError::TokenRead))
}
