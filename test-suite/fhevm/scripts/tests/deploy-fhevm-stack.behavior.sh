#!/bin/bash

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
SCRIPTS_ROOT="$(cd "${SCRIPT_DIR}/.." && pwd)"

DEFAULT_COMPLETE_SERVICES="fhevm-minio-setup coprocessor-db-migration kms-connector-db-migration gateway-deploy-mocked-zama-oft gateway-set-relayer-mocked-payment gateway-sc-deploy gateway-sc-add-network gateway-sc-trigger-keygen gateway-sc-trigger-crsgen gateway-sc-add-pausers host-sc-deploy host-sc-add-pausers"
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
  cp "${SCRIPTS_ROOT}/bun/cli.ts" "${FIXTURE_ROOT}/scripts/bun/cli.ts"
  cp "${SCRIPTS_ROOT}/bun/manifest.ts" "${FIXTURE_ROOT}/scripts/bun/manifest.ts"
  cp "${SCRIPTS_ROOT}/bun/process.ts" "${FIXTURE_ROOT}/scripts/bun/process.ts"

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
      if [[ "$*" == *"http://fhevm-relayer:3000/v2/keyurl"* ]]; then
        cat <<'JSON'
{"response":{"fheKeyInfo":[{"fhePublicKey":{"urls":["http://mock-fhevm/key"]}}],"crs":{"2048":{"urls":["http://mock-fhevm/crs"]}}}}
JSON
        exit 0
      fi

      if [[ "$*" == *"http://mock-fhevm/key"* || "$*" == *"http://mock-fhevm/crs"* ]]; then
        exit 0
      fi
    fi

    exit 0
    ;;
  ps)
    if [[ "${1:-}" == "-a" ]]; then
      shift
      service_filter=""
      while [[ $# -gt 0 ]]; do
        case "$1" in
          --filter)
            if [[ "${2:-}" == name=* ]]; then
              service_filter="${2#name=}"
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

      service_name="${service_filter%\$}"
      if [[ -n "${DOCKER_MISSING_CONTAINERS:-}" && " ${DOCKER_MISSING_CONTAINERS} " == *" ${service_name} "* ]]; then
        exit 0
      fi

      echo "${service_name}-container"
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
        echo "${DOCKER_MINIO_IP:-172.20.0.2}"
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

if [[ "${MOCK_JAEGER_SERVICES_STATUS:-0}" != "0" ]]; then
  exit "${MOCK_JAEGER_SERVICES_STATUS}"
fi

echo "${MOCK_JAEGER_SERVICES_JSON:-{\"data\":[]}}"
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

write_network_dashboard_fixture() {
  local fixture_file=$1
  cat > "${fixture_file}" <<'HTML'
<div data-testid="data-testid Panel header Testnet Currently Deployed Versions">Panel header Testnet Currently Deployed Versions</div>
<div role="gridcell">coprocessor-gw-listener</div><div role="gridcell">ghcr.io</div><div role="gridcell">zama-ai/fhevm/coprocessor/gw-listener</div><div role="gridcell">v0.10.10</div>
<div role="gridcell">coprocessor-host-listener</div><div role="gridcell">ghcr.io</div><div role="gridcell">zama-ai/fhevm/coprocessor/host-listener</div><div role="gridcell">v0.10.10</div>
<div role="gridcell">coprocessor-tx-sender</div><div role="gridcell">ghcr.io</div><div role="gridcell">zama-ai/fhevm/coprocessor/tx-sender</div><div role="gridcell">v0.10.10</div>
<div role="gridcell">kms-connector-db-migration</div><div role="gridcell">hub.zama.org</div><div role="gridcell">zama-protocol/zama-ai/fhevm/kms-connector/db-migration</div><div role="gridcell">v0.10.8</div>
<div role="gridcell">kms-connector-gw-listener</div><div role="gridcell">hub.zama.org</div><div role="gridcell">zama-protocol/zama-ai/fhevm/kms-connector/gw-listener</div><div role="gridcell">v0.10.8</div>
<div role="gridcell">kms-connector-kms-worker</div><div role="gridcell">hub.zama.org</div><div role="gridcell">zama-protocol/zama-ai/fhevm/kms-connector/kms-worker</div><div role="gridcell">v0.10.8</div>
<div role="gridcell">kms-connector-tx-sender</div><div role="gridcell">hub.zama.org</div><div role="gridcell">zama-protocol/zama-ai/fhevm/kms-connector/tx-sender</div><div role="gridcell">v0.10.8</div>
<div role="gridcell">kms-core-enclave</div><div role="gridcell">hub.zama.org</div><div role="gridcell">zama-protocol/zama-ai/kms/core-service-enclave</div><div role="gridcell">v0.12.7</div>
<div class="scene-resize-handle"></div>
<div data-testid="data-testid Panel header Mainnet Currently Deployed Versions">Panel header Mainnet Currently Deployed Versions</div>
<div role="gridcell">coprocessor-gw-listener</div><div role="gridcell">hub.zama.org</div><div role="gridcell">ghcr/zama-ai/fhevm/coprocessor/gw-listener</div><div role="gridcell">v0.10.5</div>
<div role="gridcell">coprocessor-host-listener</div><div role="gridcell">hub.zama.org</div><div role="gridcell">internal/zama-ai/fhevm/coprocessor/host-listener</div><div role="gridcell">v0.10.9</div>
<div role="gridcell">coprocessor-tx-sender</div><div role="gridcell">hub.zama.org</div><div role="gridcell">internal/zama-ai/fhevm/coprocessor/tx-sender</div><div role="gridcell">v0.10.5</div>
<div role="gridcell">kms-connector-db-migration</div><div role="gridcell">hub.zama.org</div><div role="gridcell">zama-protocol/zama-ai/fhevm/kms-connector/db-migration</div><div role="gridcell">v0.10.7</div>
<div role="gridcell">kms-connector-gw-listener</div><div role="gridcell">hub.zama.org</div><div role="gridcell">zama-protocol/zama-ai/fhevm/kms-connector/gw-listener</div><div role="gridcell">v0.10.8</div>
<div role="gridcell">kms-connector-kms-worker</div><div role="gridcell">hub.zama.org</div><div role="gridcell">zama-protocol/zama-ai/fhevm/kms-connector/kms-worker</div><div role="gridcell">v0.10.8</div>
<div role="gridcell">kms-connector-tx-sender</div><div role="gridcell">hub.zama.org</div><div role="gridcell">zama-protocol/zama-ai/fhevm/kms-connector/tx-sender</div><div role="gridcell">v0.10.8</div>
<div role="gridcell">kms-core-enclave</div><div role="gridcell">hub.zama.org</div><div role="gridcell">zama-protocol/zama-ai/kms/core-service-enclave</div><div role="gridcell">v0.12.7</div>
<div class="scene-resize-handle"></div>
HTML
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

run_cli_fail_contains() {
  local label=$1
  local expected=$2
  shift 2
  local output_file="${TEST_TMP_DIR}/${label}.out"
  if run_cli "${output_file}" "$@"; then
    echo "Command should fail: bun scripts/bun/cli.ts $*" >&2
    cat "${output_file}" >&2
    return 1
  fi
  assert_contains "${output_file}" "${expected}"
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

  assert_contains "${output_file}" "looks OOM-killed"
  assert_contains "${output_file}" "./fhevm-cli deploy --resume coprocessor"
  assert_not_contains "${output_file}" "Usage:"

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

  assert_contains "${output_file}" "Detected key-bootstrap-not-ready state"
  assert_contains "${output_file}" "./fhevm-cli deploy --resume gateway-sc"
  assert_not_contains "${output_file}" "Usage:"

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

  assert_contains "${output_file}" "Telemetry endpoint http://jaeger:4317 is configured but Jaeger is not running."
  cleanup_fixture
}

test_deploy_network_profile_applies_versions() {
  setup_fixture
  local dashboard_fixture="${TEST_TMP_DIR}/dashboard.html"
  write_network_dashboard_fixture "${dashboard_fixture}"
  export FHEVM_GRAFANA_DASHBOARD_HTML_FILE="${dashboard_fixture}"

  local output_file="${TEST_TMP_DIR}/network-mainnet.out"
  if ! run_deploy "${output_file}" --network mainnet; then
    echo "Deploy with --network mainnet should succeed" >&2
    cat "${output_file}" >&2
    return 1
  fi

  assert_contains "${output_file}" "Applied 8 version overrides from 'mainnet' public dashboard snapshot."
  assert_contains "${output_file}" "coprocessor/gw-listener:v0.10.5"
  assert_contains "${output_file}" "coprocessor/host-listener:v0.10.9"
  assert_contains "${output_file}" "coprocessor/tx-sender:v0.10.5"
  assert_contains "${output_file}" "kms-connector/db-migration:v0.10.7"
  assert_contains "${output_file}" "kms-core-service:v0.12.7"

  unset FHEVM_GRAFANA_DASHBOARD_HTML_FILE
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

  assert_contains "${output_file}" "Invalid deploy network profile: foo"
  assert_contains "${output_file}" "Allowed values: testnet mainnet"
  assert_contains "${output_file}" "Usage:"

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

  assert_contains "${output_file}" "Invalid coprocessor threshold: 3 (must be <= --coprocessors 2)"
  assert_contains "${output_file}" "Usage:"
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

test_usage_is_shown_for_cli_argument_errors() {
  setup_fixture

  local output_file="${TEST_TMP_DIR}/usage-error.out"
  if run_cli "${output_file}" unknown-command; then
    echo "Unknown command should fail" >&2
    cat "${output_file}" >&2
    return 1
  fi

  assert_contains "${output_file}" "Unknown command: unknown-command"
  assert_contains "${output_file}" "Usage:"
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

  assert_contains "${output_file}" "Jaeger container is not running."
  cleanup_fixture
}

test_command_and_flag_matrix() {
  setup_fixture

  local dashboard_fixture="${TEST_TMP_DIR}/dashboard-matrix.html"
  write_network_dashboard_fixture "${dashboard_fixture}"
  export FHEVM_GRAFANA_DASHBOARD_HTML_FILE="${dashboard_fixture}"

  run_cli_ok "help-command" help
  assert_contains "${TEST_TMP_DIR}/help-command.out" "input-proof-compute-decrypt"
  assert_contains "${TEST_TMP_DIR}/help-command.out" "paused-host-contracts"
  assert_contains "${TEST_TMP_DIR}/help-command.out" "paused-gateway-contracts"
  run_cli_ok "help-short" -h
  run_cli_ok "help-long" --help

  run_cli_ok "deploy-default-matrix" deploy
  run_cli_ok "deploy-build-local-matrix" deploy --build --local
  run_cli_ok "deploy-network-only-matrix" deploy --network testnet --only coprocessor
  run_cli_ok "deploy-resume-build-matrix" deploy --build --resume kms-connector
  run_cli_fail_contains "deploy-invalid-network-matrix" "Allowed values: testnet mainnet" deploy --network prod
  run_cli_fail_contains "deploy-resume-only-conflict-matrix" "Cannot use --resume and --only together" deploy --resume core --only core

  run_cli_ok "pause-host" pause host
  run_cli_ok "pause-gateway" pause gateway
  run_cli_ok "unpause-host" unpause host
  run_cli_ok "unpause-gateway" unpause gateway
  run_cli_fail_contains "pause-missing" "Unknown service:" pause
  run_cli_fail_contains "unpause-invalid" "Unknown service: foo" unpause foo

  local test_type
  for test_type in input-proof input-proof-compute-decrypt user-decryption delegated-user-decryption public-decryption erc20 public-decrypt-http-ebool public-decrypt-http-mixed operators random random-subset paused-host-contracts paused-gateway-contracts debug; do
    run_cli_ok "test-${test_type}" test "${test_type}"
  done
  run_cli_ok "test-options-combo" test input-proof -v -n staging -r --no-hardhat-compile -g "my grep"
  run_cli_fail_contains "test-unknown" "Unknown test type: unknown-test" test unknown-test
  run_cli_fail_contains "test-network-missing" "Network argument missing" test input-proof --network
  run_cli_fail_contains "test-grep-missing" "Grep pattern missing" test input-proof --grep

  local service
  for service in minio core gateway-node gateway-sc gateway-mocked-payment host-node host-sc kms-connector coprocessor relayer test-suite; do
    run_cli_ok "upgrade-${service}" upgrade "${service}"
  done
  run_cli_fail_contains "upgrade-unknown" "Unknown service: foo" upgrade foo

  run_cli_ok "logs-relayer" logs relayer
  run_cli_fail_contains "logs-missing" "Service name is required" logs

  run_cli_ok "clean-default" clean
  run_cli_ok "clean-purge-images" clean --purge-images
  run_cli_ok "clean-purge-build-cache" clean --purge-build-cache
  run_cli_ok "clean-purge-networks" clean --purge-networks
  run_cli_ok "clean-purge-local-cache" clean --purge-local-cache
  run_cli_ok "clean-purge-combo" clean --purge-networks --purge-local-cache
  run_cli_ok "clean-purge-all" clean --purge
  run_cli_fail_contains "clean-invalid" "Unknown option for clean: --bad-flag" clean --bad-flag

  run_cli_fail_contains "telemetry-smoke-missing-jaeger" "Jaeger container is not running." telemetry-smoke

  unset FHEVM_GRAFANA_DASHBOARD_HTML_FILE
  cleanup_fixture
}

main() {
  trap cleanup_fixture EXIT

  test_default_flow_and_env_patch
  test_resume_preserves_prior_steps_and_restarts_tail
  test_only_runs_single_step
  test_build_flag_applies_only_to_buildable_steps
  test_oom_failure_is_actionable
  test_key_bootstrap_failure_is_actionable
  test_strict_otel_requires_jaeger
  test_deploy_network_profile_applies_versions
  test_deploy_network_profile_rejects_invalid_value
  test_quoted_otel_endpoint_is_accepted
  test_multicoprocessor_flags_are_validated
  test_multicoprocessor_env_and_extra_instances
  test_multicoprocessor_resume_forces_minio_reset
  test_clean_purge_invokes_prunes
  test_telemetry_smoke_requires_jaeger
  test_command_and_flag_matrix
  test_usage_is_shown_for_cli_argument_errors

  echo "All deploy behavior tests passed"
}

main "$@"
