use thiserror::Error;

use crate::config::ConfigError;

#[derive(Debug, Error)]
pub enum AppError {
  #[error("Server '{0}' not found in configuration")]
  CliConfigServerMissing(String),
  #[error("Configuration error: {0}")]
  Config(#[from] ConfigError),
  #[error("Jenkins build not found in response headers")]
  JenkinsBuildNotFound,
  #[error("Failed to parse build text size header")]
  JenkinsBuildParseTextSize,
  #[error("Failed to serialize build parameters: {0}")]
  JenkinsBuildParamSerialize(serde_url_params::Error),
  #[error("Failed to stream build log: {0}")]
  JenkinsBuildStream(reqwest::Error),
  #[error("Failed to read build response body: {0}")]
  JenkinsBuildResponseRead(reqwest::Error),
  #[error("Failed to write build output to stdout: {0}")]
  JenkinsBuildOutput(std::io::Error),
  #[error("Failed to deserialize Jenkins response: {0}")]
  JenkinsDeserialize(serde_json::Error),
  #[error("Failed to enqueue Jenkins build: {0}")]
  JenkinsEnqueue(reqwest::Error),
  #[error("Failed to deserialize Jenkins queue response: {0}")]
  JenkinsEnqueueDeserialize(String),
  #[error("Failed to parse queue wait duration: {0}")]
  JenkinsEnqueueSecondsParse(std::num::ParseIntError),
  #[error("Unrecognized queue wait reason: {0}")]
  JenkinsEnqueueWait(String),
  #[error("Failed to parse response header value: {0}")]
  JenkinsHeader(reqwest::header::ToStrError),
}
