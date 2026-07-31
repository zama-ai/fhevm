#!/usr/bin/env bash
#
# Regenerates the normative Connector-authorization vectors from the runner that checks them.
#
# There is deliberately no separate generator: the test builds the set in memory and compares it
# against the committed file, and this script runs the same build with the write gate set. A
# generator that is not the runner drifts from it, and then the committed file agrees with neither.
#
# The set can only be generated once the authorization path is implemented — the builder signs
# permits, derives addresses and serializes accounts through the real code.

set -euo pipefail

repository_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"

cd "${repository_root}/kms-connector"

ZAMA_UPDATE_CONNECTOR_AUTH_VECTORS=1 \
  cargo test -p kms-worker --test solana_connector_auth_vectors \
  committed_vectors_match_the_generator -- --exact

echo "wrote ${repository_root}/solana/test-fixtures/connector-auth/connector_auth_v1.json"
