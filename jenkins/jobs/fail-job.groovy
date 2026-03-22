// Always exits with FAILURE.  Used to verify that jj propagates the result
// code correctly (exit 1).
pipeline {
  agent any
  parameters {
    string(
      name: 'duration',
      defaultValue: '10',
      description: 'Seconds to sleep before failing; non-zero lets --follow-next detect the build while it is still running.',
    )
  }
  stages {
    stage('Fail') {
      steps {
        script {
          def secs = params.duration.toInteger()
          if (secs > 0) {
            sleep secs
          }
          error 'This build always fails intentionally.'
        }
      }
    }
  }
}
