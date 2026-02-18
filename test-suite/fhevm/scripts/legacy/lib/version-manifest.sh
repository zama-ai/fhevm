#!/bin/bash

# Canonical version/default manifest.
# Format:
# env_var|default_value|group|display_name|append_build_tag
FHEVM_VERSION_MANIFEST=(
  "GATEWAY_VERSION|v0.11.0-1|FHEVM Contracts|gateway-contracts|true"
  "HOST_VERSION|v0.11.0-1|FHEVM Contracts|host-contracts|true"

  "COPROCESSOR_DB_MIGRATION_VERSION|v0.11.0-1|FHEVM Coprocessor Services|coprocessor/db-migration|true"
  "COPROCESSOR_GW_LISTENER_VERSION|v0.11.0-1|FHEVM Coprocessor Services|coprocessor/gw-listener|true"
  "COPROCESSOR_HOST_LISTENER_VERSION|v0.11.0-1|FHEVM Coprocessor Services|coprocessor/host-listener|true"
  "COPROCESSOR_HOST_LISTENER_VERSION|v0.11.0-1|FHEVM Coprocessor Services|coprocessor/poller|true"
  "COPROCESSOR_TX_SENDER_VERSION|v0.11.0-1|FHEVM Coprocessor Services|coprocessor/tx-sender|true"
  "COPROCESSOR_TFHE_WORKER_VERSION|v0.11.0-1|FHEVM Coprocessor Services|coprocessor/tfhe-worker|true"
  "COPROCESSOR_SNS_WORKER_VERSION|v0.11.0-1|FHEVM Coprocessor Services|coprocessor/sns-worker|true"
  "COPROCESSOR_ZKPROOF_WORKER_VERSION|v0.11.0-1|FHEVM Coprocessor Services|coprocessor/zkproof-worker|true"

  "CONNECTOR_DB_MIGRATION_VERSION|v0.11.0-1|FHEVM KMS Connector Services|kms-connector/db-migration|true"
  "CONNECTOR_GW_LISTENER_VERSION|v0.11.0-1|FHEVM KMS Connector Services|kms-connector/gw-listener|true"
  "CONNECTOR_KMS_WORKER_VERSION|v0.11.0-1|FHEVM KMS Connector Services|kms-connector/kms-worker|true"
  "CONNECTOR_TX_SENDER_VERSION|v0.11.0-1|FHEVM KMS Connector Services|kms-connector/tx-sender|true"

  "TEST_SUITE_VERSION|v0.11.0-1|FHEVM Test Suite|test-suite/e2e|true"

  "CORE_VERSION|v0.13.0-rc.2|External Dependencies|kms-core-service|false"
  "RELAYER_VERSION|v0.9.0-rc.1|External Dependencies|fhevm-relayer|false"
  "RELAYER_MIGRATE_VERSION|v0.9.0-rc.1|External Dependencies|fhevm-relayer-migrate|false"
)

fhevm_export_default_versions() {
  local seen_vars=" "
  local spec var_name default_value current_value

  for spec in "${FHEVM_VERSION_MANIFEST[@]}"; do
    IFS='|' read -r var_name default_value _ _ _ <<< "$spec"

    if [[ "$seen_vars" == *" $var_name "* ]]; then
      continue
    fi
    seen_vars+="$var_name "

    current_value=${!var_name:-}
    if [[ -z "$current_value" ]]; then
      export "$var_name=$default_value"
    else
      export "$var_name=$current_value"
    fi
  done
}

fhevm_print_versions() {
  local log_fn=${1:-echo}
  local build_tag=$2
  local current_group=""
  local spec var_name _default_value group display_name append_build_tag value suffix

  "$log_fn" "FHEVM Stack Versions:"

  for spec in "${FHEVM_VERSION_MANIFEST[@]}"; do
    IFS='|' read -r var_name _default_value group display_name append_build_tag <<< "$spec"

    if [[ "$group" != "$current_group" ]]; then
      "$log_fn" "$group:"
      current_group="$group"
    fi

    value=${!var_name}
    suffix=""
    if [[ "$append_build_tag" == "true" ]]; then
      suffix="$build_tag"
    fi

    "$log_fn" "  $display_name:${value}${suffix}"
  done
}
