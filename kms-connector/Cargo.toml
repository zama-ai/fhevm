[workspace]
resolver = "2"
members = ["crates/*"]

[workspace.package]
authors = ["Zama"]
edition = "2024"
license = "BSD-3-Clause-Clear"
publish = true
version = "0.9.0"

[workspace.dependencies]

#####################################################################
#                       Internal dependencies                       #
#####################################################################
gw-listener.path = "crates/gw-listener"
kms-worker.path = "crates/kms-worker"
tx-sender.path = "crates/tx-sender"
connector-utils.path = "crates/utils"
fhevm_gateway_bindings = { git = "https://github.com/zama-ai/fhevm.git", tag = "v0.9.0", default-features = false }
kms-grpc = { git = "https://github.com/zama-ai/kms.git", rev = "c22a3c5cfc6603114d411d0d3d5c5252b33ad0cb", default-features = true }
bc2wrap = { git = "https://github.com/zama-ai/kms.git", tag = "v0.12.0", default-features = true }
tfhe = "=1.4.0-alpha.3"

#####################################################################
#                       External dependencies                       #
#####################################################################
actix-web = "=4.11.0"
alloy = { version = "=1.0.38", default-features = false, features = [
    "essentials",
    "provider-debug-api",
    "provider-ws",
    "reqwest-rustls-tls",
    "signer-aws",
    "std",
] }
anyhow = { version = "=1.0.98", default-features = false }
async-trait = { version = "=0.1.88", default-features = false }
aws-config = { version = "=1.8.6", default-features = true }
aws-sdk-kms = { version = "=1.86.0", default-features = true }
aws-sdk-s3 = { version = "=1.105.0", default-features = true }
clap = { version = "=4.5.47", default-features = true, features = [
    "cargo",
    "derive",
] }
config = { version = "=0.15.15", default-features = false, features = ["toml"] }
dashmap = { version = "=6.1.0", default-features = false }
futures = { version = "=0.3.31", default-features = false }
git-version = { version = "=0.3.9", default-features = false }
opentelemetry = "=0.30.0"
opentelemetry-otlp = { version = "=0.30.0", features = ["grpc-tonic"] }
opentelemetry_sdk = "=0.30.0"
prometheus = "=0.14.0"
rustls = { version = "=0.23.31", default-features = false, features = [
    "aws-lc-rs",
] }
serde = { version = "=1.0.226", default-features = false, features = [
    "derive",
    "std",
] }
serde_json = { version = "=1.0.145", default-features = false, features = [
    "std",
] }
sha3 = { version = "=0.10.8", default-features = false }
sqlx = { version = "=0.8.6", default-features = false, features = [
    "chrono",
    "derive",
    "macros",
    "postgres",
    "runtime-tokio",
    "tls-rustls",
] }
thiserror = { version = "=2.0.12", default-features = false }
tokio = { version = "=1.47.1", default-features = false, features = [
    "macros",
    "rt-multi-thread",
    "signal",
    "sync",
] }
tokio-util = { version = "=0.7.16", default-features = false }
tokio-stream = { version = "=0.1.17", default-features = false }
tonic = { version = "=0.13.1", default-features = true, features = [
    "tls-ring",
    "tls-native-roots",
] }
tonic-health = { version = "=0.13.1", default-features = false }
tracing = { version = "=0.1.41", default-features = true }
tracing-opentelemetry = "=0.31.0"
tracing-subscriber = { version = "=0.3.20", default-features = true, features = [
    "env-filter",
] }

#####################################################################
#                       Testing dependencies                        #
#####################################################################
connector-tests.path = "tests"
rand = "=0.9.2"
rstest = "=0.26.1"
serial_test = "3.2.0"
tempfile = "=3.20.0"
testcontainers = "=0.24.0"
toml = { version = "=0.9.5", default-features = true }
tracing-test = { version = "=0.2.5", default-features = false }
