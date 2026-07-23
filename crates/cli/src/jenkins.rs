// When starting a build for a Jenkins job, this immediately puts the job into
// the build queue.  The build queue might be consumed very quickly, and so
// there's a challenge involved in picking up the associated job that came with
// it.  Creating a job from the API isn't enough to get this information, and
// documentation is very slim.
//
// The abstract documentation is here:
// https://www.jenkins.io/doc/book/using/remote-access-api/
//
// Viewing specific documentation is much more difficult.  While there is
// documentation to be found on the Jenkins server in question, the
// documentation there doesn't lend itself well to API consumers who are looking
// for strict contracts, documented edge cases, etc.  In absence of this hard
// documentation, we will assume a defensive posture with Jenkins.
use either::{Either, Left, Right};
use futures::StreamExt;
use hash_color_lib::HashColorizer;
use jj_lib::build::BuildStatus;
use reqwest_middleware::ClientWithMiddleware;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::Write;
use tracing::*;

use crate::config::ConfigServer;
use crate::error;
use crate::error::AppError;
use crate::error::AppError::JenkinsBuildParamSerialize;

// Responses are consumed the moment you read in something like its body.  If
// easily toggleable debugging is desired, reqwest::Response is not the way to
// go - you will enter borrow-checker hell, from which there is no escape or
// respite.  This is proven science.
#[derive(Debug)]
pub struct BufferedResponse {
  pub headers: reqwest::header::HeaderMap,
  pub status: reqwest::StatusCode,
  pub text: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JenkinsBuildSummary {
  pub number: u64,
  pub url: String,
  pub building: bool,
  // "SUCCESS" | "FAILURE" | "ABORTED" | "UNSTABLE" | null while building.
  pub result: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct JenkinsJobBuilds {
  pub builds: Vec<JenkinsBuildSummary>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
// Mirrors the Jenkins build-detail API response; some fields are retained for
// documentation and future use rather than read today.
#[allow(dead_code)]
pub struct JenkinsBuildDetail {
  pub number: u64,
  pub url: String,
  pub building: bool,
  pub result: Option<String>,
  pub timestamp: u64,
  pub duration: u64,
  pub display_name: Option<String>,
  pub actions: Vec<JenkinsBuildDetailAction>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
// Mirrors the Jenkins API; not all fields are read.
#[allow(dead_code)]
pub struct JenkinsBuildDetailAction {
  #[serde(alias = "_class")]
  pub class: Option<String>,
  pub causes: Option<Vec<JenkinsBuildDetailCause>>,
  pub last_built_revision: Option<JenkinsBuildDetailRevision>,
  pub remote_urls: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
// Mirrors the Jenkins API; not all fields are read.
#[allow(dead_code)]
pub struct JenkinsBuildDetailCause {
  #[serde(alias = "_class")]
  pub class: Option<String>,
  pub short_description: Option<String>,
  pub user_id: Option<String>,
  pub user_name: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JenkinsBuildDetailRevision {
  pub sha1: Option<String>,
  pub branch: Option<Vec<JenkinsBuildDetailBranch>>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
// Mirrors the Jenkins API; not all fields are read.
#[allow(dead_code)]
pub struct JenkinsBuildDetailBranch {
  pub name: Option<String>,
  pub sha1: Option<String>,
}

async fn to_buffered_response(
  r: reqwest::Response,
) -> Result<BufferedResponse, error::AppError> {
  Ok(BufferedResponse {
    headers: r.headers().clone(),
    status: r.status(),
    text: r
      .text()
      .await
      .map_err(|e| error::AppError::JenkinsEnqueue(e.into()))?,
  })
}

// Serializes the run's parameters into the `json` form payload the Jenkins web
// UI submits: an array of {name, value} objects.  This is the only submission
// form every parameter type understands; buildWithParameters silently drops
// values for richer types such as ExtendedChoiceParameter (JENKINS-57125), so
// jj posts this payload through /build instead.
fn params_to_json_payload(
  params: &HashMap<String, String>,
) -> Result<String, AppError> {
  #[derive(Serialize)]
  struct BuildParam<'a> {
    name: &'a str,
    value: &'a str,
  }
  #[derive(Serialize)]
  struct BuildParams<'a> {
    parameter: Vec<BuildParam<'a>>,
  }
  serde_json::to_string(&BuildParams {
    parameter: params
      .iter()
      .map(|(name, value)| BuildParam { name, value })
      .collect(),
  })
  .map_err(JenkinsBuildParamSerialize)
}

// Jenkins offers no single enqueue endpoint that both carries every parameter
// type and hands back a queue-item handle, so jj picks one per run:
//
//   - /build for a parameterless job and /buildWithParameters for a
//     parameterized one are the documented API and return a queue-item Location
//     jj can watch precisely (race-free).
//   - The web UI's /build `json` form is the only form that carries richer
//     types whose values buildWithParameters silently drops (JENKINS-57125,
//     e.g. ExtendedChoiceParameter), but it returns the job URL rather than a
//     queue item (JENKINS-30317, resolved "Not A Defect"), so the build is
//     watched by number, accepting a small enqueue race for that case.
//
// A run takes the queue path only when every parameter it sets is a type
// buildWithParameters is known to carry (the built-in core types plus any the
// server config adds); any other run uses the form.  Encoding the known-good
// types — rather than the ever-growing set of plugin types buildWithParameters
// drops — means an unrecognized type merely degrades to the race, never
// silently drops a value.

#[derive(Deserialize)]
struct JobConfig {
  #[serde(default)]
  property: Vec<JobProperty>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct JobProperty {
  parameter_definitions: Option<Vec<ParameterDefinition>>,
}

#[derive(Deserialize)]
struct ParameterDefinition {
  name: String,
  #[serde(alias = "_class")]
  class: String,
}

#[derive(Deserialize)]
struct JenkinsQueueItem {
  executable: Option<JenkinsQueueItemExecutable>,
  why: Option<String>,
}

#[derive(Deserialize)]
struct JenkinsQueueItemExecutable {
  number: u64,
  url: String,
}

// Jenkins core parameter types buildWithParameters is known to carry.  A run
// that sets only these (plus any the server config adds) can take the race-free
// queue path.  This is deliberately a known-good list: an unrecognized type
// degrades to the /build form (the race), never to a silently dropped value.
const BUILD_WITH_PARAMETERS_CORE_TYPES: &[&str] = &[
  "hudson.model.StringParameterDefinition",
  "hudson.model.BooleanParameterDefinition",
  "hudson.model.ChoiceParameterDefinition",
  "hudson.model.TextParameterDefinition",
  "hudson.model.PasswordParameterDefinition",
];

fn build_with_parameters_safe(class: &str, extra: &[String]) -> bool {
  BUILD_WITH_PARAMETERS_CORE_TYPES.contains(&class)
    || extra.iter().any(|t| t == class)
}

async fn job_parameter_definitions(
  client: &ClientWithMiddleware,
  server: &ConfigServer,
  job: &str,
) -> Result<Vec<ParameterDefinition>, error::AppError> {
  let response = jenkins_request(
    client,
    server,
    reqwest::Method::GET,
    format!("{}/job/{}/api/json", server.host_url, job),
  )
  .await
  .map_err(error::AppError::JenkinsParameterDefinitions)?;
  let text = response
    .text()
    .await
    .map_err(|e| error::AppError::JenkinsParameterDefinitions(e.into()))?;
  Ok(
    serde_json::from_str::<JobConfig>(&text)
      .map_err(error::AppError::JenkinsDeserialize)?
      .property
      .into_iter()
      .filter_map(|p| p.parameter_definitions)
      .flatten()
      .collect(),
  )
}

// Enqueues a run and returns its (build URL, build number).  See the comment
// above for how the endpoint is chosen from the job's parameter types.
pub async fn build_start(
  client: &ClientWithMiddleware,
  server: &ConfigServer,
  job: &str,
  params: &HashMap<String, String>,
) -> Result<(String, u64), error::AppError> {
  let definitions = job_parameter_definitions(client, server, job).await?;
  if definitions.is_empty() {
    // Parameterless: bare /build returns a queue item to watch.
    let url = format!("{}/job/{}/build", server.host_url, job);
    let queue_url = build_enqueue_to_queue(client, server, url, params).await?;
    return build_queue_item_poll(client, server, queue_url).await;
  }
  // Parameterized: buildWithParameters keeps the race-free queue handle, but
  // only if every value the run sets is a type it is known to carry.
  let queue_safe = params.keys().all(|name| {
    definitions.iter().any(|d| {
      &d.name == name
        && build_with_parameters_safe(
          &d.class,
          &server.build_with_parameters_additional_types,
        )
    })
  });
  if queue_safe {
    let url = format!("{}/job/{}/buildWithParameters", server.host_url, job);
    let queue_url = build_enqueue_to_queue(client, server, url, params).await?;
    build_queue_item_poll(client, server, queue_url).await
  } else {
    // The form path yields no queue handle, so record the number the next build
    // will take, enqueue, then watch that build.
    let build_number = job_next_build_number(client, server, job).await?;
    build_enqueue_form(client, server, job, params).await?;
    let build_url =
      build_await_start(client, server, job, build_number).await?;
    Ok((build_url, build_number))
  }
}

// Enqueues through an endpoint that returns the queue-item Location (bare
// /build, or /buildWithParameters with the params as query) and returns it.
async fn build_enqueue_to_queue(
  client: &ClientWithMiddleware,
  server: &ConfigServer,
  url: String,
  params: &HashMap<String, String>,
) -> Result<String, error::AppError> {
  debug!("Enqueueing at '{}'", url);
  trace!("Using token {}", server.token);
  let response = client
    .request(reqwest::Method::POST, url)
    .basic_auth(&server.username, Some(&server.token))
    .query(params)
    .send()
    .await
    .map_err(error::AppError::JenkinsEnqueue)?;
  let buffered_response = to_buffered_response(response).await?;
  debug!(
    "result? {}\n{}\n{}",
    buffered_response.status,
    headers_to_string(buffered_response.headers.clone())?,
    buffered_response.text,
  );
  if !buffered_response.status.is_success() {
    return Err(error::AppError::JenkinsEnqueueRejected(
      buffered_response.status,
    ));
  }
  buffered_response
    .headers
    .get(reqwest::header::LOCATION)
    .ok_or(error::AppError::JenkinsBuildNotFound)?
    .to_str()
    .map_err(error::AppError::JenkinsHeader)
    .map(str::to_string)
}

// Enqueues through the web UI's /build `json` form, which carries every
// parameter type but returns only the job URL (no queue handle).
async fn build_enqueue_form(
  client: &ClientWithMiddleware,
  server: &ConfigServer,
  job: &str,
  params: &HashMap<String, String>,
) -> Result<(), error::AppError> {
  let url = format!("{}/job/{}/build", server.host_url, job);
  debug!("Enqueueing (form) at '{}'", url);
  trace!("Using token {}", server.token);
  let response = client
    .request(reqwest::Method::POST, url)
    .basic_auth(&server.username, Some(&server.token))
    .form(&[("json", params_to_json_payload(params)?)])
    .send()
    .await
    .map_err(error::AppError::JenkinsEnqueue)?;
  let buffered_response = to_buffered_response(response).await?;
  debug!(
    "result? {}\n{}\n{}",
    buffered_response.status,
    headers_to_string(buffered_response.headers.clone())?,
    buffered_response.text,
  );
  if buffered_response.status.is_success() {
    Ok(())
  } else {
    Err(error::AppError::JenkinsEnqueueRejected(buffered_response.status))
  }
}

// When jj can read Jenkins' reported quiet-period delay it waits exactly that
// long before re-checking — the server's own floor; when it cannot (an
// unrecognised reason such as "Waiting for next available executor") it falls
// back to this interval and keeps polling.
const ASSUMED_QUEUE_DELAY: std::time::Duration =
  std::time::Duration::from_millis(500);

// Parses the delay Jenkins suggests before re-checking a still-queued item from
// its `why` message.  The reported forms, each a witnessed variation, are:
//
//   - "In the quiet period. Expires in 3 sec"
//   - "In the quiet period. Expires in 3 secs"
//   - "In the quiet period. Expires in 3.15 sec"
//   - "In the quiet period. Expires in 3.15 secs"
//   - "In the quiet period. Expires in 234 ms"
//   - "In the quiet period. Expires in 1.234 ms"
//   - "Finished waiting"
//
// "Finished waiting" is a small lie — the build URL is not quite ready — so we
// wait a beat.  Any other message returns None, and the caller polls at
// ASSUMED_QUEUE_DELAY rather than giving up on a reason it has not seen before.
fn parse_why(why: &str) -> Option<std::time::Duration> {
  if why == "Finished waiting" {
    return Some(std::time::Duration::from_secs(1));
  }
  let captures = lazy_regex::regex!(
    r"^In the quiet period\. Expires in ([0-9]+)(?:\.([0-9]+))? (secs?|ms)$"
  )
  .captures(why)?;
  let whole: u64 = captures.get(1)?.as_str().parse().ok()?;
  let millis = if captures.get(3)?.as_str() == "ms" {
    // A fractional millisecond is below our polling resolution, so ignore it.
    whole
  } else {
    // Seconds, with the fractional part read to millisecond resolution: "15"
    // means 0.15 s (150 ms), "5" means 0.5 s (500 ms).
    let fraction_ms: String = captures
      .get(2)
      .map_or("", |m| m.as_str())
      .chars()
      .chain("000".chars())
      .take(3)
      .collect();
    whole * 1000 + fraction_ms.parse::<u64>().unwrap_or(0)
  };
  Some(std::time::Duration::from_millis(millis))
}

// The delay to wait before re-checking a still-queued item: what Jenkins
// reported, or the assumed interval when its reason is unrecognised.
fn queue_delay(item: &JenkinsQueueItem) -> std::time::Duration {
  item.why.as_deref().and_then(parse_why).unwrap_or_else(|| {
    warn!(
      "Unrecognised queue reason {:?}; polling at the assumed interval.",
      item.why,
    );
    ASSUMED_QUEUE_DELAY
  })
}

// Given a polled queue item, returns either the delay to wait before re-checking
// (still queued) or the started build's (URL, number).
fn parse_item(
  item: &JenkinsQueueItem,
) -> Either<std::time::Duration, (String, u64)> {
  item.executable.as_ref().map_or_else(
    || Left(queue_delay(item)),
    |executable| Right((executable.url.clone(), executable.number)),
  )
}

async fn build_queue_item_get(
  client: &ClientWithMiddleware,
  server: &ConfigServer,
  queue_url: &str,
) -> Result<JenkinsQueueItem, error::AppError> {
  let response = jenkins_request(
    client,
    server,
    reqwest::Method::GET,
    format!("{}api/json?tree=executable[number,url],why", queue_url),
  )
  .await
  .map_err(error::AppError::JenkinsQueueItemAwait)?;
  let text = response
    .text()
    .await
    .map_err(|e| error::AppError::JenkinsQueueItemAwait(e.into()))?;
  serde_json::from_str(&text).map_err(error::AppError::JenkinsDeserialize)
}

// Polls a queue item until Jenkins assigns it an executable (the started build),
// waiting the server's reported delay each round (or an assumed interval for an
// unrecognised reason).  It has no timeout: a build can legitimately sit in the
// queue for a long time on a busy server, so giving up on our own would be the
// wrong surprise; a caller that wants a deadline can wrap this in one.
async fn build_queue_item_poll(
  client: &ClientWithMiddleware,
  server: &ConfigServer,
  queue_url: String,
) -> Result<(String, u64), error::AppError> {
  loop {
    // await-in-loop: sequential poll — re-check the same queue item after
    // waiting the reported (or assumed) delay; nothing runs concurrently.
    let item = build_queue_item_get(client, server, &queue_url).await?;
    match parse_item(&item) {
      Right(build) => return Ok(build),
      Left(delay) => tokio::time::sleep(delay).await,
    }
  }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct JenkinsJobNextBuild {
  next_build_number: u64,
}

// Reads the number Jenkins will assign to the job's next build.  jj captures
// this immediately before enqueuing so it can watch the resulting build: /build
// returns the job URL, not a queue item, so there is no queue Location to
// follow.  A concurrent trigger on the same job in the window between this read
// and the enqueue could take the number instead, but that race is negligible
// for a single user driving one build.
pub async fn job_next_build_number(
  client: &ClientWithMiddleware,
  server: &ConfigServer,
  job: &str,
) -> Result<u64, error::AppError> {
  let response = jenkins_request(
    client,
    server,
    reqwest::Method::GET,
    format!("{}/job/{}/api/json?tree=nextBuildNumber", server.host_url, job),
  )
  .await
  .map_err(error::AppError::JenkinsNextBuildNumber)?;
  let text = response
    .text()
    .await
    .map_err(|e| error::AppError::JenkinsNextBuildNumber(e.into()))?;
  serde_json::from_str::<JenkinsJobNextBuild>(&text)
    .map(|n| n.next_build_number)
    .map_err(error::AppError::JenkinsDeserialize)
}

// Waits for the numbered build to start and returns its URL.  Jenkins may hold
// the build in the queue (quiet period, no free executor), during which the
// build's api/json 404s; poll until it appears.  Like the queue poll, it has no
// timeout — a busy server may legitimately take a while to start the build.
pub async fn build_await_start(
  client: &ClientWithMiddleware,
  server: &ConfigServer,
  job: &str,
  build_number: u64,
) -> Result<String, error::AppError> {
  let build_url = format!("{}/job/{}/{}/", server.host_url, job, build_number);
  loop {
    let response = jenkins_request(
      client,
      server,
      reqwest::Method::GET,
      format!("{}api/json?tree=number", build_url),
    )
    // await-in-loop: sequential poll for the build to appear; nothing to run
    // concurrently.
    .await
    .map_err(error::AppError::JenkinsBuildAwait)?;
    if response.status().is_success() {
      return Ok(build_url);
    }
    if response.status() != reqwest::StatusCode::NOT_FOUND {
      return Err(error::AppError::JenkinsBuildAwaitStatus(response.status()));
    }
    tokio::time::sleep(ASSUMED_QUEUE_DELAY).await;
  }
}

pub async fn build_log_stream(
  client: &ClientWithMiddleware,
  server: &ConfigServer,
  url: String,
  mut start_pos: u64,
  build_number: u64,
  colorizer: &HashColorizer,
) -> Result<(), error::AppError> {
  loop {
    let response = jenkins_request(
      client,
      server,
      reqwest::Method::GET,
      format!("{}logText/progressiveText?start={}", url, start_pos),
    )
    // await-in-loop: streaming the log in order — each page is fetched and
    // printed, and its end offset drives the next request; sequential by
    // construction.
    .await
    .map_err(error::AppError::JenkinsBuildStream)?;

    debug!(
      "Headers for stream: \n{}",
      headers_to_string(response.headers().clone())?,
    );
    // Headers must be parsed before the response body is consumed.
    let offset = header_parse::<_, u64>("x-text-size", "0", &response)?;
    let more = header_parse::<_, bool>(
      "x-more-data",
      // It may stop appearing if the job is done.  Default to false.
      "false",
      &response,
    )?;
    debug!("Found offset of {}.", offset);
    debug!("Need more? {}", more);
    let prefix =
      format!("[{}] ", colorizer.colorize(&build_number.to_string()));
    stream_with_prefix(response, &prefix).await?;

    if !more {
      return Ok(());
    }
    start_pos = offset;
  }
}

async fn stream_with_prefix(
  response: reqwest::Response,
  prefix: &str,
) -> Result<(), error::AppError> {
  let mut stream = response.bytes_stream();
  // await-in-loop: consuming a byte stream in arrival order; inherently
  // sequential.
  while let Some(chunk) = stream.next().await {
    let bytes = chunk.map_err(error::AppError::JenkinsBuildResponseRead)?;
    prefixed_write(&bytes, prefix)?;
  }
  Ok(())
}

fn prefixed_write(chunk: &[u8], prefix: &str) -> Result<(), error::AppError> {
  let text = String::from_utf8_lossy(chunk);
  let stdout = std::io::stdout();
  let mut stdout_locked = stdout.lock();
  text
    .split('\n')
    .filter(|line| !line.is_empty())
    .try_for_each(|line| {
      writeln!(stdout_locked, "{}{}", prefix, line)
        .map_err(error::AppError::JenkinsBuildOutput)
    })
}

pub async fn jenkins_job_builds(
  client: &ClientWithMiddleware,
  server: &ConfigServer,
  job: &str,
) -> Result<JenkinsJobBuilds, error::AppError> {
  let url = format!(
    "{}/job/{}/api/json?tree=builds[number,url,result,building]{{0,20}}",
    server.host_url, job,
  );
  let response = jenkins_request(client, server, reqwest::Method::GET, url)
    .await
    .map_err(error::AppError::JenkinsJobBuildsRequest)?;
  let text = response
    .text()
    .await
    .map_err(|e| error::AppError::JenkinsJobBuildsRequest(e.into()))?;
  serde_json::from_str(&text)
    .map_err(error::AppError::JenkinsJobBuildsDeserialize)
}

pub async fn build_detail_get(
  client: &ClientWithMiddleware,
  server: &ConfigServer,
  job: &str,
  build_number: u64,
) -> Result<JenkinsBuildDetail, AppError> {
  let url =
    format!("{}/job/{}/{}/api/json", server.host_url, job, build_number,);
  let response = jenkins_request(client, server, reqwest::Method::GET, url)
    .await
    .map_err(AppError::JenkinsBuildDetailRequest)?;
  let text = response
    .text()
    .await
    .map_err(|e| AppError::JenkinsBuildDetailRequest(e.into()))?;
  serde_json::from_str(&text).map_err(AppError::JenkinsBuildDetailDeserialize)
}

pub async fn build_log_fetch(
  client: &ClientWithMiddleware,
  server: &ConfigServer,
  job: &str,
  build_number: u64,
) -> Result<String, AppError> {
  let url =
    format!("{}/job/{}/{}/consoleText", server.host_url, job, build_number,);
  let response = jenkins_request(client, server, reqwest::Method::GET, url)
    .await
    .map_err(AppError::JenkinsBuildLogFetch)?;
  response.text().await.map_err(AppError::JenkinsBuildLogRead)
}

pub fn jenkins_result_to_status(result: Option<&str>) -> BuildStatus {
  match result {
    Some("SUCCESS") => BuildStatus::Success,
    Some("FAILURE") => BuildStatus::Failure,
    Some("ABORTED") => BuildStatus::Aborted,
    Some("UNSTABLE") => BuildStatus::Unstable,
    None => BuildStatus::Running,
    Some(other) => BuildStatus::Unknown(other.to_string()),
  }
}

fn header_parse<K, V>(
  header_name: K,
  default: &str,
  response: &reqwest::Response,
) -> Result<V, error::AppError>
where
  K: reqwest::header::AsHeaderName,
  V: core::str::FromStr,
{
  response
    .headers()
    .get(header_name)
    .map_or(default, |s| s.to_str().unwrap_or(default))
    .parse::<V>()
    .map_err(|_| error::AppError::JenkinsBuildParseTextSize)
}

async fn jenkins_request(
  client: &ClientWithMiddleware,
  server: &ConfigServer,
  method: reqwest::Method,
  url: String,
) -> Result<reqwest::Response, reqwest_middleware::Error> {
  client
    .request(method, url)
    .basic_auth(&server.username, Some(&server.token))
    .send()
    .await
}

fn headers_to_string(
  headers: reqwest::header::HeaderMap,
) -> Result<String, error::AppError> {
  headers
    .into_iter()
    .map(|(k, v)| {
      Ok(format!(
        "{}: {}",
        k.as_ref().map_or_else(|| "UnknownHeader", |h| h.as_str()),
        v.to_str()?,
      ))
    })
    .collect::<Result<Vec<String>, reqwest::header::ToStrError>>()
    .map_err(error::AppError::JenkinsHeader)
    .map(|xs| xs.join("\n"))
}

#[cfg(test)]
mod tests {
  use super::parse_why;
  use std::time::Duration;

  #[test]
  fn parse_why_whole_seconds_singular_and_plural() {
    assert_eq!(
      parse_why("In the quiet period. Expires in 3 sec"),
      Some(Duration::from_secs(3)),
    );
    assert_eq!(
      parse_why("In the quiet period. Expires in 3 secs"),
      Some(Duration::from_secs(3)),
    );
  }

  #[test]
  fn parse_why_fractional_seconds_singular_and_plural() {
    assert_eq!(
      parse_why("In the quiet period. Expires in 3.15 sec"),
      Some(Duration::from_millis(3150)),
    );
    assert_eq!(
      parse_why("In the quiet period. Expires in 3.15 secs"),
      Some(Duration::from_millis(3150)),
    );
  }

  #[test]
  fn parse_why_milliseconds() {
    assert_eq!(
      parse_why("In the quiet period. Expires in 234 ms"),
      Some(Duration::from_millis(234)),
    );
    // A sub-millisecond fraction is below our polling resolution.
    assert_eq!(
      parse_why("In the quiet period. Expires in 1.234 ms"),
      Some(Duration::from_millis(1)),
    );
  }

  #[test]
  fn parse_why_treats_finished_waiting_as_a_short_wait() {
    assert_eq!(parse_why("Finished waiting"), Some(Duration::from_secs(1)));
  }

  #[test]
  fn parse_why_returns_none_for_an_unrecognised_reason() {
    // e.g. waiting for an executor — the caller then polls at the assumed
    // interval rather than giving up.
    assert_eq!(parse_why("Waiting for next available executor"), None);
    assert_eq!(parse_why(""), None);
  }
}
