// Marks the build UNSTABLE without failing the pipeline step.  Used to verify
// that jj maps UNSTABLE to exit code 3.
pipeline {
  agent any
  parameters {
    string(
      name: 'duration',
      defaultValue: '10',
      description: 'Seconds to sleep before marking unstable; non-zero lets --follow-next detect the build while it is still running.',
    )
  }
  stages {
    stage('Mark Unstable') {
      steps {
        script {
          def secs = params.duration.toInteger()
          if (secs > 0) {
            sleep secs
          }
          currentBuild.result = 'UNSTABLE'
        }
      }
    }
  }
}
