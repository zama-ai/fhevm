#!/usr/bin/env bash
# =============================================================================
# lib-common.sh — minimal, self-contained deploy helpers for
# @fhevm/host-contracts-cleartext.
#
# This is a trimmed, package-local version of the js-sdk test harness helper of
# the same name. It carries ONLY what the cleartext host deploy needs and reads
# chain config from this package's own `chains/` dir (no dependency on the SDK).
#
# Path convention: this file lives in `<package>/scripts/`, so `<package>` root
# is one level up and chain config is at `<package>/chains/`.
#
# Provides: is_truthy, detect_rpc_client, fhevm_assert_chain,
#           resolve_chain_rpc_url, resolve_fhetest_address, fhevm_chain_file,
#           resolve_signer_address, pad_address, normalize_address.
# =============================================================================

# Load guard so re-sourcing is safe.
if [[ -n "${__LIB_COMMON_SH_LOADED:-}" ]]; then
    return 0 2>/dev/null || true
fi
__LIB_COMMON_SH_LOADED=1

# Absolute path to this package's chains/ dir (BASH_SOURCE-relative).
_pkg_chains_dir() {
    cd "$(dirname "${BASH_SOURCE[0]}")/../chains" && pwd
}

is_truthy() {
    case "${1:-}" in
        1|true|TRUE|True|yes|YES|on|ON) return 0 ;;
        *) return 1 ;;
    esac
}

detect_rpc_client() {
    cast client --rpc-url "$RPC_URL" 2>/dev/null | tr '[:upper:]' '[:lower:]'
}

# Validates that <chain_name> is a key in chain-defaults.json (fail-fast).
fhevm_assert_chain() {
    local chain="$1"
    local chain_defaults_file
    chain_defaults_file="$(_pkg_chains_dir)/chain-defaults.json"

    if [[ ! -f "$chain_defaults_file" ]]; then
        echo "❌ chain-defaults.json not found at $chain_defaults_file" >&2
        exit 1
    fi
    if [[ -z "$chain" ]]; then
        echo "❌ chain name is required (expected: $(jq -r 'keys | join(" | ")' "$chain_defaults_file"))" >&2
        exit 1
    fi
    if ! jq -e --arg c "$chain" 'has($c)' "$chain_defaults_file" >/dev/null; then
        echo "❌ unsupported chain '$chain' (expected: $(jq -r 'keys | join(" | ")' "$chain_defaults_file"))" >&2
        exit 1
    fi
}

# Resolves the RPC URL for a chain. Precedence: process-env RPC_URL, then the
# chain's rpcUrl in chain-defaults.json.
resolve_chain_rpc_url() {
    local chain="$1"
    local chain_defaults_file
    chain_defaults_file="$(_pkg_chains_dir)/chain-defaults.json"

    if [[ ! -f "$chain_defaults_file" ]]; then
        echo "resolve_chain_rpc_url: chain-defaults.json not found at $chain_defaults_file" >&2
        return 1
    fi

    local default_rpc_url
    default_rpc_url="$(jq -r --arg c "$chain" '.[$c].rpcUrl // ""' "$chain_defaults_file")"
    local rpc_url="${RPC_URL:-$default_rpc_url}"

    if [[ -z "$rpc_url" ]]; then
        echo "resolve_chain_rpc_url: no RPC URL for chain '$chain' — set RPC_URL or add '${chain}.rpcUrl' to $chain_defaults_file" >&2
        return 1
    fi
    printf '%s\n' "$rpc_url"
}

resolve_fhetest_address() {
    local chain="$1"
    local chain_defaults_file="${2:-$(_pkg_chains_dir)/chain-defaults.json}"
    if [[ ! -f "$chain_defaults_file" ]]; then
        echo "resolve_fhetest_address: chain-defaults.json not found at $chain_defaults_file" >&2
        return 1
    fi
    local fhetest_address
    fhetest_address="$(jq -r --arg c "$chain" '.[$c].fheTestAddress // ""' "$chain_defaults_file")"
    if [[ -z "$fhetest_address" ]]; then
        echo "resolve_fhetest_address: no fheTestAddress for chain '$chain' in $chain_defaults_file" >&2
        return 1
    fi
    printf '%s\n' "$fhetest_address"
}

# Resolves the chain TS file. This package only ships local cleartext chains.
fhevm_chain_file() {
    local chain="$1"
    case "$chain" in
        localcleartext|localcleartext_*)
            printf '%s/localcleartext.ts\n' "$(_pkg_chains_dir)"
            ;;
        *)
            echo "fhevm_chain_file: unsupported chain '$chain' (this package ships localcleartext only)" >&2
            return 1
            ;;
    esac
}

resolve_signer_address() {
    local addr_var="$1"
    local pk_var="$2"
    if [[ -n "${!addr_var:-}" ]]; then
        printf '%s\n' "${!addr_var}"
        return 0
    fi
    cast wallet address --private-key "${!pk_var}" 2>/dev/null
}

pad_address() {
    local address_value="${1#0x}"
    printf '0x%064s\n' "$address_value" | tr ' ' '0'
}

normalize_address() {
    printf '%s\n' "$1" | tr '[:upper:]' '[:lower:]'
}
