pipeline {
    agent { dockerfile true }
    stages {
        stage ("Version") {
            steps {
                sh "cargo --version"
                sh "rustc --version"
                sh "rustup component add rustfmt"
            }
        }
        stage ("Build") {
            steps {
                sh "cargo build"
                sh "cargo run"
            }
        }
        stage ("Check") {
            steps {
                sh "rustfmt --check build.rs src/test.rs src/lib.rs"
            }
        }
    }
}