// A long-running job for exercising --follow and --follow-next.  The duration
// parameter controls how long it runs so tests can tune the window.
pipeline {
  agent any
  parameters {
    string(
      name: 'duration',
      defaultValue: '15',
      description: 'How long to run, in seconds.',
    )
  }
  stages {
    stage('Sleep') {
      steps {
        script {
          def secs = params.duration.toInteger()
          for (int i = 1; i <= secs; i++) {
            echo "tick ${i}/${secs}"
            sleep 1
          }
        }
      }
    }
  }
}
