//! Integration tests for the jj CLI.
//!
//! These tests require a live Jenkins instance.  Configure it by setting:
//!
//!   JENKINS_URL   — e.g. `http://localhost:11990`
//!   JENKINS_USER  — Jenkins username
//!   JENKINS_TOKEN — API token or password
//!
//! Tests skip (return without failure) when these variables are absent.
//! Run serially to prevent concurrent builds from confusing `--follow-next`:
//!
//!   cargo test --test integration -- --test-threads=1

use assert_cmd::cargo::CommandCargoExt;
use assert_cmd::Command;
use predicates::prelude::*;
use serial_test::serial;
use std::process::Command as StdCommand;
use std::time::Duration;
use tempfile::TempDir;

struct JenkinsTest {
    // Owns the temp directory; dropping this struct removes it.
    _dir: TempDir,
    home: String,
}

impl JenkinsTest {
    // Returns None when the required environment variables are absent,
    // causing the calling test to return without failure (skip).
    fn setup() -> Option<Self> {
        let url = std::env::var("JENKINS_URL").ok()?;
        let user = std::env::var("JENKINS_USER").ok()?;
        let token = std::env::var("JENKINS_TOKEN").ok()?;

        let dir = tempfile::tempdir().ok()?;
        let config_dir = dir.path().join(".config").join("jj");
        std::fs::create_dir_all(&config_dir).ok()?;

        // Write the token to a file so token_eval reads it via `cat` without
        // needing to shell-quote arbitrary token string values.
        let token_file = dir.path().join("jenkins-token");
        std::fs::write(&token_file, &token).ok()?;

        let config = format!(
            "default_server = \"test\"\n\n\
             [test]\n\
             host_url = \"{url}\"\n\
             username = \"{user}\"\n\
             token_eval = \"cat '{tok}'\"\n",
            tok = token_file.display(),
        );
        std::fs::write(config_dir.join("config.toml"), config).ok()?;

        let home = dir.path().to_str()?.to_string();
        Some(JenkinsTest { _dir: dir, home })
    }

    // assert_cmd Command for assertions on exit code and stdout.
    fn cmd(&self) -> Command {
        let mut cmd = Command::cargo_bin("jj").unwrap();
        cmd.env("HOME", &self.home);
        cmd
    }

    // std::process::Command for spawning background processes where a Child
    // handle is needed (kill, wait, pid).
    fn std_cmd(&self) -> StdCommand {
        let mut cmd = StdCommand::cargo_bin("jj").unwrap();
        cmd.env("HOME", &self.home);
        cmd
    }
}

// --- --follow-next ---

// Trigger a long-running build so --follow-next finds it already in-flight,
// streams it to completion, and exits with the build's result code.
#[test]
#[serial]
fn follow_next_success() {
    let Some(jt) = JenkinsTest::setup() else {
        return;
    };

    // sleep-job runs for 12 seconds; the 5-second poll cycle has time to
    // catch it while building=true.
    let mut background = jt
        .std_cmd()
        .args(["sleep-job", "-P", "duration=12"])
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .spawn()
        .expect("spawn background sleep-job");

    // Allow Jenkins to pull the build from the queue and begin executing.
    std::thread::sleep(Duration::from_secs(5));

    jt.cmd()
        .args(["sleep-job", "--follow-next"])
        .timeout(Duration::from_secs(60))
        .assert()
        .success()
        .stdout(predicate::str::contains("tick"));

    let _ = background.kill();
    let _ = background.wait();
}

// Start --follow-next before the build exists so it enters its waiting loop,
// then trigger fail-job concurrently.  Verifies exit code 1 for FAILURE.
#[test]
#[serial]
fn follow_next_failure() {
    let Some(jt) = JenkinsTest::setup() else {
        return;
    };
    let home = jt.home.clone();

    // --follow-next runs in a thread so we can trigger the build from the main
    // thread while it waits.
    let handle = std::thread::spawn(move || {
        StdCommand::cargo_bin("jj")
            .unwrap()
            .env("HOME", &home)
            .args(["fail-job", "--follow-next"])
            .status()
            .expect("run --follow-next for fail-job")
    });

    // Give --follow-next time to record its baseline before the build appears.
    std::thread::sleep(Duration::from_secs(2));

    // Trigger fail-job (default duration=10s) in the background; both the
    // enqueue-path jj process and the --follow-next thread will stream it.
    let mut trigger = jt
        .std_cmd()
        .arg("fail-job")
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .spawn()
        .expect("spawn fail-job trigger");

    let status = handle.join().expect("--follow-next thread panicked");
    assert_eq!(status.code(), Some(1), "FAILURE should produce exit code 1");

    let _ = trigger.wait();
}

// Same structure as follow_next_failure but for UNSTABLE → exit code 3.
#[test]
#[serial]
fn follow_next_unstable() {
    let Some(jt) = JenkinsTest::setup() else {
        return;
    };
    let home = jt.home.clone();

    let handle = std::thread::spawn(move || {
        StdCommand::cargo_bin("jj")
            .unwrap()
            .env("HOME", &home)
            .args(["unstable-job", "--follow-next"])
            .status()
            .expect("run --follow-next for unstable-job")
    });

    std::thread::sleep(Duration::from_secs(2));

    let mut trigger = jt
        .std_cmd()
        .arg("unstable-job")
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .spawn()
        .expect("spawn unstable-job trigger");

    let status = handle.join().expect("--follow-next thread panicked");
    assert_eq!(status.code(), Some(3), "UNSTABLE should produce exit code 3");

    let _ = trigger.wait();
}

// --- --follow ---

// Verify --follow exits cleanly (code 0) when interrupted with SIGINT.
#[test]
#[serial]
#[cfg(unix)]
fn follow_exits_on_sigint() {
    let Some(jt) = JenkinsTest::setup() else {
        return;
    };

    let mut child = jt
        .std_cmd()
        .args(["sleep-job", "--follow"])
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .spawn()
        .expect("spawn --follow");

    // Wait for --follow to complete at least one poll cycle (POLL_INTERVAL=5s).
    std::thread::sleep(Duration::from_secs(7));

    let pid = child.id();
    // Safety: sending SIGINT to our own child process is well-defined behavior.
    unsafe {
        libc::kill(pid as libc::pid_t, libc::SIGINT);
    }

    let status = child.wait().expect("wait for --follow child");
    assert!(status.success(), "--follow should exit 0 after SIGINT");
}
