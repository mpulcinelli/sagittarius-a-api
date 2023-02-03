pipeline {
    agent any

    stages {
        stage('Build') {
            steps {
                sh 'rustup default stable'
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