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
          elif [[ "${complete_services}" == *" ${service_name} "* ]]; then
            echo "exited"
          else
            echo "running"
          fi
          ;;
        "{{.State.ExitCode}}")
          if [[ -n "${fail_service}" && "${service_name}" == "${fail_service}" ]]; then
            echo "${fail_exit_code}"
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

  : > "${COMMAND_LOG}"
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

    bun scripts/bun/cli.ts "$@"
  ) > "${output_file}" 2>&1
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
  assert_contains "${COMMAND_LOG}" "gateway-sc-docker-compose.yml up -d"
  assert_contains "${COMMAND_LOG}" "host-sc-docker-compose.yml up -d"
  assert_contains "${COMMAND_LOG}" "relayer-docker-compose.yml up -d"
  assert_contains "${COMMAND_LOG}" "test-suite-docker-compose.yml up -d"
  assert_contains "${COMMAND_LOG}" "setup-kms-signer"

  assert_order "${COMMAND_LOG}" "minio-docker-compose.yml up -d" "core-docker-compose.yml up -d"
  assert_order "${COMMAND_LOG}" "core-docker-compose.yml up -d" "database-docker-compose.yml up -d"
  assert_order "${COMMAND_LOG}" "database-docker-compose.yml up -d" "host-node-docker-compose.yml up -d"
  assert_order "${COMMAND_LOG}" "docker inspect -f {{range .NetworkSettings.Networks}}{{.IPAddress}}{{end}} fhevm-minio" "coprocessor-docker-compose.yml up -d"

  assert_contains "${FIXTURE_ROOT}/env/staging/.env.coprocessor.local" "AWS_ENDPOINT_URL=http://172.20.0.2:9000"
  assert_contains "${FIXTURE_ROOT}/env/staging/.env.coprocessor.local" "OTEL_EXPORTER_OTLP_ENDPOINT=http://jaeger:4317"

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
  assert_contains "${COMMAND_LOG}" "gateway-sc-docker-compose.yml up -d"
  assert_contains "${COMMAND_LOG}" "host-sc-docker-compose.yml up -d"
  assert_contains "${COMMAND_LOG}" "relayer-docker-compose.yml up -d"
  assert_contains "${COMMAND_LOG}" "test-suite-docker-compose.yml up -d"

  assert_contains "${COMMAND_LOG}" "test-suite-docker-compose.yml down -v --remove-orphans"
  assert_contains "${COMMAND_LOG}" "relayer-docker-compose.yml down -v --remove-orphans"
  assert_contains "${COMMAND_LOG}" "host-sc-docker-compose.yml down -v --remove-orphans"
  assert_contains "${COMMAND_LOG}" "gateway-sc-docker-compose.yml down -v --remove-orphans"
  assert_contains "${COMMAND_LOG}" "gateway-mocked-payment-docker-compose.yml down -v --remove-orphans"
  assert_contains "${COMMAND_LOG}" "kms-connector-docker-compose.yml down -v --remove-orphans"
  assert_not_contains "${COMMAND_LOG}" "coprocessor-docker-compose.yml down -v --remove-orphans"

  assert_order "${COMMAND_LOG}" "test-suite-docker-compose.yml down -v --remove-orphans" "kms-connector-docker-compose.yml down -v --remove-orphans"
  assert_order "${COMMAND_LOG}" "docker inspect -f {{range .NetworkSettings.Networks}}{{.IPAddress}}{{end}} fhevm-minio" "kms-connector-docker-compose.yml up -d"

  unset DOCKER_RUNNING_NAMES
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

  assert_contains "${COMMAND_LOG}" "coprocessor-docker-compose.yml down -v --remove-orphans"
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

  local output_file="${TEST_TMP_DIR}/clean-purge.out"
  if ! run_cli "${output_file}" clean --purge; then
    echo "Clean --purge should succeed" >&2
    cat "${output_file}" >&2
    return 1
  fi

  assert_contains "${COMMAND_LOG}" "docker compose -p fhevm down -v --remove-orphans"
  assert_contains "${COMMAND_LOG}" "docker network ls --format {{.Name}}"
  assert_contains "${COMMAND_LOG}" "docker image prune -af"
  assert_contains "${COMMAND_LOG}" "docker builder prune -af"
  assert_contains "${output_file}" "removes ALL unused Docker images system-wide"
  assert_contains "${output_file}" "removes ALL unused Docker build cache system-wide"
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

main() {
  trap cleanup_fixture EXIT

  test_default_flow_and_env_patch
  test_resume_preserves_prior_steps_and_restarts_tail
  test_only_runs_single_step
  test_build_flag_applies_only_to_buildable_steps
  test_oom_failure_is_actionable
  test_key_bootstrap_failure_is_actionable
  test_strict_otel_requires_jaeger
  test_quoted_otel_endpoint_is_accepted
  test_clean_purge_invokes_prunes
  test_telemetry_smoke_requires_jaeger
  test_usage_is_shown_for_cli_argument_errors

  echo "All deploy behavior tests passed"
}

main "$@"
