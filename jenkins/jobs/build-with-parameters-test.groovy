pipeline {
  agent any
  parameters {
    string(
      name: 'foo',
      defaultValue: 'default-foo',
      description: 'A string parameter.',
    )
    booleanParam(
      name: 'bar',
      defaultValue: false,
      description: 'A boolean parameter.',
    )
  }
  stages {
    stage('Echo') {
      steps {
        echo "foo=${params.foo}"
        echo "bar=${params.bar}"
      }
    }
  }
}
