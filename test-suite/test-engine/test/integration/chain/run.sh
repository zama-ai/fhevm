#!/bin/sh
# Starts the throwaway Anvil node, runs the Chain module tests against it, and always removes it.
# Set TE_ANVIL_PORT to use another host port than 8545.
set -u
cd "$(dirname "$0")/../../.."

COMPOSE="docker compose -f test/integration/chain/compose.yaml"
cleanup() { $COMPOSE down -v --remove-orphans >/dev/null 2>&1; }
trap cleanup EXIT
trap 'cleanup; exit 130' INT TERM

$COMPOSE up -d --wait || exit 1
TE_ANVIL_RPC_URL="http://127.0.0.1:${TE_ANVIL_PORT:-8545}" \
  node --test --test-reporter=spec "dist/test/integration/chain/**/*.test.js"
