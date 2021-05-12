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

fn build_enqueue(config: cli::ConfigValid) {
    reqwest::post(format!("{}/job/{}", config.server.host_url, config.job))
        .basic_auth(config.server.username, config.server.token)
        .send()
}

fn build_summarize() {

}

// Possibly also to become build_log_stream.
fn build_log_watch() {

}
