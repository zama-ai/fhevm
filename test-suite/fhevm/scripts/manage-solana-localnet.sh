#!/bin/bash

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/../../.." && pwd)"
SOLANA_ROOT="$REPO_ROOT/solana-host-contracts"
STATE_DIR="$REPO_ROOT/local/solana-stack"
PID_FILE="$STATE_DIR/localnet.pid"
LOG_FILE="$STATE_DIR/localnet.log"
ADDRESSES_ENV="$SOLANA_ROOT/addresses/.env.host"
RPC_URL="${SOLANA_STACK_RPC_URL:-http://127.0.0.1:18999}"
LOGIN_SHELL="${SHELL:-/bin/zsh}"
LEDGER_DIR="$SOLANA_ROOT/.anchor/test-ledger"
AUTHORITY_KEYPAIR="$SOLANA_ROOT/tests/fixtures/anchor-authority.json"
HOST_PROGRAM_SO="$SOLANA_ROOT/target/deploy/solana_host_contracts.so"
TEST_INPUT_PROGRAM_SO="$SOLANA_ROOT/target/deploy/solana_test_input_program.so"
ENCRYPTED_ERC20_PROGRAM_SO="$SOLANA_ROOT/target/deploy/solana_encrypted_erc20_program.so"
HOST_PROGRAM_ID="5TeWSsjg2gbxCyWVniXeCmwM7UtHTCK7svzJr5xYJzHf"
TEST_INPUT_PROGRAM_ID="5MaDNrtMTmYccr1ASgE1i2LZgbnyBPeDR7tN8Q8ewXTv"
ENCRYPTED_ERC20_PROGRAM_ID="Cjb3AVoxxKmG4TGWX5gzSjCNwtxN6gneVsWB7f9i8Csx"

mkdir -p "$STATE_DIR"

rpc_healthy() {
  curl -sS \
    -H 'content-type: application/json' \
    -d '{"jsonrpc":"2.0","id":1,"method":"getHealth","params":[]}' \
    "$RPC_URL" >/dev/null 2>&1
}

pid_is_running() {
  local pid="$1"
  kill -0 "$pid" >/dev/null 2>&1
}

wait_for_rpc() {
  local timeout_secs="${1:-180}"
  local started_at
  started_at=$(date +%s)

  while true; do
    if rpc_healthy; then
      return 0
    fi

    if [[ $(( $(date +%s) - started_at )) -ge "$timeout_secs" ]]; then
      echo "Timed out waiting for Solana validator on $RPC_URL" >&2
      exit 1
    fi

    sleep 1
  done
}

wait_for_localnet() {
  local timeout_secs="${1:-180}"
  local started_at
  started_at=$(date +%s)

  while true; do
    if rpc_healthy && [[ -f "$ADDRESSES_ENV" ]]; then
      return 0
    fi

    if [[ $(( $(date +%s) - started_at )) -ge "$timeout_secs" ]]; then
      echo "Timed out waiting for Solana address bootstrap on $RPC_URL" >&2
      exit 1
    fi

    sleep 1
  done
}

start_localnet() {
  if [[ -f "$PID_FILE" ]]; then
    local existing_pid
    existing_pid=$(cat "$PID_FILE")
    if pid_is_running "$existing_pid" && rpc_healthy; then
      echo "Managed Solana localnet already running (pid=$existing_pid)"
      return 0
    fi
    rm -f "$PID_FILE"
  fi

  if rpc_healthy; then
    echo "A Solana validator is already responding on $RPC_URL; reusing external localnet"
    "$LOGIN_SHELL" -lic "cd '$SOLANA_ROOT' && exec node tests/anchor-localnet.mjs"
    return 0
  fi

  "$LOGIN_SHELL" -lic "cd '$SOLANA_ROOT' && make build-sbf >/dev/null"
  rm -rf "$LEDGER_DIR"
  nohup "$LOGIN_SHELL" -lic "cd '$SOLANA_ROOT' && exec solana-test-validator \
    --reset \
    --ledger '.anchor/test-ledger' \
    --bind-address 127.0.0.1 \
    --rpc-port 18999 \
    --faucet-port 19900 \
    --upgradeable-program '$HOST_PROGRAM_ID' 'target/deploy/solana_host_contracts.so' 'tests/fixtures/anchor-authority.json' \
    --upgradeable-program '$TEST_INPUT_PROGRAM_ID' 'target/deploy/solana_test_input_program.so' 'tests/fixtures/anchor-authority.json' \
    --upgradeable-program '$ENCRYPTED_ERC20_PROGRAM_ID' 'target/deploy/solana_encrypted_erc20_program.so' 'tests/fixtures/anchor-authority.json'" >"$LOG_FILE" 2>&1 &
  echo $! >"$PID_FILE"

  wait_for_rpc 240
  "$LOGIN_SHELL" -lic "cd '$SOLANA_ROOT' && exec node tests/anchor-localnet.mjs"
  wait_for_localnet 240
  echo "Started managed Solana localnet (pid=$(cat "$PID_FILE"))"
}

stop_localnet() {
  if [[ -f "$PID_FILE" ]]; then
    local pid
    pid=$(cat "$PID_FILE")
    if pid_is_running "$pid"; then
      kill "$pid" >/dev/null 2>&1 || true
      for _ in $(seq 1 30); do
        if ! pid_is_running "$pid"; then
          break
        fi
        sleep 1
      done
      if pid_is_running "$pid"; then
        kill -9 "$pid" >/dev/null 2>&1 || true
      fi
    fi
    rm -f "$PID_FILE"
    echo "Stopped managed Solana localnet"
    return 0
  fi

  echo "No managed Solana localnet PID file found"
}

show_status() {
  if [[ -f "$PID_FILE" ]]; then
    local pid
    pid=$(cat "$PID_FILE")
    if pid_is_running "$pid"; then
      echo "managed-running pid=$pid rpc=$RPC_URL"
      return 0
    fi
    if rpc_healthy; then
      echo "external-running rpc=$RPC_URL"
      return 0
    fi
    echo "stale-pid pid=$pid rpc=$RPC_URL"
    return 1
  fi

  if rpc_healthy; then
    echo "external-running rpc=$RPC_URL"
    return 0
  fi

  echo "stopped rpc=$RPC_URL"
  return 1
}

bootstrap_only() {
  "$LOGIN_SHELL" -lic "cd '$SOLANA_ROOT' && exec node tests/anchor-localnet.mjs"
}

case "${1:-}" in
  start)
    start_localnet
    ;;
  stop)
    stop_localnet
    ;;
  restart)
    stop_localnet || true
    start_localnet
    ;;
  status)
    show_status
    ;;
  bootstrap)
    bootstrap_only
    ;;
  *)
    echo "Usage: $0 {start|stop|restart|status|bootstrap}" >&2
    exit 1
    ;;
esac
