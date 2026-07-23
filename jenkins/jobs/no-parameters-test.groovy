// A job with no parameters block at all, exercising the plain /build enqueue
// path jj must use when a run supplies no parameters.  buildWithParameters
// 400s for such a job, so this is the case that formerly could not run.
pipeline {
  agent any
  stages {
    stage('Echo') {
      steps {
        echo 'no-parameters-test ran'
      }
    }
  }
}
