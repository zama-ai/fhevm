#!/usr/bin/env bash
# =============================================================================
# fhevm-deploy.sh — cleartext-stack deploy dispatcher.
#
# js-sdk no longer vendors the cleartext contracts. The cleartext host stack is
# owned by the sibling `@fhevm/host-contracts-cleartext` package, published per
# protocol version. This script is a thin alias: it selects the package for the
# requested version and delegates to that package's self-contained deploy.
#
#   FOUNDRY_PROFILE=v13  → host-contracts-cleartext-v13  (protocol v0.13)
#   FOUNDRY_PROFILE=v14  → host-contracts-cleartext      (protocol v0.14)
#
# All arguments (e.g. `--chain localcleartext`, `--dry-run`) are forwarded
# verbatim to the package deploy. v0.11/v0.12 are intentionally unsupported.
# =============================================================================
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
FHEVM_DIR="$(cd "$SCRIPT_DIR/../../../.." && pwd)"   # -> fhevm/

profile="${FOUNDRY_PROFILE:-v14}"
case "$profile" in
    v13)
        pkg_dir="$FHEVM_DIR/host-contracts-cleartext-v13"
        ;;
    v14|latest|default)
        pkg_dir="$FHEVM_DIR/host-contracts-cleartext"
        ;;
    *)
        echo "❌ fhevm-deploy.sh: unsupported FOUNDRY_PROFILE '$profile' (expected: v13 | v14)" >&2
        exit 1
        ;;
esac

pkg_deploy="$pkg_dir/scripts/fhevm-deploy.sh"
if [[ ! -x "$pkg_deploy" ]]; then
    echo "❌ fhevm-deploy.sh: package deploy not found/executable at $pkg_deploy" >&2
    exit 1
fi

echo "🔗 profile=$profile → deploying cleartext stack from $(basename "$pkg_dir")"
# The package is single-version (one default Foundry profile); don't leak our
# vXX profile into its build.
exec env -u FOUNDRY_PROFILE bash "$pkg_deploy" "$@"
