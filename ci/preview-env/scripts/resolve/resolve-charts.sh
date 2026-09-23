#!/usr/bin/env bash
# Resolve in-repo vs OCI chart paths into GITHUB_ENV.
# Env: GITHUB_WORKSPACE, RUNNER_TEMP, FHEVM_CHARTS_OCI_PREFIX, GITHUB_ENV,
# ANVIL_NODE_CHART_VERSION, CONTRACTS_CHART_VERSION, COPROCESSOR_CHART_VERSION,
# KMS_CONNECTOR_CHART_VERSION, LISTENER_CHART_VERSION.
set -euo pipefail

chart_source() {
  local var="$1" name="$2" version="$3"
  if [[ -z "${version}" ]]; then
    echo "${var}=${GITHUB_WORKSPACE}/charts/${name}" >> "$GITHUB_ENV"
    echo "${name}: charts/${name} from this checkout"
    return
  fi
  local dir="${RUNNER_TEMP}/preview-charts/${name}"
  mkdir -p "${dir}"
  helm pull "${FHEVM_CHARTS_OCI_PREFIX}/${name}" --version "${version}" --untar --untardir "${dir}"
  echo "${var}=${dir}/${name}" >> "$GITHUB_ENV"
  echo "${name}: published OCI chart ${version} (dispatch override)"
}

chart_source ANVIL_NODE_CHART anvil-node "${ANVIL_NODE_CHART_VERSION}"
chart_source CONTRACTS_CHART contracts "${CONTRACTS_CHART_VERSION}"
chart_source COPROCESSOR_CHART coprocessor "${COPROCESSOR_CHART_VERSION}"
chart_source KMS_CONNECTOR_CHART kms-connector "${KMS_CONNECTOR_CHART_VERSION}"
chart_source LISTENER_CHART listener "${LISTENER_CHART_VERSION}"
