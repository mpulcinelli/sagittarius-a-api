pipeline {
    agent {
        docker {
            image 'rust:latest'
        }
    }
    stages {
        stage('Build') {
            steps {
                sh 'cargo build --release'
            }
        }
        stage('Test') {
            steps {
                sh 'cargo test'
            }
        }
        stage('Publish') {
            steps {
                sh 'cargo publish --token $CARGO_REGISTRY_TOKEN'
            }
        }
    }
}