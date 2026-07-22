#!/usr/bin/env bash
# =============================================================================
# lib-common.sh — Source-of-truth library for shell-side resolve / detect
# helpers used by both the test scripts (test/scripts/) and the deploy
# scripts (contracts/scripts/, which sources this via fhevm-lib.sh).
# =============================================================================

# Guard against double-sourcing — re-sourcing is harmless but wasteful.
if [[ -n "${__LIB_COMMON_SH_LOADED:-}" ]]; then
    return 0
fi
__LIB_COMMON_SH_LOADED=1

is_truthy() {
    case "${1:-}" in
        1|true|TRUE|True|yes|YES|on|ON)
            return 0
            ;;
        *)
            return 1
            ;;
    esac
}

detect_rpc_client() {
    cast client --rpc-url "$RPC_URL" 2>/dev/null | tr '[:upper:]' '[:lower:]'
}

# Resolves the RPC URL for a chain name. Precedence — matches the
# convention used by test/setupCommon.ts:
#   1. RPC_URL in test/.env.<chain> (or test/.env.localstack for any
#      localstack* variant — v11, v12, ... all share that file)
#   2. RPC_URL in the process environment
#   3. <chain>.rpcUrl in test/chains/chain-defaults.json
#      (the canonical source of truth for chain defaults)
#
# Errors if none provide a value or the chain has no entry in
# chain-defaults.json and no env override.
#
# Usage: resolve_chain_rpc_url <chain>
#
#   rpc_url="$(resolve_chain_rpc_url localcleartext)"
# Validates that <chain_name> is a key in chain-defaults.json. Exits 1
# (intentionally fail-fast — meant to be called at script entry) when the
# argument is empty or absent from the JSON.
#
# Usage: fhevm_assert_chain <chain_name>
fhevm_assert_chain() {
    local chain="$1"
    local test_dir
    test_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
    local chain_defaults_file="$test_dir/chains/chain-defaults.json"

    if [[ ! -f "$chain_defaults_file" ]]; then
        echo "❌ chain-defaults.json not found at $chain_defaults_file" >&2
        exit 1
    fi

    if [[ -z "$chain" ]]; then
        local valid
        valid="$(jq -r 'keys | join(" | ")' "$chain_defaults_file")"
        echo "❌ chain name is required (expected: $valid)" >&2
        exit 1
    fi

    if ! jq -e --arg c "$chain" 'has($c)' "$chain_defaults_file" >/dev/null; then
        local valid
        valid="$(jq -r 'keys | join(" | ")' "$chain_defaults_file")"
        echo "❌ unsupported chain '$chain' (expected: $valid)" >&2
        exit 1
    fi
}

# Resolves the chain TS file for a given fhevm chain name. Paths are computed
# relative to this library's location (assuming the standard SDK layout under
# sdk/js-sdk/).
#
# Public-network chains live under src/core/chains/definitions/; local /
# fixture chains live under test/chains/. The chain name doesn't
# always match the filename (testnet → sepolia.ts).
#
# Usage: fhevm_chain_file <chain_name>
fhevm_chain_file() {
    local chain="$1"
    local sdk_root
    sdk_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"

    case "$chain" in
        mainnet)
            # SDK file is named after the network (sepolia); testnet is an alias.
            printf '%s/src/core/chains/definitions/mainnet.ts\n' "$sdk_root"
            ;;
        sepolia|testnet)
            # SDK file is named after the network (sepolia); testnet is an alias.
            printf '%s/src/core/chains/definitions/sepolia.ts\n' "$sdk_root"
            ;;
        devnet|polygon_devnet|localcleartext|localstack|localstack_*|ingen_trex_*|hoodi_*)
            printf '%s/test/chains/%s.ts\n' "$sdk_root" "$chain"
            ;;
        *)
            echo "fhevm_chain_file: unsupported chain '$chain'" >&2
            return 1
            ;;
    esac
}

resolve_chain_rpc_url() {
    local chain="$1"
    local test_dir
    test_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

    local chain_defaults_file="$test_dir/chains/chain-defaults.json"
    if [[ ! -f "$chain_defaults_file" ]]; then
        echo "resolve_chain_rpc_url: chain-defaults.json not found at $chain_defaults_file" >&2
        return 1
    fi

    # All localstack* variants share .env.localstack; everything else uses .env.<chain>.
    local env_file
    if [[ "$chain" == localstack* ]]; then
        env_file="$test_dir/.env.localstack"
    else
        env_file="$test_dir/.env.$chain"
    fi

    local env_override=""
    if [[ -f "$env_file" ]]; then
        env_override="$(set -a; source "$env_file"; set +a; printf '%s' "${RPC_URL:-}")"
    fi

    local default_rpc_url
    default_rpc_url="$(jq -r --arg c "$chain" '.[$c].rpcUrl // ""' "$chain_defaults_file")"

    local rpc_url="${env_override:-${RPC_URL:-$default_rpc_url}}"

    if [[ -z "$rpc_url" ]]; then
        echo "resolve_chain_rpc_url: no RPC URL for chain '$chain' — set RPC_URL in $env_file, in the process env, or add a '${chain}.rpcUrl' entry to $chain_defaults_file" >&2
        return 1
    fi

    printf '%s\n' "$rpc_url"
}

resolve_fhetest_address() {
    local chain="$1"
    local chain_defaults_file="${2:-}"

    if [[ -z "$chain_defaults_file" ]]; then
        local test_dir
        test_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
        chain_defaults_file="$test_dir/chains/chain-defaults.json"
    fi

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

verify_acl_deploy() {
    local acl_address="$1"
    local expected_executor_address="$2"
    local expected_pauser_set_address="$3"
    local rpc_url="${4:-${RPC_URL:-}}"

    if [[ -z "$rpc_url" ]]; then
        echo "Error: verify_acl_deploy requires an RPC URL" >&2
        return 1
    fi

    local actual_executor_address
    local actual_pauser_set_address

    actual_executor_address="$(cast call "$acl_address" "getFHEVMExecutorAddress()(address)" --rpc-url "$rpc_url")"
    actual_pauser_set_address="$(cast call "$acl_address" "getPauserSetAddress()(address)" --rpc-url "$rpc_url")"

    if [[ "$(normalize_address "$actual_executor_address")" != "$(normalize_address "$expected_executor_address")" ]]; then
        echo "Error: ACL getFHEVMExecutorAddress() returned ${actual_executor_address}, expected ${expected_executor_address}" >&2
        return 1
    fi

    if [[ "$(normalize_address "$actual_pauser_set_address")" != "$(normalize_address "$expected_pauser_set_address")" ]]; then
        echo "Error: ACL getPauserSetAddress() returned ${actual_pauser_set_address}, expected ${expected_pauser_set_address}" >&2
        return 1
    fi
}

verify_fhevm_executor_deploy() {
    local executor_address="$1"
    local expected_acl_address="$2"
    local expected_input_verifier_address="$3"
    local expected_hcu_limit_address="$4"
    local rpc_url="${5:-${RPC_URL:-}}"

    if [[ -z "$rpc_url" ]]; then
        echo "Error: verify_fhevm_executor_deploy requires an RPC URL" >&2
        return 1
    fi

    local version
    local handle_version
    local actual_acl_address
    local actual_input_verifier_address
    local actual_hcu_limit_address

    version="$(cast call "$executor_address" "getVersion()(string)" --rpc-url "$rpc_url")"
    handle_version="$(cast call "$executor_address" "getHandleVersion()(uint8)" --rpc-url "$rpc_url")"
    actual_acl_address="$(cast call "$executor_address" "getACLAddress()(address)" --rpc-url "$rpc_url")"
    actual_input_verifier_address="$(cast call "$executor_address" "getInputVerifierAddress()(address)" --rpc-url "$rpc_url")"
    actual_hcu_limit_address="$(cast call "$executor_address" "getHCULimitAddress()(address)" --rpc-url "$rpc_url")"

    if [[ -z "$version" ]]; then
        echo "Error: FHEVMExecutor getVersion() returned an empty value" >&2
        return 1
    fi

    if [[ -z "$handle_version" ]]; then
        echo "Error: FHEVMExecutor getHandleVersion() returned an empty value" >&2
        return 1
    fi

    if [[ "$(normalize_address "$actual_acl_address")" != "$(normalize_address "$expected_acl_address")" ]]; then
        echo "Error: FHEVMExecutor getACLAddress() returned ${actual_acl_address}, expected ${expected_acl_address}" >&2
        return 1
    fi

    if [[ "$(normalize_address "$actual_input_verifier_address")" != "$(normalize_address "$expected_input_verifier_address")" ]]; then
        echo "Error: FHEVMExecutor getInputVerifierAddress() returned ${actual_input_verifier_address}, expected ${expected_input_verifier_address}" >&2
        return 1
    fi

    if [[ "$(normalize_address "$actual_hcu_limit_address")" != "$(normalize_address "$expected_hcu_limit_address")" ]]; then
        echo "Error: FHEVMExecutor getHCULimitAddress() returned ${actual_hcu_limit_address}, expected ${expected_hcu_limit_address}" >&2
        return 1
    fi
}

verify_input_verifier_deploy() {
    local input_verifier_address="$1"
    local expected_coprocessor_signer="$2"
    local expected_threshold="$3"
    local rpc_url="${4:-${RPC_URL:-}}"

    if [[ -z "$rpc_url" ]]; then
        echo "Error: verify_input_verifier_deploy requires an RPC URL" >&2
        return 1
    fi

    local version
    local handle_version
    local threshold
    local coprocessor_signers

    version="$(cast call "$input_verifier_address" "getVersion()(string)" --rpc-url "$rpc_url")"
    handle_version="$(cast call "$input_verifier_address" "getHandleVersion()(uint8)" --rpc-url "$rpc_url")"
    threshold="$(cast call "$input_verifier_address" "getThreshold()(uint256)" --rpc-url "$rpc_url")"
    coprocessor_signers="$(cast call "$input_verifier_address" "getCoprocessorSigners()(address[])" --rpc-url "$rpc_url")"

    if [[ -z "$version" ]]; then
        echo "Error: InputVerifier getVersion() returned an empty value" >&2
        return 1
    fi

    if [[ -z "$handle_version" ]]; then
        echo "Error: InputVerifier getHandleVersion() returned an empty value" >&2
        return 1
    fi

    if [[ "$threshold" != "$expected_threshold" ]]; then
        echo "Error: InputVerifier getThreshold() returned ${threshold}, expected ${expected_threshold}" >&2
        return 1
    fi

    if [[ "$(normalize_address "$coprocessor_signers")" != *"$(normalize_address "$expected_coprocessor_signer")"* ]]; then
        echo "Error: InputVerifier getCoprocessorSigners() returned ${coprocessor_signers}, expected ${expected_coprocessor_signer}" >&2
        return 1
    fi
}

verify_kms_verifier_deploy() {
    local kms_verifier_address="$1"
    local expected_kms_signer="$2"
    local expected_threshold="$3"
    local rpc_url="${4:-${RPC_URL:-}}"

    if [[ -z "$rpc_url" ]]; then
        echo "Error: verify_kms_verifier_deploy requires an RPC URL" >&2
        return 1
    fi

    local version
    local threshold
    local kms_signers

    version="$(cast call "$kms_verifier_address" "getVersion()(string)" --rpc-url "$rpc_url")"
    threshold="$(cast call "$kms_verifier_address" "getThreshold()(uint256)" --rpc-url "$rpc_url")"
    kms_signers="$(cast call "$kms_verifier_address" "getKmsSigners()(address[])" --rpc-url "$rpc_url")"

    if [[ -z "$version" ]]; then
        echo "Error: KMSVerifier getVersion() returned an empty value" >&2
        return 1
    fi

    if [[ "$threshold" != "$expected_threshold" ]]; then
        echo "Error: KMSVerifier getThreshold() returned ${threshold}, expected ${expected_threshold}" >&2
        return 1
    fi

    if [[ "$(normalize_address "$kms_signers")" != *"$(normalize_address "$expected_kms_signer")"* ]]; then
        echo "Error: KMSVerifier getKmsSigners() returned ${kms_signers}, expected ${expected_kms_signer}" >&2
        return 1
    fi
}

# write_fhevm_host_addresses() {
#     local acl_address="$1"
#     local executor_address="$2"
#     local kms_verifier_address="$3"
#     local input_verifier_address="$4"
#     local hcu_limit_address="$5"
#     local pauser_set_address="$6"

#     local addresses_file="$7"
#
#     cat >"$addresses_file" <<EOF
# // SPDX-License-Identifier: BSD-3-Clause-Clear

# pragma solidity ^0.8.24;

# // Auto-generated by deploy scripts - do not edit by hand.

# address constant aclAdd = address(${acl_address});

# address constant fhevmExecutorAdd = address(${executor_address});

# address constant kmsVerifierAdd = address(${kms_verifier_address});

# address constant inputVerifierAdd = address(${input_verifier_address});

# address constant hcuLimitAdd = address(${hcu_limit_address});

# address constant pauserSetAdd = address(${pauser_set_address});
# EOF
# }

# extract_address_constant_from_file() {
#     local addresses_file="$1"
#     local name="$2"
#     local result
#     result="$(sed -n "s/address constant ${name} = address(\\(0x[0-9A-Fa-f]*\\));/\\1/p" "$addresses_file")"
#     if [[ -z "$result" ]]; then
#         echo "Error: could not find address constant '${name}' in ${addresses_file}" >&2
#         return 1
#     fi
#     printf '%s\n' "$result"
# }
