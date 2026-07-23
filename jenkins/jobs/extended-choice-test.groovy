// Exercises a composite multi-select parameter through jj's /build json enqueue
// path.  buildWithParameters drops an extended-choice value (JENKINS-57125),
// while the json payload jj posts to /build carries it.  The
// extended-choice-parameter plugin is deprecated and kept solely for this
// coverage; production jobs should prefer a maintained parameter type.
pipeline {
  agent any
  parameters {
    extendedChoice(
      name: 'colors',
      type: 'PT_CHECKBOX',
      value: 'red,green,blue',
      multiSelectDelimiter: ',',
      quoteValue: false,
      saveJSONParameterToFile: false,
      visibleItemCount: 3,
      description: 'A composite multi-select parameter.'
    )
  }
  stages {
    stage('Echo') {
      steps {
        echo "colors=${params.colors}"
      }
    }
  }
}
