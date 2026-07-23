#!/usr/bin/env bash
# Integration test runner.  Invoked from GitHub Actions inside the Nix dev
# shell:
#
#   nix develop --command bash .github/scripts/run-tests.sh
#
# The dev-shell hook has already:
#   - exported JENKINS_HOME, CASC_JENKINS_CONFIG, and JAVA_OPTS
#   - created JENKINS_HOME/plugins/ and symlinked all plugins into it
set -euo pipefail

# Shared cookie jar so the CSRF crumb fetch and the token-generating POST run in
# one Jenkins HTTP session.  Newer Jenkins binds the crumb to the session, so
# cookie-less requests are rejected with 403.
COOKIE_JAR="$(mktemp)"

# Generate Pipeline job config.xml files from the Groovy sources so Jenkins
# loads the current job definitions on startup.
python3 jenkins/apply-jobs.py

# Start Jenkins in the background.  nixpkgs ships Jenkins as a WAR (no `jenkins`
# command); JENKINS_WAR is exported by the dev-shell hook.
java -jar "$JENKINS_WAR" \
  --enable-future-java \
  --httpListenAddress=localhost \
  --httpPort=11990 \
  --prefix=/ \
  > /tmp/jenkins.log 2>&1 &
JENKINS_PID=$!
echo "Jenkins started with PID $JENKINS_PID"

# Kill Jenkins and remove the cookie jar when the script exits, whether it
# succeeds or fails.
trap 'kill "$JENKINS_PID" 2>/dev/null || true; rm --force "$COOKIE_JAR"' EXIT

# Poll until Jenkins answers an *authenticated* request.  A 200 on /login can
# precede JCasC finishing the security realm, so a crumb fetch right after that
# races the admin user into existence and fails.  Poll the very crumb endpoint
# the token step needs instead: success proves Jenkins is up, admin auth works,
# and the crumb issuer is ready.  JCasC + plugin init can take a few minutes.
echo "Waiting for Jenkins to become available..."
CRUMB_JSON=""
for i in $(seq 1 60); do
  if CRUMB_JSON=$(curl --silent --fail --cookie-jar "$COOKIE_JAR" \
    --user "admin:admin" \
    "http://localhost:11990/crumbIssuer/api/json" 2>/dev/null); then
    echo "Jenkins ready after $((i * 5))s"
    break
  fi
  if [ "$i" -eq 60 ]; then
    echo "ERROR: Jenkins did not become ready within 5 minutes"
    echo "--- Jenkins log (last 100 lines) ---"
    tail -100 /tmp/jenkins.log
    exit 1
  fi
  printf "  (%d/60) not ready yet, waiting 5s...\n" "$i"
  sleep 5
done

# Basic Auth with a password requires a CSRF crumb on POST requests; Basic
# Auth with an API token bypasses CSRF.  Reuse the crumb fetched by the readiness
# poll above to mint a token, then export the token for the integration tests.
CRUMB_FIELD=$(echo "$CRUMB_JSON" | jq -r '.crumbRequestField')
CRUMB=$(echo "$CRUMB_JSON" | jq -r '.crumb')

TOKEN_JSON=$(curl --silent --fail \
  --cookie "$COOKIE_JAR" --cookie-jar "$COOKIE_JAR" \
  --request POST \
  --user "admin:admin" \
  --header "$CRUMB_FIELD: $CRUMB" \
  --data "newTokenName=ci-token" \
  "http://localhost:11990/user/admin/descriptorByName/jenkins.security.ApiTokenProperty/generateNewToken")
API_TOKEN=$(echo "$TOKEN_JSON" | jq -r '.data.tokenValue')
echo "API token generated for ci-token"

# Hydrate the Pipeline jobs.  A job declares its parameters inside the pipeline
# `parameters {}` block, which Jenkins only registers as the job's parameter
# definitions after the job has built once; until then a build silently drops
# any values passed to it.  So trigger one build of each job and wait for it to
# finish before the real tests run.  The API token bypasses CSRF, so these POSTs
# need no crumb.
echo "Hydrating jobs so their parameters register..."
HYDRATE_JOBS="sleep-job fail-job unstable-job build-with-parameters-test no-parameters-test extended-choice-test"
for job in $HYDRATE_JOBS; do
  curl --silent --fail --request POST --user "admin:$API_TOKEN" \
    "http://localhost:11990/job/$job/build" >/dev/null || true
done
# Wait on the build's `result` (a string once it finishes) rather than its
# `building` flag: jq's `//` treats the boolean `false` as empty, so
# `.building // "..."` never reports a completed build.  A null/absent result
# (still building, or no build yet) keeps the loop waiting.
for job in $HYDRATE_JOBS; do
  for _ in $(seq 1 90); do
    result=$(curl --silent --user "admin:$API_TOKEN" \
      "http://localhost:11990/job/$job/lastBuild/api/json?tree=result" \
      2>/dev/null | jq -r '.result // ""')
    [ -n "$result" ] && break
    sleep 2
  done
done
echo "Jobs hydrated."

# Build the jj binary so assert_cmd can locate it in the Cargo output directory.
cargo build

export JENKINS_URL=http://localhost:11990
export JENKINS_USER=admin
export JENKINS_TOKEN="$API_TOKEN"

cargo test --test integration_test -- --test-threads=1
