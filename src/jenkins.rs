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
use crate::cli;
use crate::error;

// Responses are consumed the moment you read in something like its body. If
// easily toggleable debugging is desired, reqwest::Response is not the way to
// go - you will enter borrow-checker hell, from which there is no escape or
// respite. This is proven science.
//
pub struct BufferedResponse {
    headers: reqwest::header::HeaderMap,
    status: reqwest::StatusCode,
    text: String,
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
    println!("Enqueueing at '{}'", url);
    // println!("Using token {}", config.server.token);
    let response = jenkins_request(
        &config,
        reqwest::Method::POST,
        // I reckon this can't be borrowed because it's going into a Future.
        url.clone(),
    ).await?;
    let response = reqwest::Client::new()
        .post(url)
        .basic_auth(&config.server.username, Some(&config.server.token))
        .send()
        .await
        .map_err(error::AppError::JenkinsEnqueueError)?;
    let buffered_response = to_buffered_response(response).await?;
    let location = buffered_response.headers[reqwest::header::LOCATION]
            .to_str()
            .map_err(error::AppError::JenkinsHeaderError)?
            .to_string();
    println!(
        "result? {}\n{}\n{}",
        buffered_response.status,
        location,
        buffered_response.text,
    );
    // TODO: Return a URL and make sure this is a URL.
    Ok(location)
}

// Uses waiting period to poll for the queue item's state.
pub async fn build_queue_item_poll<'a>(
    config: &'a cli::CliValid,
    url: String,
) {

}

pub async fn build_summarize<'a>(
    config: &'a cli::CliValid,
    url: String,
) -> Result<String, error::AppError> {
    let response = jenkins_request(
        &config,
        reqwest::Method::GET,
        // See https://issues.jenkins.io/browse/JENKINS-45218 which indicates
        // the URL needs an additional "/api/<format>" suffix to work.
        format!("{}/api/json", url),
    ).await?;
    let buffered_response = to_buffered_response(response).await?;
    println!(
        "result? {}\n{}\n{}",
        buffered_response.status,
        headers_to_string(buffered_response.headers)?,
        buffered_response.text,
    );
    Ok(buffered_response.text)
}

async fn jenkins_request<'a>(
    config: &'a cli::CliValid,
    method: reqwest::Method,
    url: String,
) -> Result<reqwest::Response, error::AppError> {
    reqwest::Client::new()
        .request(method, url)
        .basic_auth(&config.server.username, Some(&config.server.token))
        .send()
        .await
        .map_err(error::AppError::JenkinsEnqueueError)
}

// Possibly also to become build_log_stream.
fn build_log_watch() {

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
