use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
  #[error("Server '{0}' not found in configuration")]
  CliConfigServerMissing(String),
  #[error("Failed to parse build text size header")]
  JenkinsBuildParseTextSize,
  #[error("Failed to serialize build parameters: {0}")]
  JenkinsBuildParamSerialize(serde_json::Error),
  #[error("Failed to stream build log: {0}")]
  JenkinsBuildStream(reqwest_middleware::Error),
  #[error("Failed to read build response body: {0}")]
  JenkinsBuildResponseRead(reqwest::Error),
  #[error("Failed to write build output to stdout: {0}")]
  JenkinsBuildOutput(std::io::Error),
  #[error("Failed to deserialize Jenkins response: {0}")]
  JenkinsDeserialize(serde_json::Error),
  #[error("Failed to request the job's parameter definitions: {0}")]
  JenkinsParameterDefinitions(reqwest_middleware::Error),
  #[error("Failed to enqueue Jenkins build: {0}")]
  JenkinsEnqueue(reqwest_middleware::Error),
  #[error("Jenkins rejected the build enqueue with status {0}")]
  JenkinsEnqueueRejected(reqwest::StatusCode),
  #[error("Jenkins returned no queue-item Location for the enqueued build")]
  JenkinsBuildNotFound,
  #[error("Failed to poll the queue item for the started build: {0}")]
  JenkinsQueueItemAwait(reqwest_middleware::Error),
  #[error("Failed to request the job's next build number: {0}")]
  JenkinsNextBuildNumber(reqwest_middleware::Error),
  #[error("Failed to poll for the enqueued build to start: {0}")]
  JenkinsBuildAwait(reqwest_middleware::Error),
  #[error("Unexpected status {0} while waiting for the build to start")]
  JenkinsBuildAwaitStatus(reqwest::StatusCode),
  #[error("Failed to parse response header value: {0}")]
  JenkinsHeader(reqwest::header::ToStrError),
  #[error("Failed to request Jenkins job builds: {0}")]
  JenkinsJobBuildsRequest(reqwest_middleware::Error),
  #[error("Failed to deserialize Jenkins job builds response: {0}")]
  JenkinsJobBuildsDeserialize(serde_json::Error),
  #[error("Failed to request Jenkins build detail: {0}")]
  JenkinsBuildDetailRequest(reqwest_middleware::Error),
  #[error("Failed to deserialize Jenkins build detail: {0}")]
  JenkinsBuildDetailDeserialize(serde_json::Error),
  #[error("Failed to fetch Jenkins build log: {0}")]
  JenkinsBuildLogFetch(reqwest_middleware::Error),
  #[error("Failed to read Jenkins build log response: {0}")]
  JenkinsBuildLogRead(reqwest::Error),
}
