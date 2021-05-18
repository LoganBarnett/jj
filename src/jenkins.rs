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

pub async fn build_enqueue(
    config: cli::CliValid,
) -> Result<BufferedResponse, error::AppError> {
    let url = format!("{}/job/{}/build", config.server.host_url, config.job);
    println!("Enqueueing at '{}'", url);
    // println!("Using token {}", config.server.token);
    let response = reqwest::Client::new()
        .post(url)
        .basic_auth(config.server.username, Some(config.server.token))
        .send()
        .await
        .map_err(error::AppError::JenkinsEnqueueError)?;
    let buffered_response = to_buffered_response(response).await?;
    println!(
        "result? {}\n{}\n{}",
        buffered_response.status,
        buffered_response.headers[reqwest::header::LOCATION]
            .to_str()
            .map_err(error::AppError::JenkinsHeaderError)?
            .to_string(),
        buffered_response.text,
    );
    Ok(buffered_response)
}

fn build_summarize() {

}

// Possibly also to become build_log_stream.
fn build_log_watch() {

}
