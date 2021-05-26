/**
 * When starting a build for a Jenkins job, this immediately puts the job into
 * the build queue. The build queue might be consumed very quickly, and so
 * there's a challenge involved in picking up the associated job that came with
 * it. Creating a job from the API isn't enough to get this information, and
 * documentation is very slim.
 *
 * The abstract documentation is here:
 * https://www.jenkins.io/doc/book/using/remote-access-api/
 *
 * Viewing specific documentation is much more difficult. While there is
 * documentation to be found on the Jenkins server in question, the
 * documentation there doesn't lend itself well to API consumers who are looking
 * for strict contracts, documented edge cases, etc. In absense of this hard
 * documentation, we will assume a defensive posture with Jenkins.
 *
 * It's possible the author has missed the in-depth documentation, but readers
 * should be aware that at least some searching was done before declaring said
 * documentation absent.
 */
use either::{Either, Left, Right};
// Give me all of the futures magic, reasonability be damned.
use futures::FutureExt;
use futures::StreamExt;
// Gives us macros such as debug! and error! See logging.rs for setup.
use log::*;
use serde::{Deserialize, Serialize};
// Used for writing to our stdout via a stream.
use std::io::Write;
use crate::cli;
use crate::error;

// Responses are consumed the moment you read in something like its body. If
// easily toggleable debugging is desired, reqwest::Response is not the way to
// go - you will enter borrow-checker hell, from which there is no escape or
// respite. This is proven science.
//
#[derive(Debug)]
pub struct BufferedResponse {
    headers: reqwest::header::HeaderMap,
    status: reqwest::StatusCode,
    text: String,
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
    pub causes: Vec<JenkinsQueueItemActionCause>,
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
    // This value is rather important. One example is

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

async fn to_buffered_response(
    r: reqwest::Response,
) -> Result<BufferedResponse, error::AppError> {
    Ok(BufferedResponse {
        headers: r.headers().clone(),
        status: r.status(),
        text: r.text().await.map_err(error::AppError::JenkinsEnqueueError)?,
    })
}

// Returns the link to the queue item.
pub async fn build_enqueue<'a>(
    config: &'a cli::CliValid,
) -> Result<String, error::AppError> {
    let url = format!("{}/job/{}/build", config.server.host_url, config.job);
    debug!("Enqueueing at '{}'", url);
    trace!("Using token {}", config.server.token);
    let response = jenkins_request(
        &config,
        reqwest::Method::POST,
        // I reckon this can't be borrowed because it's going into a Future.
        url.clone(),
    )
        .await
        .map_err(error::AppError::JenkinsEnqueueError)?;
    let buffered_response = to_buffered_response(response).await?;
    let location = buffered_response.headers[reqwest::header::LOCATION]
            .to_str()
            .map_err(error::AppError::JenkinsHeaderError)?
            .to_string();
    debug!(
        "result? {}\n{}\n{}",
        buffered_response.status,
        location,
        buffered_response.text,
    );
    // TODO: Return a URL and make sure this is a URL.
    Ok(location)
}

fn parse_item(
    item: &JenkinsQueueItem,
) -> Result<Either<u64, String>, error::AppError> {
    match &item.executable {
        Some(ex) => Ok(Right(ex.url.clone())),
        None => {
            let why = item.why.as_ref().ok_or_else(|| {
                error::AppError::JenkinsEnqueueDeserializeError(format!(
                    "Both executable and why fields missing data: {:?}",
                    item,
                ))
            })?;
            Ok(Left(parse_why(&why)?))
        },
    }
}

fn parse_why(why: &String) -> Result<u64, error::AppError> {
    lazy_static::lazy_static! {
        // Broken out to print during an error.
        static ref QUEUE_DELAY_REGEX_STRING: &'static str =
            // How to do something like sec(?:s)? ?
            //
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
        .captures(&why)
        .and_then(|c| {
            Some((
                // Sometimes we get "Expires in x.y secs" and others
                // "Expires in x ms". Instead of putting in a bunch of
                // chained conditional logic, just assume 0 seconds if
                // we are on the latter form.
                c.name("secs1")
                 .or(c.name("secs2"))
                 .map(|x| x.as_str())
                 .or(Some("0"))?,
                c.name("millis1")
                 .or(c.name("millis2"))?.as_str()
            ))
        })
        .or_else(|| {
            if QUEUE_FINISHED_WAITING_REGEX.is_match(&why) {
                // Just wait for a second. Finished waiting is actually
                // a lie.  We need to wait a little more and we can get
                // the build URL.
                Some(("1", "0"))
            } else {
                None
            }
        })
        ;
    match option {
        Some((seconds_string, millis_string)) => {
            let seconds = seconds_string.parse::<u64>()
                .map_err(error::AppError::JenkinsEnqueueSecondsParseError)?
            * 1000;
            let millis = millis_string.parse::<u64>()
                .map_err(error::AppError::JenkinsEnqueueSecondsParseError)?;
            Ok(seconds + millis)
        },
        None => {
            Err(error::AppError::JenkinsEnqueueWaitError(format!(
                "Queue item's why of '{}' does not match regular expression /{}/.",
                why,
                QUEUE_DELAY_REGEX_STRING.to_string(),
            )))
        }
    }
}

// Uses waiting period to poll for the queue item's state.
pub fn build_queue_item_poll<'a>(
    config: &'a cli::CliValid,
    url: String,
) ->
    // What have I done?
    std::pin::Pin<
            Box<dyn std::future::Future<
                    Output = Result<String, error::AppError>
                    > + Send + 'a
                >> {
        async move {
            let item = build_queue_item_get(&config, url.clone()).await?;
            match parse_item(&item)? {
                Left(expires_millis) => {
                    std::thread::sleep(
                        std::time::Duration::from_millis(expires_millis),
                    );
                    build_queue_item_poll(&config, url).await
                },
                Right(url) => Ok(url),
            }
        }.boxed()
}

pub async fn build_queue_item_get<'a>(
    config: &'a cli::CliValid,
    url: String,
) -> Result<JenkinsQueueItem, error::AppError> {
    let response = jenkins_request(
        &config,
        reqwest::Method::GET,
        // See https://issues.jenkins.io/browse/JENKINS-45218 which indicates
        // the URL needs an additional "/api/<format>" suffix to work.
        format!("{}/api/json", url),
    )
        .await
        .map_err(error::AppError::JenkinsEnqueueError)?;
    let buffered_response = to_buffered_response(response).await?;
    debug!(
        "result? {}\n{}\n{}",
        buffered_response.status,
        headers_to_string(buffered_response.headers)?,
        buffered_response.text,
    );
    serde_json::from_str(&buffered_response.text)
        .map_err(error::AppError::JenkinsDeserializeError)
}

pub fn build_log_stream<'a>(
    config: &'a cli::CliValid,
    url: String,
    start_pos: u64,
) -> std::pin::Pin<
        Box<dyn std::future::Future<
                Output = Result<(), error::AppError>
                > + Send + 'a
            >> {
    async move {
        let response = jenkins_request(
            &config,
            reqwest::Method::GET,
            format!("{}logText/progressiveText?start={}", url, start_pos),
        )
            .await
            .map_err(error::AppError::JenkinsBuildStreamError)?;

        debug!(
            "Headers for stream: \n{}",
            headers_to_string(response.headers().clone())?,
        );
        // This needs to happen before stdout_write lest we anger the borrow
        // checker.
        //
        // TODO: Clean up type parameter usage.
        let offset = header_parse::<_, u64>("x-text-size", "0", &response)?;
        let more = header_parse::<_, bool>(
            "x-more-data",
            // It may stop appearing if the job is done. Default to false.
            "false",
            &response,
        )?;
        info!("Found offset of {}.", offset);
        info!("Need more? {}", more);
        stdout_write(response)?;

        if !more {
            Ok(())
        } else {
            build_log_stream(&config, url, offset).await
        }
    }.boxed()
}

fn stdout_write(
    response: reqwest::Response,
) -> Result<(), error::AppError> {
    let mut stream = response.bytes_stream();
    let stdout = std::io::stdout();
    let mut stdout_locked = stdout.lock();

    futures::executor::block_on(async move {
        // It would be great to be able to just connect these streams together
        // via some pipe operation, but that seems beyond my capability right
        // now.
        let mut result = Ok(());
        while let Some(chunk) = stream.next().await {
            result = chunk
                .map_err(error::AppError::JenkinsBuildResponseReadError)
                .and_then(|c| {
                    stdout_locked
                        .write(&c)
                        .map_err(error::AppError::JenkinsBuildOutputError)
                })
                .map(|_| ())
        };
        result
    })
}

fn header_parse<'a, 'b, K, V>(
    header_name: K,
    default: &str,
    response: &'a reqwest::Response,
) -> Result<V, error::AppError>
    // where T: core::str::FromStr
// where T: Into<&'b str> + core::str::FromStr
where
    K: reqwest::header::AsHeaderName,
    V: core::str::FromStr,
{
    response.headers().get(header_name)
        .map(|s| s.to_str().unwrap_or(default))
        .or(Some(default))
        .unwrap() // Zomg a safe unwrap.
        .parse::<V>()
        .map_err(|_| error::AppError::JenkinsBuildParseTextSize)
}

async fn jenkins_request<'a>(
    config: &'a cli::CliValid,
    method: reqwest::Method,
    url: String,
) -> Result<reqwest::Response, reqwest::Error> {
    reqwest::Client::new()
        .request(method, url)
        .basic_auth(&config.server.username, Some(&config.server.token))
        .send()
        .await
}

fn headers_to_string(
    headers: reqwest::header::HeaderMap,
) -> Result<String, error::AppError> {
    headers.into_iter().map(|(k, v)|  {
        Ok(format!(
            "{}: {}",
            k.as_ref()
             .map_or_else(|| "UnknownHeader", |h| h.as_str())
             .to_string(),
            v.to_str()?,
        ))
    }).collect::<Result<Vec<String>, reqwest::header::ToStrError>>()
        .map_err(error::AppError::JenkinsHeaderError)
        .and_then(|xs| Ok(xs.join("\n")))
}
