#!/bin/bash

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
FHEVM_DIR="$(cd "${SCRIPT_DIR}/../.." && pwd)"
DEFAULT_COMPLETE_SERVICES="fhevm-minio-setup coprocessor-db-migration coprocessor1-db-migration kms-connector-db-migration gateway-deploy-mocked-zama-oft gateway-set-relayer-mocked-payment gateway-sc-deploy gateway-sc-add-network gateway-sc-trigger-keygen gateway-sc-trigger-crsgen gateway-sc-add-pausers host-sc-deploy host-sc-add-pausers"

TMP_DIR=""
FIXTURE=""

cleanup() {
  if [[ -n "${TMP_DIR}" && -d "${TMP_DIR}" ]]; then
    rm -rf "${TMP_DIR}"
  fi
}

setup_fixture() {
  TMP_DIR="$(mktemp -d)"
  FIXTURE="${TMP_DIR}/fhevm"

  mkdir -p "${FIXTURE}/scripts" "${FIXTURE}/scripts/bun" "${FIXTURE}/scripts/legacy/lib" "${FIXTURE}/scripts/tests"
  mkdir -p "${FIXTURE}/docker-compose" "${FIXTURE}/env/staging" "${FIXTURE}/config/relayer" "${FIXTURE}/mock-bin"

  cp "${FHEVM_DIR}/fhevm-cli" "${FIXTURE}/fhevm-cli"
  cp "${FHEVM_DIR}/fhevm-cli.legacy" "${FIXTURE}/fhevm-cli.legacy"
  cp "${FHEVM_DIR}/scripts/deploy-fhevm-stack.sh" "${FIXTURE}/scripts/deploy-fhevm-stack.sh"
  cp "${FHEVM_DIR}/scripts/deploy-fhevm-stack.legacy.sh" "${FIXTURE}/scripts/deploy-fhevm-stack.legacy.sh"
  cp "${FHEVM_DIR}/scripts/setup-kms-signer-address.sh" "${FIXTURE}/scripts/setup-kms-signer-address.sh"
  cp "${FHEVM_DIR}/scripts/legacy/lib/deploy-manifest.sh" "${FIXTURE}/scripts/legacy/lib/deploy-manifest.sh"
  cp "${FHEVM_DIR}/scripts/legacy/lib/version-manifest.sh" "${FIXTURE}/scripts/legacy/lib/version-manifest.sh"
  cp "${FHEVM_DIR}/scripts/bun/"*.ts "${FIXTURE}/scripts/bun/"

  chmod +x "${FIXTURE}/fhevm-cli" "${FIXTURE}/fhevm-cli.legacy" \
    "${FIXTURE}/scripts/deploy-fhevm-stack.sh" "${FIXTURE}/scripts/deploy-fhevm-stack.legacy.sh" \
    "${FIXTURE}/scripts/setup-kms-signer-address.sh" "${FIXTURE}/scripts/bun/cli.ts"

  cat > "${FIXTURE}/scripts/setup-kms-signer-address.sh" <<'SETUP'
#!/bin/bash

echo "setup-kms-signer" >> "${TEST_COMMAND_LOG}"
SETUP
  chmod +x "${FIXTURE}/scripts/setup-kms-signer-address.sh"

  cat > "${FIXTURE}/config/relayer/local.yaml" <<'RELAYER'
relayer: local
RELAYER

  # shellcheck source=/dev/null
  source "${FIXTURE}/scripts/legacy/lib/deploy-manifest.sh"

  local component
  while IFS= read -r component; do
    local env_file="${FIXTURE}/env/staging/.env.${component}"
    if [[ "${component}" == "coprocessor" ]]; then
      cat > "${env_file}" <<'ENV'
AWS_ENDPOINT_URL=http://minio:9000
ENV
    else
      cat > "${env_file}" <<'ENV'
DUMMY=1
ENV
    fi

    cat > "${FIXTURE}/docker-compose/${component}-docker-compose.yml" <<'YAML'
services: {}
YAML
  done < <(fhevm_manifest_step_components)

  cat > "${FIXTURE}/docker-compose/gateway-pause-docker-compose.yml" <<'YAML'
services: {}
YAML
  cat > "${FIXTURE}/docker-compose/gateway-unpause-docker-compose.yml" <<'YAML'
services: {}
YAML
  cat > "${FIXTURE}/docker-compose/host-pause-docker-compose.yml" <<'YAML'
services: {}
YAML
  cat > "${FIXTURE}/docker-compose/host-unpause-docker-compose.yml" <<'YAML'
services: {}
YAML

  cat > "${FIXTURE}/mock-bin/docker" <<'DOCKER'
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

    shift || true
    if [[ "${1:-}" == "curl" ]]; then
      if [[ "$*" == *"http://relayer:3000/v2/keyurl"* ]]; then
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

    if [[ "${1:-}" == "--filter" && "${2:-}" == "name=fhevm-minio" ]]; then
      if [[ "${DOCKER_RUNNING_MINIO:-false}" == "true" ]]; then
        echo "fhevm-minio"
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
  inspect)
    if [[ "${1:-}" == "-f" ]]; then
      format="${2:-}"
      if [[ "${format}" == "{{range .NetworkSettings.Networks}}{{.IPAddress}}{{end}}" ]]; then
        echo "${DOCKER_MINIO_IP:-172.20.0.2}"
        exit 0
      fi
    fi

    if [[ "${1:-}" == "--format" ]]; then
      format="${2:-}"
      target="${3:-}"
      service_name="${target%-container}"
      complete_services=" ${DOCKER_COMPLETE_SERVICES:-} "

      case "${format}" in
        "{{.State.Status}}")
          if [[ "${complete_services}" == *" ${service_name} "* ]]; then
            echo "exited"
          else
            echo "running"
          fi
          ;;
        "{{.State.ExitCode}}")
          echo "0"
          ;;
        "{{.State.OOMKilled}}")
          echo "false"
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
    echo "mock logs"
    exit 0
    ;;
  *)
    exit 0
    ;;
esac
DOCKER
  chmod +x "${FIXTURE}/mock-bin/docker"

  cat > "${FIXTURE}/mock-bin/sleep" <<'SLEEP'
#!/bin/bash
exit 0
SLEEP
  chmod +x "${FIXTURE}/mock-bin/sleep"
}

run_command() {
  local log_file=$1
  local out_file=$2
  local mode=$3
  shift 3

  (
    cd "${FIXTURE}"
    export PATH="${FIXTURE}/mock-bin:${PATH}"
    export TEST_COMMAND_LOG="${log_file}"
    export DOCKER_COMPLETE_SERVICES="${DOCKER_COMPLETE_SERVICES:-${DEFAULT_COMPLETE_SERVICES}}"
    export DOCKER_MINIO_IP="${DOCKER_MINIO_IP:-172.20.0.2}"
    export DOCKER_RUNNING_NAMES="${DOCKER_RUNNING_NAMES:-fhevm-relayer,fhevm-test-suite-e2e-debug}"
    # Keep fixture behavior aligned with legacy expected container names.
    export FHEVM_DOCKER_PROJECT="fhevm"

    if [[ "$mode" == "legacy" ]]; then
      export FHEVM_CLI_IMPL=legacy
    else
      unset FHEVM_CLI_IMPL || true
    fi

    "$@"
  ) > "${out_file}" 2>&1
}

run_case() {
  local name=$1
  local legacy_cmd=$2
  local bun_cmd=$3
  local legacy_log="${TMP_DIR}/${name}.legacy.log"
  local bun_log="${TMP_DIR}/${name}.bun.log"
  local legacy_out="${TMP_DIR}/${name}.legacy.out"
  local bun_out="${TMP_DIR}/${name}.bun.out"

  : > "${legacy_log}"
  : > "${bun_log}"

  set +e
  run_command "${legacy_log}" "${legacy_out}" legacy bash -c "$legacy_cmd"
  local legacy_status=$?
  run_command "${bun_log}" "${bun_out}" bun bash -c "$bun_cmd"
  local bun_status=$?
  set -e

  if [[ $legacy_status -ne $bun_status ]]; then
    echo "Status mismatch for case '${name}': legacy=${legacy_status}, bun=${bun_status}" >&2
    echo "--- legacy output ---" >&2
    cat "${legacy_out}" >&2
    echo "--- bun output ---" >&2
    cat "${bun_out}" >&2
    return 1
  fi

  if [[ $bun_status -eq 0 ]] && grep -Fq "ERROR_CODE=" "${bun_out}"; then
    echo "Unexpected ERROR_CODE marker in successful bun run for case '${name}'" >&2
    cat "${bun_out}" >&2
    return 1
  fi
}

main() {
  trap cleanup EXIT
  setup_fixture

  export DOCKER_RUNNING_MINIO=false
  run_case "deploy-default" "./fhevm-cli.legacy deploy" "./fhevm-cli deploy"

  run_case "deploy-build" "./fhevm-cli.legacy deploy --build" "./fhevm-cli deploy --build"

  export DOCKER_RUNNING_MINIO=true
  run_case "deploy-resume-script" "./scripts/deploy-fhevm-stack.legacy.sh --resume kms-connector" "./scripts/deploy-fhevm-stack.sh --resume kms-connector"
  export DOCKER_RUNNING_NAMES="fhevm-minio,kms-core,coprocessor-and-kms-db,host-node,gateway-node"
  run_case "deploy-only-script" "./scripts/deploy-fhevm-stack.legacy.sh --only coprocessor" "./scripts/deploy-fhevm-stack.sh --only coprocessor"
  unset DOCKER_RUNNING_NAMES || true
  export DOCKER_RUNNING_MINIO=false

  run_case "pause-host" "./fhevm-cli.legacy pause host" "./fhevm-cli pause host"
  run_case "unpause-gateway" "./fhevm-cli.legacy unpause gateway" "./fhevm-cli unpause gateway"

  run_case "test-input-proof" "./fhevm-cli.legacy test input-proof" "./fhevm-cli test input-proof"
  run_case "test-operators" "./fhevm-cli.legacy test operators" "./fhevm-cli test operators"
  run_case "test-debug" "./fhevm-cli.legacy test debug" "./fhevm-cli test debug"

  run_case "upgrade-coprocessor" "./fhevm-cli.legacy upgrade coprocessor" "./fhevm-cli upgrade coprocessor"
  run_case "logs-relayer" "./fhevm-cli.legacy logs relayer" "./fhevm-cli logs relayer"
  run_case "clean" "./fhevm-cli.legacy clean" "./fhevm-cli clean"

  echo "Parity diff checks passed"
}

main "$@"
