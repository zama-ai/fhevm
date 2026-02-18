#!/bin/bash

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
SCRIPTS_ROOT="$(cd "${SCRIPT_DIR}/.." && pwd)"

DEFAULT_COMPLETE_SERVICES="fhevm-minio-setup coprocessor-db-migration coprocessor1-db-migration kms-connector-db-migration gateway-deploy-mocked-zama-oft gateway-set-relayer-mocked-payment gateway-sc-deploy gateway-sc-add-network gateway-sc-trigger-keygen gateway-sc-trigger-crsgen gateway-sc-add-pausers host-sc-deploy host-sc-add-pausers"
COMPONENTS=("minio" "core" "database" "host-node" "gateway-node" "coprocessor" "kms-connector" "gateway-mocked-payment" "gateway-sc" "host-sc" "relayer" "test-suite")

TEST_TMP_DIR=""
FIXTURE_ROOT=""
COMMAND_LOG=""

cleanup_fixture() {
  if [[ -n "${TEST_TMP_DIR}" && -d "${TEST_TMP_DIR}" ]]; then
    rm -rf "${TEST_TMP_DIR}"
  fi
  TEST_TMP_DIR=""
  FIXTURE_ROOT=""
  COMMAND_LOG=""
}

setup_fixture() {
  cleanup_fixture

  TEST_TMP_DIR="$(mktemp -d)"
  FIXTURE_ROOT="${TEST_TMP_DIR}/fhevm"
  COMMAND_LOG="${TEST_TMP_DIR}/commands.log"

  mkdir -p "${FIXTURE_ROOT}/scripts/bun"
  mkdir -p "${FIXTURE_ROOT}/env/staging"
  mkdir -p "${FIXTURE_ROOT}/config/relayer"
  mkdir -p "${FIXTURE_ROOT}/docker-compose"
  mkdir -p "${FIXTURE_ROOT}/mock-bin"

  cp "${SCRIPTS_ROOT}/deploy-fhevm-stack.sh" "${FIXTURE_ROOT}/scripts/deploy-fhevm-stack.sh"
  cp "${SCRIPTS_ROOT}/bun/"*.ts "${FIXTURE_ROOT}/scripts/bun/"

  chmod +x "${FIXTURE_ROOT}/scripts/deploy-fhevm-stack.sh"

  cat > "${FIXTURE_ROOT}/scripts/setup-kms-signer-address.sh" <<'SETUP'
#!/bin/bash

echo "setup-kms-signer" >> "${TEST_COMMAND_LOG}"
SETUP
  chmod +x "${FIXTURE_ROOT}/scripts/setup-kms-signer-address.sh"

  echo "relayer: config" > "${FIXTURE_ROOT}/config/relayer/local.yaml"

  local component
  for component in "${COMPONENTS[@]}"; do
    local env_file="${FIXTURE_ROOT}/env/staging/.env.${component}"
    if [[ "${component}" == "coprocessor" ]]; then
      cat > "${env_file}" <<'ENV'
AWS_ENDPOINT_URL=http://minio:9000
ENV
    else
      cat > "${env_file}" <<'ENV'
DUMMY=1
ENV
    fi

    cat > "${FIXTURE_ROOT}/docker-compose/${component}-docker-compose.yml" <<'YAML'
services: {}
YAML
  done

  cat > "${FIXTURE_ROOT}/mock-bin/docker" <<'DOCKER'
#!/bin/bash
set -euo pipefail

echo "docker $*" >> "${TEST_COMMAND_LOG}"

subcommand="${1:-}"
if [[ -n "${subcommand}" ]]; then
  shift
fi

case "${subcommand}" in
  compose)
    compose_args="$*"
    if [[ "${compose_args}" == *"core-docker-compose.yml"* && "${compose_args}" == *" up "* ]]; then
      if [[ -z "${CORE_VERSION:-}" ]]; then
        echo "time=\"2026-02-16T10:18:13+01:00\" level=warning msg=\"The \\\"CORE_VERSION\\\" variable is not set. Defaulting to a blank string.\"" >&2
        echo "unable to get image 'ghcr.io/zama-ai/kms/core-service:': Error response from daemon: invalid reference format" >&2
        exit 1
      fi
    fi
    exit 0
    ;;
  exec)
    while [[ $# -gt 0 ]]; do
      case "$1" in
        -e)
          shift 2
          ;;
        *)
          break
          ;;
      esac
    done

    container="${1:-}"
    shift || true

    if [[ "${1:-}" == "curl" ]]; then
      if [[ "$*" == *"http://relayer:3000/v2/keyurl"* && "${DOCKER_RELAYER_KEYURL_FAIL_HOSTNAME:-false}" == "true" ]]; then
        echo "curl: (6) Could not resolve host: relayer" >&2
        exit 6
      fi

      if [[ "$*" == *"http://relayer:3000/v2/keyurl"* ]]; then
        cat <<'JSON'
{"response":{"fheKeyInfo":[{"fhePublicKey":{"urls":["http://mock-fhevm/key"]}}],"crs":{"2048":{"urls":["http://mock-fhevm/crs"]}}}}
JSON
        exit 0
      fi

      if [[ -n "${DOCKER_RELAYER_IP:-}" && "$*" == *"http://${DOCKER_RELAYER_IP}:3000/v2/keyurl"* ]]; then
        cat <<'JSON'
{"response":{"fheKeyInfo":[{"fhePublicKey":{"urls":["http://mock-fhevm/key"]}}],"crs":{"2048":{"urls":["http://mock-fhevm/crs"]}}}}
JSON
        exit 0
      fi

      if [[ "$*" == *"http://mock-fhevm/key"* || "$*" == *"http://mock-fhevm/crs"* ]]; then
        exit 0
      fi
    fi

    if [[ "${1:-}" == "cast" && "${2:-}" == "call" ]]; then
      if [[ "$*" == *"getActiveKeyId()(uint256)"* ]]; then
        echo "${DOCKER_CAST_ACTIVE_KEY_OUTPUT:-0}"
        exit 0
      fi
      if [[ "$*" == *"getActiveCrsId()(uint256)"* ]]; then
        echo "${DOCKER_CAST_ACTIVE_CRS_OUTPUT:-0}"
        exit 0
      fi
    fi

    exit 0
    ;;
  ps)
    if [[ "${1:-}" == "-a" ]]; then
      shift
      service_filter=""
      label_filter=""
      while [[ $# -gt 0 ]]; do
        case "$1" in
          --filter)
            if [[ "${2:-}" == name=* ]]; then
              service_filter="${2#name=}"
            elif [[ "${2:-}" == label=* ]]; then
              label_filter="${2#label=}"
            fi
            shift 2
            ;;
          --format)
            shift 2
            ;;
          *)
            shift
            ;;
        esac
      done

      if [[ -n "${label_filter}" && -z "${service_filter}" ]]; then
        names_csv="${DOCKER_PROJECT_CONTAINERS:-fhevm-minio,fhevm-minio-setup,kms-core,coprocessor-and-kms-db,host-node,gateway-node,coprocessor-db-migration,coprocessor-gw-listener,coprocessor-host-listener,coprocessor-host-listener-poller,coprocessor-transaction-sender,coprocessor-tfhe-worker,coprocessor-sns-worker,coprocessor-zkproof-worker,coprocessor1-db-migration,coprocessor1-gw-listener,coprocessor1-host-listener,coprocessor1-host-listener-poller,coprocessor1-transaction-sender,coprocessor1-tfhe-worker,coprocessor1-sns-worker,coprocessor1-zkproof-worker,kms-connector-db-migration,kms-connector-gw-listener,kms-connector-kms-worker,kms-connector-tx-sender,gateway-deploy-mocked-zama-oft,gateway-set-relayer-mocked-payment,gateway-sc-deploy,gateway-sc-add-network,gateway-sc-trigger-keygen,gateway-sc-trigger-crsgen,gateway-sc-add-pausers,host-sc-deploy,host-sc-add-pausers,fhevm-relayer,fhevm-relayer-migrate,fhevm-test-suite-e2e-debug}"
        IFS=',' read -r -a names <<< "${names_csv}"
        for name in "${names[@]}"; do
          echo "${name}"
        done
        exit 0
      fi

      service_name="${service_filter%\$}"
      if [[ -n "${DOCKER_MISSING_CONTAINERS:-}" && " ${DOCKER_MISSING_CONTAINERS} " == *" ${service_name} "* ]]; then
        exit 0
      fi

      echo "${service_name}-container"
      exit 0
    fi

    if [[ "${1:-}" == "--filter" && "${2:-}" == publish=* ]]; then
      port="${2#publish=}"
      key="DOCKER_PUBLISH_${port}"
      names_csv="${!key:-}"
      if [[ -n "${names_csv}" ]]; then
        IFS=',' read -r -a names <<< "${names_csv}"
        for name in "${names[@]}"; do
          echo "${name}"
        done
      fi
      exit 0
    fi

    names_csv="${DOCKER_RUNNING_NAMES:-}"
    if [[ -n "${names_csv}" ]]; then
      IFS=',' read -r -a names <<< "${names_csv}"
      for name in "${names[@]}"; do
        echo "${name}"
      done
    fi
    exit 0
    ;;
  volume)
    action="${1:-}"
    shift || true
    if [[ "${action}" == "ls" ]]; then
      filter_name=""
      while [[ $# -gt 0 ]]; do
        case "$1" in
          --filter)
            if [[ "${2:-}" == name=* ]]; then
              filter_name="${2#name=}"
            fi
            shift 2
            ;;
          --format)
            shift 2
            ;;
          *)
            shift
            ;;
        esac
      done

      names_csv="${DOCKER_VOLUME_NAMES:-fhevm_minio_secrets}"
      IFS=',' read -r -a names <<< "${names_csv}"
      exact="${filter_name#^}"
      exact="${exact%\$}"

      for name in "${names[@]}"; do
        if [[ -z "${exact}" || "${name}" == "${exact}" ]]; then
          echo "${name}"
        fi
      done
      exit 0
    fi
    exit 0
    ;;
  inspect)
    if [[ "${1:-}" == "-f" ]]; then
      format="${2:-}"
      target="${3:-}"
      if [[ "${format}" == "{{range .NetworkSettings.Networks}}{{.IPAddress}}{{end}}" ]]; then
        if [[ "${target}" == *"relayer"* ]]; then
          echo "${DOCKER_RELAYER_IP:-172.20.0.9}"
        else
          echo "${DOCKER_MINIO_IP:-172.20.0.2}"
        fi
        exit 0
      fi
    fi

    if [[ "${1:-}" == "--format" ]]; then
      format="${2:-}"
      target="${3:-}"
      service_name="${target%-container}"
      fail_service="${DOCKER_FAIL_SERVICE:-}"
      fail_exit_code="${DOCKER_FAIL_EXIT_CODE:-1}"
      fail_oom="${DOCKER_FAIL_OOM:-false}"
      complete_services=" ${DOCKER_COMPLETE_SERVICES:-} "

      case "${format}" in
        "{{.State.Status}}")
          if [[ -n "${fail_service}" && "${service_name}" == "${fail_service}" ]]; then
            echo "exited"
          elif [[ "${service_name}" =~ ^coprocessor[0-9]+-db-migration$ ]]; then
            echo "exited"
          elif [[ "${complete_services}" == *" ${service_name} "* ]]; then
            echo "exited"
          else
            echo "running"
          fi
          ;;
        "{{.State.ExitCode}}")
          if [[ -n "${fail_service}" && "${service_name}" == "${fail_service}" ]]; then
            echo "${fail_exit_code}"
          elif [[ "${service_name}" =~ ^coprocessor[0-9]+-db-migration$ ]]; then
            echo "0"
          elif [[ "${complete_services}" == *" ${service_name} "* ]]; then
            echo "0"
          else
            echo "0"
          fi
          ;;
        "{{.State.OOMKilled}}")
          if [[ -n "${fail_service}" && "${service_name}" == "${fail_service}" && "${fail_oom}" == "true" ]]; then
            echo "true"
          else
            echo "false"
          fi
          ;;
        *)
          echo ""
          ;;
      esac
      exit 0
    fi

    exit 0
    ;;
  logs)
    target="${1:-}"
    service_name="${target%-container}"
    key="DOCKER_LOGS_${service_name//-/_}"
    echo "${!key:-mock logs for ${service_name}}"
    exit 0
    ;;
  *)
    exit 0
    ;;
esac
DOCKER
  chmod +x "${FIXTURE_ROOT}/mock-bin/docker"

  cat > "${FIXTURE_ROOT}/mock-bin/sleep" <<'SLEEP'
#!/bin/bash
exit 0
SLEEP
  chmod +x "${FIXTURE_ROOT}/mock-bin/sleep"

  cat > "${FIXTURE_ROOT}/mock-bin/curl" <<'CURL'
#!/bin/bash
set -euo pipefail

url=""
for arg in "$@"; do
  if [[ "$arg" =~ ^https?:// ]]; then
    url="$arg"
  fi
done

if [[ "$url" == *"/api/public/dashboards/"*"/panels/"*"/query"* ]]; then
  panel_id="${url#*/panels/}"
  panel_id="${panel_id%%/*}"
  panel_key="MOCK_GRAFANA_PANEL_QUERY_JSON_${panel_id}"
  panel_payload="${!panel_key:-${MOCK_GRAFANA_PANEL_QUERY_JSON:-}}"
  if [[ -z "${panel_payload}" ]]; then
    exit 1
  fi
  echo "${panel_payload}"
  exit 0
fi

if [[ "$url" == *"/api/public/dashboards/"* ]]; then
  if [[ -n "${MOCK_GRAFANA_DASHBOARD_JSON:-}" ]]; then
    echo "${MOCK_GRAFANA_DASHBOARD_JSON}"
    exit 0
  fi
  exit 1
fi

if [[ "$url" == *"/api/services"* ]]; then
  if [[ "${MOCK_JAEGER_SERVICES_STATUS:-0}" != "0" ]]; then
    exit "${MOCK_JAEGER_SERVICES_STATUS}"
  fi

  echo "${MOCK_JAEGER_SERVICES_JSON:-{\"data\":[]}}"
  exit 0
fi

echo "${MOCK_CURL_DEFAULT_JSON:-{\"data\":[]}}"
CURL
  chmod +x "${FIXTURE_ROOT}/mock-bin/curl"

  cat > "${FIXTURE_ROOT}/mock-bin/cast" <<'CAST'
#!/bin/bash
set -euo pipefail

echo "cast $*" >> "${TEST_COMMAND_LOG}"

if [[ "${1:-}" == "--version" ]]; then
  echo "cast 1.0.0"
  exit 0
fi

if [[ "${1:-}" != "wallet" ]]; then
  exit 1
fi

kind="${2:-}"
index="0"
while [[ $# -gt 0 ]]; do
  if [[ "$1" == "--mnemonic-index" ]]; then
    index="${2:-0}"
    break
  fi
  shift
done

if [[ "${kind}" == "address" ]]; then
  printf '0x%040x\n' "${index}"
  exit 0
fi

if [[ "${kind}" == "private-key" ]]; then
  printf '0x%064x\n' "${index}"
  exit 0
fi

exit 1
CAST
  chmod +x "${FIXTURE_ROOT}/mock-bin/cast"

  : > "${COMMAND_LOG}"
}

set_network_profile_api_fixture() {
  export FHEVM_GRAFANA_PUBLIC_VERSIONS_URL="https://zamablockchain.grafana.net/public-dashboards/mocktoken"
  export MOCK_GRAFANA_DASHBOARD_JSON='{"dashboard":{"panels":[{"id":11,"title":"Testnet Currently Deployed Versions"},{"id":22,"title":"Mainnet Currently Deployed Versions"}]}}'
  export MOCK_GRAFANA_PANEL_QUERY_JSON_11='{"results":{"A":{"frames":[{"schema":{"fields":[{"type":"number","labels":{"container_name":"coprocessor-gw-listener","image_registry":"ghcr.io","image_repository":"zama-ai/fhevm/coprocessor/gw-listener","image_tag":"v0.10.10"}}]}},{"schema":{"fields":[{"type":"number","labels":{"container_name":"coprocessor-host-listener","image_registry":"ghcr.io","image_repository":"zama-ai/fhevm/coprocessor/host-listener","image_tag":"v0.10.10"}}]}},{"schema":{"fields":[{"type":"number","labels":{"container_name":"coprocessor-tx-sender","image_registry":"ghcr.io","image_repository":"zama-ai/fhevm/coprocessor/tx-sender","image_tag":"v0.10.10"}}]}},{"schema":{"fields":[{"type":"number","labels":{"container_name":"kms-connector-db-migration","image_registry":"hub.zama.org","image_repository":"zama-protocol/zama-ai/fhevm/kms-connector/db-migration","image_tag":"v0.10.8"}}]}},{"schema":{"fields":[{"type":"number","labels":{"container_name":"kms-connector-gw-listener","image_registry":"hub.zama.org","image_repository":"zama-protocol/zama-ai/fhevm/kms-connector/gw-listener","image_tag":"v0.10.8"}}]}},{"schema":{"fields":[{"type":"number","labels":{"container_name":"kms-connector-kms-worker","image_registry":"hub.zama.org","image_repository":"zama-protocol/zama-ai/fhevm/kms-connector/kms-worker","image_tag":"v0.10.8"}}]}},{"schema":{"fields":[{"type":"number","labels":{"container_name":"kms-connector-tx-sender","image_registry":"hub.zama.org","image_repository":"zama-protocol/zama-ai/fhevm/kms-connector/tx-sender","image_tag":"v0.10.8"}}]}},{"schema":{"fields":[{"type":"number","labels":{"container_name":"kms-core-enclave","image_registry":"hub.zama.org","image_repository":"zama-protocol/zama-ai/kms/core-service-enclave","image_tag":"v0.12.7"}}]}}]}}}'
  export MOCK_GRAFANA_PANEL_QUERY_JSON_22='{"results":{"A":{"frames":[{"schema":{"fields":[{"type":"number","labels":{"container_name":"coprocessor-gw-listener","image_registry":"hub.zama.org","image_repository":"ghcr/zama-ai/fhevm/coprocessor/gw-listener","image_tag":"v0.10.5"}}]}},{"schema":{"fields":[{"type":"number","labels":{"container_name":"coprocessor-host-listener","image_registry":"hub.zama.org","image_repository":"internal/zama-ai/fhevm/coprocessor/host-listener","image_tag":"v0.10.9"}}]}},{"schema":{"fields":[{"type":"number","labels":{"container_name":"coprocessor-tx-sender","image_registry":"hub.zama.org","image_repository":"internal/zama-ai/fhevm/coprocessor/tx-sender","image_tag":"v0.10.5"}}]}},{"schema":{"fields":[{"type":"number","labels":{"container_name":"kms-connector-db-migration","image_registry":"hub.zama.org","image_repository":"zama-protocol/zama-ai/fhevm/kms-connector/db-migration","image_tag":"v0.10.7"}}]}},{"schema":{"fields":[{"type":"number","labels":{"container_name":"kms-connector-gw-listener","image_registry":"hub.zama.org","image_repository":"zama-protocol/zama-ai/fhevm/kms-connector/gw-listener","image_tag":"v0.10.8"}}]}},{"schema":{"fields":[{"type":"number","labels":{"container_name":"kms-connector-kms-worker","image_registry":"hub.zama.org","image_repository":"zama-protocol/zama-ai/fhevm/kms-connector/kms-worker","image_tag":"v0.10.8"}}]}},{"schema":{"fields":[{"type":"number","labels":{"container_name":"kms-connector-tx-sender","image_registry":"hub.zama.org","image_repository":"zama-protocol/zama-ai/fhevm/kms-connector/tx-sender","image_tag":"v0.10.8"}}]}},{"schema":{"fields":[{"type":"number","labels":{"container_name":"kms-core-enclave","image_registry":"hub.zama.org","image_repository":"zama-protocol/zama-ai/kms/core-service-enclave","image_tag":"v0.12.7"}}]}}]}}}'
}

unset_network_profile_api_fixture() {
  unset FHEVM_GRAFANA_PUBLIC_VERSIONS_URL
  unset MOCK_GRAFANA_DASHBOARD_JSON
  unset MOCK_GRAFANA_PANEL_QUERY_JSON_11
  unset MOCK_GRAFANA_PANEL_QUERY_JSON_22
}

run_deploy() {
  local output_file=$1
  shift

  (
    cd "${FIXTURE_ROOT}/scripts"
    export PATH="${FIXTURE_ROOT}/mock-bin:${PATH}"
    export TEST_COMMAND_LOG="${COMMAND_LOG}"
    export DOCKER_COMPLETE_SERVICES="${DOCKER_COMPLETE_SERVICES:-${DEFAULT_COMPLETE_SERVICES}}"
    export DOCKER_MINIO_IP="${DOCKER_MINIO_IP:-172.20.0.2}"
    export DOCKER_RUNNING_NAMES="${DOCKER_RUNNING_NAMES:-fhevm-relayer,fhevm-test-suite-e2e-debug}"
    # Keep fixture container names deterministic for mock completion checks.
    export FHEVM_DOCKER_PROJECT="fhevm"

    ./deploy-fhevm-stack.sh "$@"
  ) > "${output_file}" 2>&1
}

run_cli() {
  local output_file=$1
  shift

  (
    cd "${FIXTURE_ROOT}"
    export PATH="${FIXTURE_ROOT}/mock-bin:${PATH}"
    export TEST_COMMAND_LOG="${COMMAND_LOG}"
    export DOCKER_COMPLETE_SERVICES="${DOCKER_COMPLETE_SERVICES:-${DEFAULT_COMPLETE_SERVICES}}"
    export DOCKER_MINIO_IP="${DOCKER_MINIO_IP:-172.20.0.2}"
    export DOCKER_RUNNING_NAMES="${DOCKER_RUNNING_NAMES:-fhevm-relayer,fhevm-test-suite-e2e-debug}"
    # Keep fixture container names deterministic for mock completion checks.
    export FHEVM_DOCKER_PROJECT="fhevm"

    bun scripts/bun/cli.ts "$@"
  ) > "${output_file}" 2>&1
}

run_cli_ok() {
  local label=$1
  shift
  local output_file="${TEST_TMP_DIR}/${label}.out"
  if ! run_cli "${output_file}" "$@"; then
    echo "Command should succeed: bun scripts/bun/cli.ts $*" >&2
    cat "${output_file}" >&2
    return 1
  fi
}

run_cli_fail_code() {
  local label=$1
  local code=$2
  shift 2
  local output_file="${TEST_TMP_DIR}/${label}.out"
  if run_cli "${output_file}" "$@"; then
    echo "Command should fail: bun scripts/bun/cli.ts $*" >&2
    cat "${output_file}" >&2
    return 1
  fi
  assert_error_code "${output_file}" "${code}"
}

assert_contains() {
  local file=$1
  local pattern=$2

  if ! grep -Fq "${pattern}" "${file}"; then
    echo "Assertion failed: expected '${pattern}' in ${file}" >&2
    echo "---- ${file} ----" >&2
    cat "${file}" >&2
    return 1
  fi
}

assert_not_contains() {
  local file=$1
  local pattern=$2

  if grep -Fq "${pattern}" "${file}"; then
    echo "Assertion failed: did not expect '${pattern}' in ${file}" >&2
    echo "---- ${file} ----" >&2
    cat "${file}" >&2
    return 1
  fi
}

assert_error_code() {
  local file=$1
  local code=$2
  assert_contains "${file}" "ERROR_CODE=${code}"
}

line_number() {
  local file=$1
  local pattern=$2

  grep -Fn "${pattern}" "${file}" | head -n1 | cut -d: -f1
}

assert_order() {
  local file=$1
  local first=$2
  local second=$3
  local first_line
  local second_line

  first_line="$(line_number "${file}" "${first}")"
  second_line="$(line_number "${file}" "${second}")"

  if [[ -z "${first_line}" || -z "${second_line}" ]]; then
    echo "Assertion failed: unable to find ordered patterns in ${file}" >&2
    echo "first=${first}" >&2
    echo "second=${second}" >&2
    return 1
  fi

  if (( first_line >= second_line )); then
    echo "Assertion failed: '${first}' should appear before '${second}'" >&2
    return 1
  fi
}

test_default_flow_and_env_patch() {
  setup_fixture

  local output_file="${TEST_TMP_DIR}/default.out"
  if ! run_deploy "${output_file}"; then
    echo "Default deploy should succeed" >&2
    cat "${output_file}" >&2
    return 1
  fi

  assert_contains "${COMMAND_LOG}" "minio-docker-compose.yml up -d"
  assert_contains "${COMMAND_LOG}" "core-docker-compose.yml up -d"
  assert_contains "${COMMAND_LOG}" "database-docker-compose.yml up -d"
  assert_contains "${COMMAND_LOG}" "host-node-docker-compose.yml up -d"
  assert_contains "${COMMAND_LOG}" "gateway-node-docker-compose.yml up -d"
  assert_contains "${COMMAND_LOG}" "coprocessor-docker-compose.yml up -d"
  assert_contains "${COMMAND_LOG}" "kms-connector-docker-compose.yml up -d"
  assert_contains "${COMMAND_LOG}" "gateway-mocked-payment-docker-compose.yml up -d"
  assert_contains "${COMMAND_LOG}" "gateway-sc-docker-compose.yml up --force-recreate -d gateway-sc-deploy"
  assert_contains "${COMMAND_LOG}" "host-sc-docker-compose.yml up --force-recreate -d host-sc-deploy"
  assert_contains "${COMMAND_LOG}" "relayer-docker-compose.yml up -d"
  assert_contains "${COMMAND_LOG}" "test-suite-docker-compose.yml up -d"
  assert_contains "${COMMAND_LOG}" "setup-kms-signer"

  assert_order "${COMMAND_LOG}" "minio-docker-compose.yml up -d" "core-docker-compose.yml up -d"
  assert_order "${COMMAND_LOG}" "core-docker-compose.yml up -d" "database-docker-compose.yml up -d"
  assert_order "${COMMAND_LOG}" "database-docker-compose.yml up -d" "host-node-docker-compose.yml up -d"
  assert_order "${COMMAND_LOG}" "docker inspect -f {{range .NetworkSettings.Networks}}{{.IPAddress}}{{end}} fhevm-minio" "coprocessor-docker-compose.yml up -d"

  assert_contains "${FIXTURE_ROOT}/env/staging/.env.coprocessor.local" "AWS_ENDPOINT_URL=http://172.20.0.2:9000"
  assert_contains "${FIXTURE_ROOT}/env/staging/.env.coprocessor.local" "OTEL_EXPORTER_OTLP_ENDPOINT=http://jaeger:4317"
  assert_not_contains "${output_file}" "CORE_VERSION variable is not set"

  cleanup_fixture
}

test_resume_preserves_prior_steps_and_restarts_tail() {
  setup_fixture

  local output_file="${TEST_TMP_DIR}/resume.out"
  export DOCKER_RUNNING_NAMES="fhevm-minio"

  if ! run_deploy "${output_file}" --resume kms-connector; then
    echo "Resume deploy should succeed" >&2
    cat "${output_file}" >&2
    return 1
  fi

  assert_not_contains "${COMMAND_LOG}" "minio-docker-compose.yml up -d"
  assert_not_contains "${COMMAND_LOG}" "core-docker-compose.yml up -d"
  assert_not_contains "${COMMAND_LOG}" "database-docker-compose.yml up -d"
  assert_not_contains "${COMMAND_LOG}" "host-node-docker-compose.yml up -d"
  assert_not_contains "${COMMAND_LOG}" "gateway-node-docker-compose.yml up -d"
  assert_not_contains "${COMMAND_LOG}" "coprocessor-docker-compose.yml up -d"

  assert_contains "${COMMAND_LOG}" "kms-connector-docker-compose.yml up -d"
  assert_contains "${COMMAND_LOG}" "gateway-mocked-payment-docker-compose.yml up -d"
  assert_contains "${COMMAND_LOG}" "gateway-sc-docker-compose.yml up --force-recreate -d gateway-sc-deploy"
  assert_contains "${COMMAND_LOG}" "host-sc-docker-compose.yml up --force-recreate -d host-sc-deploy"
  assert_contains "${COMMAND_LOG}" "relayer-docker-compose.yml up -d"
  assert_contains "${COMMAND_LOG}" "test-suite-docker-compose.yml up -d"

  assert_contains "${COMMAND_LOG}" "test-suite-docker-compose.yml down -v"
  assert_contains "${COMMAND_LOG}" "relayer-docker-compose.yml down -v"
  assert_contains "${COMMAND_LOG}" "host-sc-docker-compose.yml down -v"
  assert_contains "${COMMAND_LOG}" "gateway-sc-docker-compose.yml down -v"
  assert_contains "${COMMAND_LOG}" "gateway-mocked-payment-docker-compose.yml down -v"
  assert_contains "${COMMAND_LOG}" "kms-connector-docker-compose.yml down -v"
  assert_not_contains "${COMMAND_LOG}" "coprocessor-docker-compose.yml down -v"

  assert_order "${COMMAND_LOG}" "test-suite-docker-compose.yml down -v" "kms-connector-docker-compose.yml down -v"
  assert_order "${COMMAND_LOG}" "docker inspect -f {{range .NetworkSettings.Networks}}{{.IPAddress}}{{end}} fhevm-minio" "kms-connector-docker-compose.yml up -d"

  unset DOCKER_RUNNING_NAMES
  cleanup_fixture
}

test_multicoprocessor_resume_forces_minio_reset() {
  setup_fixture

  cat > "${FIXTURE_ROOT}/env/staging/.env.gateway-sc" <<'ENV'
MNEMONIC=test test test test test test test test test test test junk
COPROCESSOR_THRESHOLD=1
NUM_COPROCESSORS=1
COPROCESSOR_TX_SENDER_ADDRESS_0=0x1111111111111111111111111111111111111111
COPROCESSOR_SIGNER_ADDRESS_0=0x1111111111111111111111111111111111111111
COPROCESSOR_S3_BUCKET_URL_0=http://minio:9000/ct128
ENV

  cat > "${FIXTURE_ROOT}/env/staging/.env.host-sc" <<'ENV'
COPROCESSOR_THRESHOLD=1
NUM_COPROCESSORS=1
COPROCESSOR_SIGNER_ADDRESS_0=0x1111111111111111111111111111111111111111
ENV

  cat > "${FIXTURE_ROOT}/env/staging/.env.coprocessor" <<'ENV'
AWS_ENDPOINT_URL=http://minio:9000
DATABASE_URL="postgresql://${POSTGRES_USER}:${POSTGRES_PASSWORD}@db:5432/coprocessor"
TX_SENDER_PRIVATE_KEY=0xaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa
ENV

  local output_file="${TEST_TMP_DIR}/multicoprocessor-resume-forced-minio.out"
  if ! run_deploy "${output_file}" --resume coprocessor --coprocessors 2 --coprocessor-threshold 2; then
    echo "Deploy with resume+multicoprocessor should succeed" >&2
    cat "${output_file}" >&2
    return 1
  fi

  assert_contains "${output_file}" "Forcing resume from 'minio'."
  assert_contains "${COMMAND_LOG}" "minio-docker-compose.yml down -v"
  assert_contains "${COMMAND_LOG}" "minio-docker-compose.yml up -d"
  cleanup_fixture
}

test_only_runs_single_step() {
  setup_fixture

  local output_file="${TEST_TMP_DIR}/only.out"
  export DOCKER_RUNNING_NAMES="fhevm-minio,kms-core,coprocessor-and-kms-db,host-node,gateway-node"
  if ! run_deploy "${output_file}" --only coprocessor; then
    echo "Only-step deploy should succeed" >&2
    cat "${output_file}" >&2
    return 1
  fi

  assert_contains "${COMMAND_LOG}" "coprocessor-docker-compose.yml down -v"
  assert_contains "${COMMAND_LOG}" "coprocessor-docker-compose.yml up -d"
  assert_not_contains "${COMMAND_LOG}" "host-node-docker-compose.yml up -d"
  assert_not_contains "${COMMAND_LOG}" "gateway-node-docker-compose.yml up -d"
  assert_not_contains "${COMMAND_LOG}" "kms-connector-docker-compose.yml up -d"
  assert_not_contains "${COMMAND_LOG}" "setup-kms-signer"

  unset DOCKER_RUNNING_NAMES
  cleanup_fixture
}

test_build_flag_applies_only_to_buildable_steps() {
  setup_fixture

  local output_file="${TEST_TMP_DIR}/build.out"
  if ! run_deploy "${output_file}" --build; then
    echo "Build deploy should succeed" >&2
    cat "${output_file}" >&2
    return 1
  fi

  assert_contains "${COMMAND_LOG}" "minio-docker-compose.yml up -d"
  assert_contains "${COMMAND_LOG}" "core-docker-compose.yml up -d"
  assert_contains "${COMMAND_LOG}" "database-docker-compose.yml up -d"
  assert_not_contains "${COMMAND_LOG}" "database-docker-compose.yml up --build -d"

  assert_contains "${COMMAND_LOG}" "host-node-docker-compose.yml up --build -d"
  assert_contains "${COMMAND_LOG}" "gateway-node-docker-compose.yml up --build -d"
  assert_contains "${COMMAND_LOG}" "coprocessor-docker-compose.yml up --build -d"

  cleanup_fixture
}

test_local_flag_implies_build_for_buildable_steps() {
  setup_fixture

  local output_file="${TEST_TMP_DIR}/local-implies-build.out"
  if ! run_deploy "${output_file}" --local; then
    echo "Local deploy should succeed" >&2
    cat "${output_file}" >&2
    return 1
  fi

  assert_contains "${COMMAND_LOG}" "minio-docker-compose.yml up -d"
  assert_contains "${COMMAND_LOG}" "core-docker-compose.yml up -d"
  assert_contains "${COMMAND_LOG}" "database-docker-compose.yml up -d"
  assert_not_contains "${COMMAND_LOG}" "database-docker-compose.yml up --build -d"

  assert_contains "${COMMAND_LOG}" "host-node-docker-compose.yml up --build -d"
  assert_contains "${COMMAND_LOG}" "gateway-node-docker-compose.yml up --build -d"
  assert_contains "${COMMAND_LOG}" "coprocessor-docker-compose.yml up --build -d"

  cleanup_fixture
}

test_oom_failure_is_actionable() {
  setup_fixture

  local output_file="${TEST_TMP_DIR}/oom.out"
  export DOCKER_FAIL_SERVICE="coprocessor-tfhe-worker"
  export DOCKER_FAIL_EXIT_CODE="137"
  export DOCKER_FAIL_OOM="true"

  if run_deploy "${output_file}"; then
    echo "OOM scenario should fail" >&2
    cat "${output_file}" >&2
    return 1
  fi

  assert_error_code "${output_file}" "E_SERVICE_OOM"

  unset DOCKER_FAIL_SERVICE
  unset DOCKER_FAIL_EXIT_CODE
  unset DOCKER_FAIL_OOM
  cleanup_fixture
}

test_key_bootstrap_failure_is_actionable() {
  setup_fixture

  local output_file="${TEST_TMP_DIR}/key-bootstrap.out"
  export DOCKER_FAIL_SERVICE="gateway-sc-trigger-keygen"
  export DOCKER_FAIL_EXIT_CODE="1"
  export DOCKER_LOGS_gateway_sc_trigger_keygen="execution reverted: CrsNotGenerated(1)"

  if run_deploy "${output_file}"; then
    echo "Key-bootstrap scenario should fail" >&2
    cat "${output_file}" >&2
    return 1
  fi

  assert_error_code "${output_file}" "E_KEY_BOOTSTRAP_NOT_READY"

  unset DOCKER_FAIL_SERVICE
  unset DOCKER_FAIL_EXIT_CODE
  unset DOCKER_LOGS_gateway_sc_trigger_keygen
  cleanup_fixture
}

test_strict_otel_requires_jaeger() {
  setup_fixture

  local output_file="${TEST_TMP_DIR}/strict-otel.out"
  if run_deploy "${output_file}" --strict-otel; then
    echo "Strict OTEL deploy should fail when Jaeger is absent" >&2
    cat "${output_file}" >&2
    return 1
  fi

  assert_error_code "${output_file}" "E_OTEL_JAEGER_REQUIRED"
  cleanup_fixture
}

test_deploy_network_profile_applies_versions() {
  setup_fixture
  set_network_profile_api_fixture

  local output_file="${TEST_TMP_DIR}/network-mainnet.out"
  if ! run_deploy "${output_file}" --network mainnet; then
    echo "Deploy with --network mainnet should succeed" >&2
    cat "${output_file}" >&2
    return 1
  fi

  assert_contains "${FIXTURE_ROOT}/env/staging/.env.gateway-sc.local" "COPROCESSOR_GW_LISTENER_VERSION=v0.10.5"
  assert_contains "${FIXTURE_ROOT}/env/staging/.env.gateway-sc.local" "COPROCESSOR_HOST_LISTENER_VERSION=v0.10.9"
  assert_contains "${FIXTURE_ROOT}/env/staging/.env.gateway-sc.local" "COPROCESSOR_TX_SENDER_VERSION=v0.10.5"
  assert_contains "${FIXTURE_ROOT}/env/staging/.env.gateway-sc.local" "CONNECTOR_DB_MIGRATION_VERSION=v0.10.7"
  assert_contains "${FIXTURE_ROOT}/env/staging/.env.gateway-sc.local" "CORE_VERSION=v0.12.7"

  unset_network_profile_api_fixture
  cleanup_fixture
}

test_deploy_network_profile_rejects_invalid_value() {
  setup_fixture

  local output_file="${TEST_TMP_DIR}/network-invalid.out"
  if run_deploy "${output_file}" --network foo; then
    echo "Deploy with invalid --network value should fail" >&2
    cat "${output_file}" >&2
    return 1
  fi

  assert_error_code "${output_file}" "E_USAGE"

  cleanup_fixture
}

test_quoted_otel_endpoint_is_accepted() {
  setup_fixture

  cat > "${FIXTURE_ROOT}/env/staging/.env.coprocessor" <<'ENV'
AWS_ENDPOINT_URL=http://minio:9000
OTEL_EXPORTER_OTLP_ENDPOINT="http://jaeger:4317"
ENV

  local output_file="${TEST_TMP_DIR}/quoted-otel.out"
  if ! run_deploy "${output_file}"; then
    echo "Deploy should accept quoted OTEL endpoint value" >&2
    cat "${output_file}" >&2
    return 1
  fi

  assert_contains "${FIXTURE_ROOT}/env/staging/.env.coprocessor.local" "OTEL_EXPORTER_OTLP_ENDPOINT=\"http://jaeger:4317\""
  cleanup_fixture
}

test_multicoprocessor_flags_are_validated() {
  setup_fixture

  local output_file="${TEST_TMP_DIR}/multicoprocessor-invalid-threshold.out"
  if run_deploy "${output_file}" --coprocessors 2 --coprocessor-threshold 3; then
    echo "Deploy should reject threshold larger than coprocessor count" >&2
    cat "${output_file}" >&2
    return 1
  fi

  assert_error_code "${output_file}" "E_USAGE"
  cleanup_fixture
}

test_multicoprocessor_env_and_extra_instances() {
  setup_fixture

  cat > "${FIXTURE_ROOT}/env/staging/.env.gateway-sc" <<'ENV'
MNEMONIC=test test test test test test test test test test test junk
COPROCESSOR_THRESHOLD=1
NUM_COPROCESSORS=1
COPROCESSOR_TX_SENDER_ADDRESS_0=0x1111111111111111111111111111111111111111
COPROCESSOR_SIGNER_ADDRESS_0=0x1111111111111111111111111111111111111111
COPROCESSOR_S3_BUCKET_URL_0=http://minio:9000/ct128
ENV

  cat > "${FIXTURE_ROOT}/env/staging/.env.host-sc" <<'ENV'
COPROCESSOR_THRESHOLD=1
NUM_COPROCESSORS=1
COPROCESSOR_SIGNER_ADDRESS_0=0x1111111111111111111111111111111111111111
ENV

  cat > "${FIXTURE_ROOT}/env/staging/.env.coprocessor" <<'ENV'
AWS_ENDPOINT_URL=http://minio:9000
DATABASE_URL="postgresql://${POSTGRES_USER}:${POSTGRES_PASSWORD}@db:5432/coprocessor"
TX_SENDER_PRIVATE_KEY=0xaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa
ENV

  local output_file="${TEST_TMP_DIR}/multicoprocessor-2-2.out"
  if ! run_deploy "${output_file}" --coprocessors 2 --coprocessor-threshold 2; then
    echo "Deploy with multicoprocessor topology should succeed" >&2
    cat "${output_file}" >&2
    return 1
  fi

  assert_contains "${output_file}" "Coprocessor topology: n=2 threshold=2"
  assert_contains "${COMMAND_LOG}" "cast wallet address --mnemonic test test test test test test test test test test test junk --mnemonic-index 5"
  assert_contains "${COMMAND_LOG}" "cast wallet private-key --mnemonic test test test test test test test test test test test junk --mnemonic-index 8"
  assert_contains "${COMMAND_LOG}" "coprocessor-1.generated.yml up -d coprocessor1-db-migration"
  assert_contains "${COMMAND_LOG}" "coprocessor-1.generated.yml up -d coprocessor1-host-listener coprocessor1-host-listener-poller"

  assert_contains "${FIXTURE_ROOT}/env/staging/.env.gateway-sc.local" "NUM_COPROCESSORS=2"
  assert_contains "${FIXTURE_ROOT}/env/staging/.env.gateway-sc.local" "COPROCESSOR_THRESHOLD=2"
  assert_contains "${FIXTURE_ROOT}/env/staging/.env.gateway-sc.local" "COPROCESSOR_SIGNER_ADDRESS_1=0x0000000000000000000000000000000000000008"
  assert_contains "${FIXTURE_ROOT}/env/staging/.env.host-sc.local" "COPROCESSOR_SIGNER_ADDRESS_1=0x0000000000000000000000000000000000000008"
  assert_contains "${FIXTURE_ROOT}/env/staging/.env.coprocessor.1.local" "DATABASE_URL=postgresql://postgres:postgres@db:5432/coprocessor_1"
  assert_contains "${FIXTURE_ROOT}/env/staging/.env.coprocessor.1.local" "TX_SENDER_PRIVATE_KEY=0x0000000000000000000000000000000000000000000000000000000000000008"

  cleanup_fixture
}

test_bootstrap_readiness_parses_scientific_cast_output() {
  setup_fixture

  local output_file="${TEST_TMP_DIR}/bootstrap-scientific-cast.out"
  cat > "${FIXTURE_ROOT}/env/staging/.env.gateway-sc" <<'ENV'
KMS_GENERATION_ADDRESS=0x1111111111111111111111111111111111111111
ENV
  export DOCKER_VOLUME_NAMES="fhevm_addresses-volume,fhevm_minio_secrets"
  export DOCKER_RUNNING_NAMES="fhevm-minio,kms-core,coprocessor-and-kms-db,host-node,gateway-node,fhevm-relayer,fhevm-test-suite-e2e-debug"
  export DOCKER_CAST_ACTIVE_KEY_OUTPUT="1809251394333065553493296640760748560207343510400633813116524750123642650625 [1.809e75]"
  export DOCKER_CAST_ACTIVE_CRS_OUTPUT="2261564242916331941866620800950935700259179388000792266395655937654553313281 [2.262e75]"

  if ! run_deploy "${output_file}"; then
    echo "Deploy should succeed when cast output contains scientific notation suffixes" >&2
    cat "${output_file}" >&2
    return 1
  fi

  assert_contains "${COMMAND_LOG}" "cast call"
  assert_contains "${output_file}" "Gateway key bootstrap is ready"
  assert_not_contains "${output_file}" "ERROR_CODE=E_GATEWAY_BOOTSTRAP_TIMEOUT"

  unset DOCKER_VOLUME_NAMES
  unset DOCKER_RUNNING_NAMES
  unset DOCKER_CAST_ACTIVE_KEY_OUTPUT
  unset DOCKER_CAST_ACTIVE_CRS_OUTPUT
  cleanup_fixture
}

test_usage_is_shown_for_cli_argument_errors() {
  setup_fixture

  local output_file="${TEST_TMP_DIR}/usage-error.out"
  if run_cli "${output_file}" unknown-command; then
    echo "Unknown command should fail" >&2
    cat "${output_file}" >&2
    return 1
  fi

  assert_error_code "${output_file}" "E_USAGE"
  cleanup_fixture
}

test_clean_purge_invokes_prunes() {
  setup_fixture

  mkdir -p "${FIXTURE_ROOT}/.buildx-cache/coprocessor"
  echo "dummy" > "${FIXTURE_ROOT}/.buildx-cache/coprocessor/index.json"

  local output_file="${TEST_TMP_DIR}/clean-purge.out"
  if ! run_cli "${output_file}" clean --purge; then
    echo "Clean --purge should succeed" >&2
    cat "${output_file}" >&2
    return 1
  fi

  assert_contains "${COMMAND_LOG}" "minio-docker-compose.yml down -v"
  assert_contains "${COMMAND_LOG}" "docker network ls --filter label=com.docker.compose.project=fhevm --format {{.Name}}"
  assert_contains "${COMMAND_LOG}" "down -v --remove-orphans --rmi all"
  assert_not_contains "${COMMAND_LOG}" "docker image prune -af"
  assert_not_contains "${COMMAND_LOG}" "docker builder prune -af"
  assert_contains "${output_file}" "Removing images referenced by fhevm compose services only."
  assert_contains "${output_file}" "Removing local fhevm Buildx cache directory."
  assert_contains "${output_file}" "Removed local Buildx cache directory"
  if [[ -d "${FIXTURE_ROOT}/.buildx-cache" ]]; then
    echo "Assertion failed: .buildx-cache should be removed by clean --purge" >&2
    return 1
  fi
  cleanup_fixture
}

test_telemetry_smoke_requires_jaeger() {
  setup_fixture

  local output_file="${TEST_TMP_DIR}/telemetry-smoke.out"
  if run_cli "${output_file}" telemetry-smoke; then
    echo "Telemetry smoke should fail when Jaeger is absent" >&2
    cat "${output_file}" >&2
    return 1
  fi

  assert_error_code "${output_file}" "E_JAEGER_NOT_RUNNING"
  cleanup_fixture
}

test_test_command_fails_when_configured_relayer_hostname_is_unreachable() {
  setup_fixture

  local output_file="${TEST_TMP_DIR}/test-relayer-hostname-failure.out"
  export DOCKER_RELAYER_KEYURL_FAIL_HOSTNAME="true"
  export DOCKER_RELAYER_IP="172.22.0.15"

  if run_cli "${output_file}" test input-proof --no-hardhat-compile; then
    echo "Test command should fail when configured relayer hostname is unreachable" >&2
    cat "${output_file}" >&2
    return 1
  fi

  assert_contains "${COMMAND_LOG}" "http://relayer:3000/v2/keyurl"
  assert_not_contains "${COMMAND_LOG}" "http://172.22.0.15:3000/v2/keyurl"
  assert_contains "${output_file}" "Relayer key URLs are not reachable"
  assert_error_code "${output_file}" "E_UNEXPECTED"

  unset DOCKER_RELAYER_KEYURL_FAIL_HOSTNAME
  unset DOCKER_RELAYER_IP
  cleanup_fixture
}

test_command_and_flag_matrix() {
  setup_fixture

  set_network_profile_api_fixture

  run_cli_ok "help-command" help
  assert_contains "${TEST_TMP_DIR}/help-command.out" "input-proof-compute-decrypt"
  assert_contains "${TEST_TMP_DIR}/help-command.out" "paused-host-contracts"
  assert_contains "${TEST_TMP_DIR}/help-command.out" "paused-gateway-contracts"
  run_cli_ok "help-short" -h
  run_cli_ok "help-long" --help

  run_cli_ok "deploy-default-matrix" deploy
  run_cli_ok "deploy-local-matrix" deploy --local
  run_cli_ok "deploy-build-local-matrix" deploy --build --local
  export DOCKER_RUNNING_NAMES="fhevm-minio,kms-core,coprocessor-and-kms-db,host-node,gateway-node"
  run_cli_ok "deploy-network-only-matrix" deploy --network testnet --only coprocessor
  unset DOCKER_RUNNING_NAMES
  run_cli_ok "deploy-resume-build-matrix" deploy --build --resume kms-connector
  run_cli_fail_code "deploy-invalid-network-matrix" "E_USAGE" deploy --network prod
  run_cli_fail_code "deploy-resume-only-conflict-matrix" "E_USAGE" deploy --resume core --only core

  run_cli_ok "pause-host" pause host
  run_cli_ok "pause-gateway" pause gateway
  run_cli_ok "unpause-host" unpause host
  run_cli_ok "unpause-gateway" unpause gateway
  run_cli_fail_code "pause-missing" "E_USAGE" pause
  run_cli_fail_code "unpause-invalid" "E_USAGE" unpause foo

  local test_type
  for test_type in input-proof input-proof-compute-decrypt user-decryption delegated-user-decryption public-decryption erc20 public-decrypt-http-ebool public-decrypt-http-mixed operators random random-subset paused-host-contracts paused-gateway-contracts debug; do
    run_cli_ok "test-${test_type}" test "${test_type}"
  done
  run_cli_ok "test-options-combo" test input-proof -v -n staging -r --no-hardhat-compile -g "my grep"
  run_cli_fail_code "test-unknown" "E_USAGE" test unknown-test
  run_cli_fail_code "test-network-missing" "E_USAGE" test input-proof --network
  run_cli_fail_code "test-grep-missing" "E_USAGE" test input-proof --grep

  local service
  for service in minio core gateway-node gateway-sc gateway-mocked-payment host-node host-sc kms-connector coprocessor relayer test-suite; do
    run_cli_ok "upgrade-${service}" upgrade "${service}"
  done
  run_cli_fail_code "upgrade-unknown" "E_USAGE" upgrade foo

  run_cli_ok "logs-relayer" logs relayer
  run_cli_fail_code "logs-missing" "E_USAGE" logs

  run_cli_ok "clean-default" clean
  run_cli_ok "clean-purge-images" clean --purge-images
  run_cli_ok "clean-purge-build-cache" clean --purge-build-cache
  run_cli_ok "clean-purge-networks" clean --purge-networks
  run_cli_ok "clean-purge-local-cache" clean --purge-local-cache
  run_cli_ok "clean-purge-combo" clean --purge-networks --purge-local-cache
  run_cli_ok "clean-purge-all" clean --purge
  run_cli_fail_code "clean-invalid" "E_USAGE" clean --bad-flag

  run_cli_fail_code "telemetry-smoke-missing-jaeger" "E_JAEGER_NOT_RUNNING" telemetry-smoke

  unset_network_profile_api_fixture
  cleanup_fixture
}

main() {
  trap cleanup_fixture EXIT

  test_default_flow_and_env_patch
  test_resume_preserves_prior_steps_and_restarts_tail
  test_only_runs_single_step
  test_build_flag_applies_only_to_buildable_steps
  test_local_flag_implies_build_for_buildable_steps
  test_oom_failure_is_actionable
  test_key_bootstrap_failure_is_actionable
  test_strict_otel_requires_jaeger
  test_deploy_network_profile_applies_versions
  test_deploy_network_profile_rejects_invalid_value
  test_quoted_otel_endpoint_is_accepted
  test_multicoprocessor_flags_are_validated
  test_multicoprocessor_env_and_extra_instances
  test_bootstrap_readiness_parses_scientific_cast_output
  test_multicoprocessor_resume_forces_minio_reset
  test_clean_purge_invokes_prunes
  test_telemetry_smoke_requires_jaeger
  test_test_command_fails_when_configured_relayer_hostname_is_unreachable
  test_command_and_flag_matrix
  test_usage_is_shown_for_cli_argument_errors

  echo "All deploy behavior tests passed"
}

main "$@"
