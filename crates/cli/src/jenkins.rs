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
use futures::FutureExt;
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

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct JenkinsQueueItem {
  #[serde(alias = "_class")]
  pub class: String,
  pub actions: Vec<JenkinsQueueItemAction>,
  pub blocked: bool,
  pub buildable: bool,
  pub executable: Option<JenkinsQueueItemExecutable>,
  pub id: u32,
  pub in_queue_since: u64,
  pub params: String,
  pub task: JenkinsQueueItemTask,
  pub url: String,
  pub why: Option<String>,
  pub timestamp: Option<u64>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct JenkinsQueueItemAction {
  #[serde(alias = "_class")]
  pub class: String,
  pub causes: Option<Vec<JenkinsQueueItemActionCause>>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct JenkinsQueueItemExecutable {
  #[serde(alias = "_class")]
  pub class: String,
  pub number: u64,
  pub url: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct JenkinsQueueItemActionCause {
  // This value is rather important.  One example is
  // "hudson.model.Cause$UserIdCause" to indicate a user started the build
  // manually.
  #[serde(alias = "_class")]
  pub class: String,
  pub short_description: String,
  pub user_id: String,
  pub user_name: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct JenkinsQueueItemTask {
  #[serde(alias = "_class")]
  pub class: String,
  pub name: String,
  pub url: String,
  // Color is indicative of the build's status.
  pub color: String,
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
pub struct JenkinsBuildDetailAction {
  #[serde(alias = "_class")]
  pub class: Option<String>,
  pub causes: Option<Vec<JenkinsBuildDetailCause>>,
  pub last_built_revision: Option<JenkinsBuildDetailRevision>,
  pub remote_urls: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
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

pub fn params_to_query_params(
  params: &HashMap<String, String>,
) -> Result<String, AppError> {
  serde_url_params::to_string(&params).map_err(JenkinsBuildParamSerialize)
}

// Returns the link to the queue item.
pub async fn build_enqueue(
  client: &ClientWithMiddleware,
  server: &ConfigServer,
  job: &str,
  params: &HashMap<String, String>,
) -> Result<String, error::AppError> {
  let url = format!(
    "{}/job/{}/buildWithParameters?{}",
    server.host_url,
    job,
    params_to_query_params(params)?,
  );
  debug!("Enqueueing at '{}'", url);
  trace!("Using token {}", server.token);
  let response = jenkins_request(
    client,
    server,
    reqwest::Method::POST,
    // I reckon this can't be borrowed because it's going into a Future.
    url.clone(),
  )
  .await
  .map_err(error::AppError::JenkinsEnqueue)?;
  let buffered_response = to_buffered_response(response).await?;
  debug!(
    "result? {}\n{}\n{}",
    buffered_response.status,
    headers_to_string(buffered_response.headers.clone())?,
    buffered_response.text,
  );
  // TODO: Return a URL and make sure this is a URL.
  // Unwrap it here so we can debug the output regardless of outcome.
  let location = buffered_response
    .headers
    .get(reqwest::header::LOCATION)
    .ok_or(error::AppError::JenkinsBuildNotFound)?
    .to_str()
    .map_err(error::AppError::JenkinsHeader)?
    .to_string();
  Ok(location)
}

fn parse_item(
  item: &JenkinsQueueItem,
) -> Result<Either<u64, (String, u64)>, error::AppError> {
  match &item.executable {
    Some(ex) => Ok(Right((ex.url.clone(), ex.number))),
    None => {
      let why = item.why.as_ref().ok_or_else(|| {
        error::AppError::JenkinsEnqueueDeserialize(format!(
          "Both executable and why fields missing data: {:?}",
          item,
        ))
      })?;
      Ok(Left(parse_why(why)?))
    }
  }
}

fn parse_why(why: &String) -> Result<u64, error::AppError> {
  lazy_static::lazy_static! {
    // Broken out to print during an error.
    static ref QUEUE_DELAY_REGEX_STRING: &'static str =
      // Examples witnessed:
      // Expires in 1.234 ms
      // Expires in 3 sec
      // Expires in 3.15 sec
      // Expires in 3 secs
      r"^In the quiet period\. Expires in (?:(?P<secs1>[0-9]+) secs?)|(?:(?P<secs2>[0-9]+)\.(?P<millis1>[0-9]+) secs?)|(?:(?P<millis2>[0-9]+) ms)$";
    static ref QUEUE_DELAY_REGEX: regex::Regex = regex::Regex::new(
      &QUEUE_DELAY_REGEX_STRING,
    ).unwrap();
    static ref QUEUE_FINISHED_WAITING_REGEX_STRING: &'static str =
      r"^Finished waiting$";
    static ref QUEUE_FINISHED_WAITING_REGEX: regex::Regex = regex::Regex::new(
      &QUEUE_FINISHED_WAITING_REGEX_STRING,
    ).unwrap();
  }
  let option = QUEUE_DELAY_REGEX
    .captures(why)
    .and_then(|c| {
      Some((
        // Sometimes we get "Expires in x.y secs" and others "Expires in x ms".
        // Instead of chained conditional logic, assume 0 seconds for the
        // latter form.
        c.name("secs1")
          .or(c.name("secs2"))
          .map(|x| x.as_str())
          .or(Some("0"))?,
        c.name("millis1").or(c.name("millis2"))?.as_str(),
      ))
    })
    .or_else(|| {
      if QUEUE_FINISHED_WAITING_REGEX.is_match(why) {
        // Just wait for a second.  "Finished waiting" is actually a lie; we
        // need to wait a little more before the build URL becomes available.
        Some(("1", "0"))
      } else {
        None
      }
    });
  match option {
    Some((seconds_string, millis_string)) => {
      let seconds = seconds_string
        .parse::<u64>()
        .map_err(error::AppError::JenkinsEnqueueSecondsParse)?
        * 1000;
      let millis = millis_string
        .parse::<u64>()
        .map_err(error::AppError::JenkinsEnqueueSecondsParse)?;
      Ok(seconds + millis)
    }
    None => Err(error::AppError::JenkinsEnqueueWait(format!(
      "Queue item's why of '{}' does not match regular expression /{}/.",
      why,
      QUEUE_DELAY_REGEX_STRING.to_string(),
    ))),
  }
}

// Uses waiting period to poll for the queue item's state.  Returns the build
// URL and build number once the item leaves the queue.
pub fn build_queue_item_poll<'a>(
  client: &'a ClientWithMiddleware,
  server: &'a ConfigServer,
  url: String,
) -> std::pin::Pin<
  Box<
    dyn std::future::Future<Output = Result<(String, u64), error::AppError>>
      + Send
      + 'a,
  >,
> {
  async move {
    let item = build_queue_item_get(client, server, url.clone()).await?;
    match parse_item(&item)? {
      Left(expires_millis) => {
        std::thread::sleep(std::time::Duration::from_millis(expires_millis));
        build_queue_item_poll(client, server, url).await
      }
      Right((build_url, build_number)) => Ok((build_url, build_number)),
    }
  }
  .boxed()
}

pub async fn build_queue_item_get<'a>(
  client: &'a ClientWithMiddleware,
  server: &'a ConfigServer,
  url: String,
) -> Result<JenkinsQueueItem, error::AppError> {
  let response = jenkins_request(
    client,
    server,
    reqwest::Method::GET,
    // See https://issues.jenkins.io/browse/JENKINS-45218 which indicates
    // the URL needs an additional "/api/<format>" suffix to work.
    format!("{}/api/json", url),
  )
  .await
  .map_err(error::AppError::JenkinsEnqueue)?;
  let buffered_response = to_buffered_response(response).await?;
  debug!(
    "result? {}\n{}\n{}",
    buffered_response.status,
    headers_to_string(buffered_response.headers)?,
    buffered_response.text,
  );
  serde_json::from_str(&buffered_response.text)
    .map_err(error::AppError::JenkinsDeserialize)
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
  for line in text.split('\n') {
    if !line.is_empty() {
      writeln!(stdout_locked, "{}{}", prefix, line)
        .map_err(error::AppError::JenkinsBuildOutput)?;
    }
  }
  Ok(())
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
  serde_json::from_str(&text).map_err(error::AppError::JenkinsJobBuildsDeserialize)
}

pub async fn build_detail_get(
  client: &ClientWithMiddleware,
  server: &ConfigServer,
  job: &str,
  build_number: u64,
) -> Result<JenkinsBuildDetail, AppError> {
  let url = format!(
    "{}/job/{}/{}/api/json",
    server.host_url, job, build_number,
  );
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
  let url = format!(
    "{}/job/{}/{}/consoleText",
    server.host_url, job, build_number,
  );
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

fn header_parse<'a, K, V>(
  header_name: K,
  default: &str,
  response: &'a reqwest::Response,
) -> Result<V, error::AppError>
where
  K: reqwest::header::AsHeaderName,
  V: core::str::FromStr,
{
  response
    .headers()
    .get(header_name)
    .map(|s| s.to_str().unwrap_or(default))
    .or(Some(default))
    .unwrap() // Safe unwrap: or(Some(default)) guarantees Some.
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
        k.as_ref()
          .map_or_else(|| "UnknownHeader", |h| h.as_str())
          .to_string(),
        v.to_str()?,
      ))
    })
    .collect::<Result<Vec<String>, reqwest::header::ToStrError>>()
    .map_err(error::AppError::JenkinsHeader)
    .and_then(|xs| Ok(xs.join("\n")))
}
