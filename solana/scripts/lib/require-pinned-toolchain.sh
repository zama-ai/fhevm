#!/usr/bin/env bash
# require-pinned-toolchain.sh — assert the local Solana and Anchor CLIs are the ones CI pins.
#
# Source it from a script whose working directory is solana/; it does not run on its own:
#   . "$(dirname "${BASH_SOURCE[0]}")/lib/require-pinned-toolchain.sh"
#
# For the scripts that regenerate committed build output — cost snapshots, IDL and ABI
# goldens. Those files are only comparable with the ones they replace if the same compiler
# wrote both, and a regeneration under a different toolchain is indistinguishable from a real
# change once it is committed.
#
# Reads the Anchor version from Anchor.toml and the Solana version from EXPECTED_SOLANA,
# which callers set to match .github/workflows/solana-tests.yml. Override EXPECTED_SOLANA
# only for experiments; do not commit output minted under a divergent toolchain.

if [[ -z "${EXPECTED_SOLANA:-}" ]]; then
  echo "error: EXPECTED_SOLANA must be set before sourcing require-pinned-toolchain.sh" >&2
  exit 1
fi

EXPECTED_ANCHOR="$(awk -F'"' '/^anchor_version/ { print $2; exit }' Anchor.toml)"
if [[ -z "$EXPECTED_ANCHOR" ]]; then
  echo "error: could not read anchor_version from Anchor.toml" >&2
  exit 1
fi

for cmd in solana anchor cargo; do
  command -v "$cmd" >/dev/null || {
    echo "error: missing required command: $cmd" >&2
    exit 1
  }
done

solana_ver="$(solana --version)"
anchor_ver="$(anchor --version)"

# Require an exact version token after `solana-cli` (followed by a space or end of string),
# so e.g. 2.1.05 cannot satisfy EXPECTED_SOLANA=2.1.0.
case "$solana_ver" in
  "solana-cli ${EXPECTED_SOLANA}"|"solana-cli ${EXPECTED_SOLANA} "*) ;;
  *)
    echo "error: need Solana CLI ${EXPECTED_SOLANA} (got: ${solana_ver})" >&2
    echo "       match CI: .github/workflows/solana-tests.yml SOLANA_VERSION" >&2
    exit 1
    ;;
esac
if [[ "$anchor_ver" != "anchor-cli ${EXPECTED_ANCHOR}" ]]; then
  echo "error: need Anchor ${EXPECTED_ANCHOR} (got: ${anchor_ver})" >&2
  echo "       match CI / Anchor.toml anchor_version" >&2
  exit 1
fi

echo "toolchain ok: ${solana_ver}; ${anchor_ver}"
