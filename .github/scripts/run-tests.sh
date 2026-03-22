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

# Generate Pipeline job config.xml files from the Groovy sources so Jenkins
# loads the current job definitions on startup.
python3 jenkins/apply-jobs.py

# Start Jenkins in the background.
jenkins \
  --enable-future-java \
  --httpListenAddress=localhost \
  --httpPort=11990 \
  --prefix=/ \
  > /tmp/jenkins.log 2>&1 &
JENKINS_PID=$!
echo "Jenkins started with PID $JENKINS_PID"

# Kill Jenkins when the script exits, whether it succeeds or fails.
trap 'kill "$JENKINS_PID" 2>/dev/null || true' EXIT

# Poll until Jenkins responds to HTTP.  JCasC + plugin initialisation can
# take several minutes, so allow up to five minutes total.
echo "Waiting for Jenkins to become available..."
for i in $(seq 1 60); do
  http_code=$(curl -s -o /dev/null -w "%{http_code}" \
    http://localhost:11990/login 2>/dev/null || true)
  if [ "$http_code" = "200" ] || [ "$http_code" = "403" ]; then
    echo "Jenkins ready after $((i * 5))s (HTTP $http_code)"
    break
  fi
  if [ "$i" -eq 60 ]; then
    echo "ERROR: Jenkins did not become ready within 5 minutes"
    echo "--- Jenkins log (last 100 lines) ---"
    tail -100 /tmp/jenkins.log
    exit 1
  fi
  printf "  (%d/60) HTTP %s, waiting 5s...\n" "$i" "$http_code"
  sleep 5
done

# Basic Auth with a password requires a CSRF crumb on POST requests; Basic
# Auth with an API token bypasses CSRF.  Obtain a crumb, use it to mint a
# token, then export the token for the integration tests.
CRUMB_JSON=$(curl -sf -u "admin:admin" \
  "http://localhost:11990/crumbIssuer/api/json")
CRUMB_FIELD=$(echo "$CRUMB_JSON" | jq -r '.crumbRequestField')
CRUMB=$(echo "$CRUMB_JSON" | jq -r '.crumb')

TOKEN_JSON=$(curl -sf -X POST \
  -u "admin:admin" \
  -H "$CRUMB_FIELD: $CRUMB" \
  --data "newTokenName=ci-token" \
  "http://localhost:11990/user/admin/descriptorByName/jenkins.security.ApiTokenProperty/generateNewToken")
API_TOKEN=$(echo "$TOKEN_JSON" | jq -r '.data.tokenValue')
echo "API token generated for ci-token"

# Build the jj binary so assert_cmd can locate it in the Cargo output directory.
cargo build

export JENKINS_URL=http://localhost:11990
export JENKINS_USER=admin
export JENKINS_TOKEN="$API_TOKEN"

cargo test --test integration -- --test-threads=1
