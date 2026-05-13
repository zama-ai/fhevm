#!/usr/bin/env bash
# =============================================================================
# fhevm-lib.sh — Small shell utilities for FHEVM deployment scripts.
#
# Source from another bash script:
#
#   SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
#   source "$SCRIPT_DIR/fhevm-lib.sh"
#
# Then call:
#
#   if is_anvil "$rpc_url"; then ...
#
# Functions defined:
#   is_anvil <rpc_url>   — exit 0 if the RPC is an anvil node, else non-zero.
# =============================================================================

# Guard against double-sourcing — re-sourcing is harmless but wasteful.
if [[ -n "${__FHEVM_LIB_SH_LOADED:-}" ]]; then
    return 0
fi
__FHEVM_LIB_SH_LOADED=1

# Prints <message> inside an `=`-banner to stderr. Used as the standard error
# format for the assert helpers in this lib.
#
# Usage: boxed_error <message>
boxed_error() {
    local message="$1"
    cat >&2 <<EOF

================================================================================
❌ ${message}
================================================================================

EOF
}

# Asserts that the deployed FHETest address matches the constants committed
# in the v2 addresses JSON file for the given chain. Exits 1 on mismatch
# (or missing file / missing key) with a banner-formatted error.
#
# Behavior per chain:
#   localhost | localstack  → checks BOTH `localhost` AND `localstack`
#                             keys in the JSON.
#   devnet                  → checks the `devnet` key.
#   anything else           → no-op (returns 0).
#
# Usage: assert_fhetest_address_in_abi_v2 <chain> <deployed_addr> <addresses_json_file>
assert_fhetest_address_in_abi_v2() {
    local chain="$1"
    local deployed_addr="$2"
    local addresses_json_file="$3"

    local -a keys=()
    case "$chain" in
        localhost|localstack)
            keys=(localhost localstack)
            ;;
        devnet)
            keys=(devnet)
            ;;
        *)
            return 0
            ;;
    esac

    if [[ ! -f "$addresses_json_file" ]]; then
        boxed_error "Expected addresses JSON at $addresses_json_file, but the file does not exist."
        exit 1
    fi

    local key expected_addr
    for key in "${keys[@]}"; do
        expected_addr="$(_extract_fhetest_address_from_json "$addresses_json_file" "$key")"
        if [[ -z "$expected_addr" || "$expected_addr" == "null" ]]; then
            boxed_error "Could not find key '${key}' in $addresses_json_file"
            exit 1
        fi

        if [[ "${expected_addr,,}" != "${deployed_addr,,}" ]]; then
            boxed_error "Key '${key}' in $addresses_json_file does not match deployed FHETest.
  expected from JSON: $expected_addr
  deployed:           $deployed_addr"
            exit 1
        fi
    done
}

# Private: extracts a FHETest address from a JSON file by key.
# JSON shape: { "<key>": "0x...", ... }. Uses jq -r so the output is the
# raw address (no quotes); returns the literal string "null" when the key
# is absent — caller should treat that as "not found".
_extract_fhetest_address_from_json() {
    local file="$1"
    local key="$2"
    jq -r --arg k "$key" '.[$k] // ""' "$file"
}

# Returns 0 if the JSON-RPC at <rpc_url> is an anvil node, non-zero otherwise.
#
# Detection: calls the standard `web3_clientVersion` RPC method and checks the
# response for the substring "anvil". Geth, reth, hardhat, and others report
# different identifiers, so the match is reliable. Returns non-zero on an
# unreachable / broken RPC as well.
#
# Usage: is_anvil <rpc_url>
is_anvil() {
    local rpc_url="$1"
    if [[ -z "$rpc_url" ]]; then
        echo "is_anvil: missing rpc_url argument" >&2
        return 2
    fi

    local version
    version="$(cast rpc web3_clientVersion --rpc-url "$rpc_url" 2>/dev/null)" || return 1
    [[ "$version" == *anvil* ]]
}

# Fail-fast variant of `is_anvil`. Exits the calling process with status 1
# (banner-formatted via `boxed_error`) when the RPC at <rpc_url> isn't an
# anvil node — i.e. either unreachable or running a different client.
#
# Use at script entry to short-circuit before any downstream call that
# depends on anvil-specific cheats (anvil_setBalance, anvil_setCode, etc.).
#
# Usage: assert_is_anvil <rpc_url>
assert_is_anvil() {
    local rpc_url="$1"
    if [[ -z "$rpc_url" ]]; then
        boxed_error "assert_is_anvil: missing rpc_url argument"
        exit 1
    fi
    if ! is_anvil "$rpc_url"; then
        boxed_error "anvil is not running at $rpc_url (or another client is on that port).
  Start anvil first, e.g.:  anvil --port 8545"
        exit 1
    fi
}

set_anvil_balance() {
    local account_addr="$1"
    local amount_eth="$2"
    local rpc_url="$3"
    local zero_address="0x0000000000000000000000000000000000000000"

    local fund_amount_wei
    fund_amount_wei="$(cast to-hex "$(cast to-wei "$amount_eth" ether)")"

    if [[ "$account_addr" != "$zero_address" ]]; then
        echo "🍟 Funding ${account_addr} with ${amount_eth} ETH..."
        cast rpc anvil_setBalance "$account_addr" "$fund_amount_wei" --rpc-url "$rpc_url" >/dev/null
    fi
}

# Verifies that <account_addr> on <rpc_url> holds exactly <expected_eth> ETH.
# Silently no-ops (return 0) when <account_addr> is the zero address — that's
# the convention for "not configured" used by `set_anvil_balance` and
# `resolveDeployersAsJson`. Prints ✅ on match (stdout) and ❌ + return 1 on
# mismatch (stderr).
#
# The optional <label> overrides the friendly name shown in the log line;
# defaults to the address itself.
#
# Usage: verify_balance <account_addr> <expected_eth> <rpc_url> [<label>]
verify_balance() {
    local account_addr="$1"
    local expected_eth="$2"
    local rpc_url="$3"
    local label="${4:-$account_addr}"
    local zero_address="0x0000000000000000000000000000000000000000"

    if [[ "$account_addr" == "$zero_address" ]]; then
        return 0
    fi

    local expected_wei actual_wei
    expected_wei="$(cast to-wei "$expected_eth" ether)"
    actual_wei="$(cast balance "$account_addr" --rpc-url "$rpc_url")"

    if [[ "$actual_wei" != "$expected_wei" ]]; then
        echo "❌ ${label} balance mismatch: expected ${expected_wei} wei, got ${actual_wei} wei" >&2
        return 1
    fi
    echo "✅ ${label} balance: $(cast from-wei "$actual_wei") ETH"
}

# Validates that <profile> is one of the supported Foundry profiles for the
# fhevm contracts workspace. On invalid input, prints an error and EXITS the
# calling process with status 1 (fail-fast; intended for script entry).
#
# Supported: v12 | v13 | latest
#
# Usage: fhevm_assert_foundry_profile <profile>
fhevm_assert_foundry_profile() {
    local profile="$1"
    case "$profile" in
        v12|v13|latest)
            return 0
            ;;
        "")
            echo "❌ Foundry profile is required (expected: v12 | v13 | latest)" >&2
            exit 1
            ;;
        *)
            echo "❌ unsupported Foundry profile '$profile' (expected: v12 | v13 | latest)" >&2
            exit 1
            ;;
    esac
}

# Resolves the FHEVMHostAddresses.sol path for a given Foundry profile.
# Path is computed relative to this library's location (assumes layout under
# sdk/js-sdk/contracts/scripts/).
#
# Usage: fhevm_host_addresses_file <profile>
#   <profile>: v12 | v13 | latest
#
#   addresses_file="$(fhevm_host_addresses_file v13)"
fhevm_host_addresses_file() {
    local profile="$1"
    local contracts_dir
    contracts_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

    case "$profile" in
        v12)
            printf '%s/src/v0.12.0/host-contracts/addresses/FHEVMHostAddresses.sol\n' "$contracts_dir"
            ;;
        v13)
            printf '%s/src/v0.13.0/host-contracts/addresses/FHEVMHostAddresses.sol\n' "$contracts_dir"
            ;;
        latest)
            printf '%s/src/latest/host-contracts/addresses/FHEVMHostAddresses.sol\n' "$contracts_dir"
            ;;
        *)
            echo "fhevm_host_addresses_file: unsupported profile '$profile' (expected: v12 | v13 | latest)" >&2
            return 1
            ;;
    esac
}

# Validates that <chain_name> is one of the supported values. On invalid
# input, prints an error and EXITS the calling process with status 1 (this
# is intentionally fail-fast — meant to be called at script entry, before
# any work is done).
#
# Supported: mainnet | testnet | devnet | localhost | localstack
#
# Usage: fhevm_assert_chain <chain_name>
fhevm_assert_chain() {
    local chain="$1"
    case "$chain" in
        mainnet|testnet|devnet|localhost|localstack)
            return 0
            ;;
        "")
            echo "❌ chain name is required (expected: mainnet | testnet | devnet | localhost | localstack)" >&2
            exit 1
            ;;
        *)
            echo "❌ unsupported chain '$chain' (expected: mainnet | testnet | devnet | localhost | localstack)" >&2
            exit 1
            ;;
    esac
}

# Resolves the chain TS file for a given fhevm chain name. Paths are computed
# relative to this library's location (assuming the standard SDK layout under
# sdk/js-sdk/).
#
# Public-network chains live under src/core/chains/definitions/; local /
# fixture chains live under test/fheTest/chains/. The chain name doesn't
# always match the filename (testnet → sepolia.ts).
#
# Usage: fhevm_chain_file <chain_name>
#   <chain_name>: mainnet | testnet | devnet | localhost | localstack
fhevm_chain_file() {
    local chain="$1"
    local sdk_root
    sdk_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"

    case "$chain" in
        mainnet)
            printf '%s/src/core/chains/definitions/mainnet.ts\n' "$sdk_root"
            ;;
        testnet)
            # SDK file is named after the network (sepolia), not the role (testnet).
            printf '%s/src/core/chains/definitions/sepolia.ts\n' "$sdk_root"
            ;;
        devnet|localhost|localstack)
            printf '%s/test/fheTest/chains/%s.ts\n' "$sdk_root" "$chain"
            ;;
        *)
            echo "fhevm_chain_file: unsupported chain '$chain' (expected: mainnet | testnet | devnet | localhost | localstack)" >&2
            return 1
            ;;
    esac
}

# Returns the RPC URL for the given chain name, by reading $RPC_URL from
# the chain's `.env.<chain>` file under sdk/js-sdk/test/.
#
# Mapping (mostly follows the chain name; testnet shares sepolia.env):
#   mainnet         → test/.env.mainnet
#   testnet         → test/.env.sepolia
#   devnet          → test/.env.devnet
#   localhost       → test/.env.localhost
#   localstack      → test/.env.localstack
#
# Usage: fhevm_rpc_url <chain_name>
#
#   rpc_url="$(fhevm_rpc_url localhost)"
fhevm_rpc_url() {
    local chain="$1"
    local sdk_root
    sdk_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"

    local env_file
    case "$chain" in
        mainnet|devnet|localhost|localstack)
            env_file="$sdk_root/test/.env.$chain"
            ;;
        testnet)
            # testnet shares the sepolia env file.
            env_file="$sdk_root/test/.env.sepolia"
            ;;
        *)
            echo "fhevm_rpc_url: unsupported chain '$chain' (expected: mainnet | testnet | devnet | localhost | localstack)" >&2
            return 1
            ;;
    esac

    if [[ ! -f "$env_file" ]]; then
        echo "fhevm_rpc_url: env file not found: $env_file" >&2
        return 1
    fi

    # Subshell-source the file to extract RPC_URL (env files are KEY="value" format).
    local rpc_url
    rpc_url="$(set -a; source "$env_file"; set +a; printf '%s' "${RPC_URL:-}")"

    if [[ -z "$rpc_url" ]]; then
        echo "fhevm_rpc_url: RPC_URL not set in $env_file" >&2
        return 1
    fi

    printf '%s\n' "$rpc_url"
}

# Reads a single contract address from a chain TS file by key.
#
# Supported keys (matched as `<key>:` in the TS source):
#   acl, inputVerifier, kmsVerifier        — host-stack contracts
#   decryption, inputVerification          — gateway-side verifying contracts
#
# Implementation note: greps for the line `<key>:` followed by the next
# `address: '0x…'` line. The chain TS files are auto-generated with a
# stable shape, so this is reliable; if the shape changes (e.g. minified
# output), revisit.
#
# Usage: fhevm_chain_address <chain_name> <key>
# 
# acl_addr="$(fhevm_chain_address localstack acl)"   
# acl_addr="$(fhevm_chain_address localhost acl)"   
fhevm_chain_address() {
    local chain="$1"
    local key="$2"

    if [[ -z "$chain" || -z "$key" ]]; then
        echo "fhevm_chain_address: missing argument(s); usage: fhevm_chain_address <chain_name> <key>" >&2
        return 2
    fi

    local file
    file="$(fhevm_chain_file "$chain")" || return 1
    if [[ ! -f "$file" ]]; then
        echo "fhevm_chain_address: chain file not found: $file" >&2
        return 1
    fi

    local addr
    addr="$(grep -A 1 "${key}:" "$file" | grep -oE "0x[0-9a-fA-F]{40}" | head -n 1)"
    if [[ -z "$addr" ]]; then
        echo "fhevm_chain_address: address for key '$key' not found in $file" >&2
        return 1
    fi
    printf '%s\n' "$addr"
}
