# Preview-env GCS images (RFC-021). One workspace builder, eight runtime
# targets, compiled with BUILD_STACK_VERSION so they detect as green.
variable "TAG" {}
variable "BUILD_STACK_VERSION" { default = "0.15.0" }
variable "RUST_IMAGE_VERSION" {}

group "default" {
  targets = [
    "tfhe-worker",
    "host-listener",
    "gw-listener",
    "sns-worker",
    "tx-sender",
    "zkproof-worker",
    "consensus-detector",
    "upgrade-controller",
  ]
}

target "_gcs" {
  context = "."
  dockerfile = "coprocessor/fhevm-engine/Dockerfile.workspace"
  args = {
    BUILD_STACK_VERSION = BUILD_STACK_VERSION
    RUST_IMAGE_VERSION  = RUST_IMAGE_VERSION
  }
}

target "tfhe-worker" {
  inherits = ["_gcs"]
  target   = "tfhe-worker"
  tags     = ["ghcr.io/zama-ai/fhevm/coprocessor/tfhe-worker:${TAG}"]
}

target "host-listener" {
  inherits = ["_gcs"]
  target   = "host-listener"
  tags     = ["ghcr.io/zama-ai/fhevm/coprocessor/host-listener:${TAG}"]
}

target "gw-listener" {
  inherits = ["_gcs"]
  target   = "gw-listener"
  tags     = ["ghcr.io/zama-ai/fhevm/coprocessor/gw-listener:${TAG}"]
}

target "sns-worker" {
  inherits = ["_gcs"]
  target   = "sns-worker"
  tags     = ["ghcr.io/zama-ai/fhevm/coprocessor/sns-worker:${TAG}"]
}

target "tx-sender" {
  inherits = ["_gcs"]
  target   = "transaction-sender"
  tags     = ["ghcr.io/zama-ai/fhevm/coprocessor/tx-sender:${TAG}"]
}

target "zkproof-worker" {
  inherits = ["_gcs"]
  target   = "zkproof-worker"
  tags     = ["ghcr.io/zama-ai/fhevm/coprocessor/zkproof-worker:${TAG}"]
}

target "consensus-detector" {
  inherits = ["_gcs"]
  target   = "consensus-detector"
  tags     = ["ghcr.io/zama-ai/fhevm/coprocessor/consensus-detector:${TAG}"]
}

target "upgrade-controller" {
  inherits = ["_gcs"]
  target   = "upgrade-controller"
  tags     = ["ghcr.io/zama-ai/fhevm/coprocessor/upgrade-controller:${TAG}"]
}
