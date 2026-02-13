#!/bin/bash

# Canonical deployment manifest.
# Format:
# step|compose_component|buildable|description|service_checks
# service_checks is a space-separated list of service_name:state pairs,
# where state is either running or complete.
FHEVM_DEPLOYMENT_MANIFEST=(
  "minio|minio|false|MinIO Services|fhevm-minio:running fhevm-minio-setup:complete"
  "core|core|false|Core Services|kms-core:running"
  "kms-signer||false|KMS signer setup|"
  "database|database|false|Database service|coprocessor-and-kms-db:running"
  "host-node|host-node|true|Host node service|host-node:running"
  "gateway-node|gateway-node|true|Gateway node service|gateway-node:running"
  "coprocessor|coprocessor|true|Coprocessor Services|coprocessor-and-kms-db:running coprocessor-db-migration:complete coprocessor-host-listener:running coprocessor-gw-listener:running coprocessor-tfhe-worker:running coprocessor-zkproof-worker:running coprocessor-sns-worker:running coprocessor-transaction-sender:running"
  "kms-connector|kms-connector|true|KMS Connector Services|coprocessor-and-kms-db:running kms-connector-db-migration:complete kms-connector-gw-listener:running kms-connector-kms-worker:running kms-connector-tx-sender:running"
  "gateway-mocked-payment|gateway-mocked-payment|true|Gateway mocked payment|gateway-deploy-mocked-zama-oft:complete gateway-set-relayer-mocked-payment:complete"
  "gateway-sc|gateway-sc|true|Gateway contracts|gateway-sc-deploy:complete gateway-sc-add-network:complete gateway-sc-trigger-keygen:complete gateway-sc-trigger-crsgen:complete gateway-sc-add-pausers:complete"
  "host-sc|host-sc|true|Host contracts|host-sc-deploy:complete host-sc-add-pausers:complete"
  "relayer|relayer|true|Relayer Services|fhevm-relayer:running"
  "test-suite|test-suite|true|Test Suite E2E Tests|fhevm-test-suite-e2e-debug:running"
)

fhevm_manifest_step_names() {
  local spec step
  for spec in "${FHEVM_DEPLOYMENT_MANIFEST[@]}"; do
    IFS='|' read -r step _ <<< "$spec"
    printf '%s\n' "$step"
  done
}

fhevm_manifest_step_spec() {
  local requested_step=$1
  local spec step

  for spec in "${FHEVM_DEPLOYMENT_MANIFEST[@]}"; do
    IFS='|' read -r step _ <<< "$spec"
    if [[ "$step" == "$requested_step" ]]; then
      printf '%s\n' "$spec"
      return 0
    fi
  done

  return 1
}

fhevm_manifest_step_index() {
  local requested_step=$1
  local index=0
  local step

  while IFS= read -r step; do
    if [[ "$step" == "$requested_step" ]]; then
      printf '%s\n' "$index"
      return 0
    fi
    ((index++))
  done < <(fhevm_manifest_step_names)

  printf '%s\n' "-1"
  return 1
}

fhevm_manifest_step_field() {
  local step=$1
  local field=$2
  local spec
  local name component buildable description services

  if ! spec=$(fhevm_manifest_step_spec "$step"); then
    return 1
  fi

  IFS='|' read -r name component buildable description services <<< "$spec"

  case "$field" in
    component)
      printf '%s\n' "$component"
      ;;
    buildable)
      printf '%s\n' "$buildable"
      ;;
    description)
      printf '%s\n' "$description"
      ;;
    services)
      printf '%s\n' "$services"
      ;;
    *)
      return 1
      ;;
  esac
}

fhevm_manifest_step_components() {
  local seen_components=" "
  local spec component

  for spec in "${FHEVM_DEPLOYMENT_MANIFEST[@]}"; do
    IFS='|' read -r _ component _ _ _ <<< "$spec"

    if [[ -n "$component" && "$seen_components" != *" $component "* ]]; then
      printf '%s\n' "$component"
      seen_components+="$component "
    fi
  done
}

fhevm_manifest_steps_string() {
  local steps=""
  local step

  while IFS= read -r step; do
    if [[ -z "$steps" ]]; then
      steps="$step"
    else
      steps+=" $step"
    fi
  done < <(fhevm_manifest_step_names)

  printf '%s\n' "$steps"
}
