#!/usr/bin/env bash
# =============================================================================
# fhevm-info.sh - Generate FHEVM host deployment information as JSON
#
# The ACL address is the root for resolving host addresses exposed by ACL and
# FHEVMExecutor. The KMSVerifier address is provided separately because it is not
# exposed by the current host contract getters.
# =============================================================================
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"
source "$SCRIPT_DIR/lib-common.sh"

print_usage() {
    cat <<'EOF'
Usage:
  ./fhevm-info.sh <acl-address> <kms-verifier-address> [options]
  ./fhevm-info.sh --acl <address> --kms-verifier <address> [options]

Generate FHEVM host deployment information as JSON from on-chain getters.

Options:
  --acl <address>             ACL contract address.
  --kms-verifier <address>    KMSVerifier contract address.
  --rpc-url <url>             Target RPC URL.
  --anvil-port <port>         Shorthand for http://127.0.0.1:<port>.
  -h, --help                  Show this help text.

Notes:
  - If no RPC target is provided, the script uses RPC_URL, then ANVIL_PORT, then 8545.

Example:
  ./fhevm-info.sh --acl 0x05fD9B5EFE0a996095f42Ed7e77c390810CF660c --kms-verifier 0xa1880e99d86F081E8D3868A8C4732C8f65dfdB11 --rpc-url http://localhost:8545
EOF
}

die() {
    echo "Error: $*" >&2
    exit 1
}

require_arg_value() {
    local option_name="$1"
    local option_value="${2:-}"

    if [[ -z "$option_value" || "$option_value" == -* ]]; then
        die "${option_name} requires a value"
    fi
}

is_address() {
    [[ "$1" =~ ^0x[0-9a-fA-F]{40}$ ]]
}

require_address() {
    local name="$1"
    local address="$2"

    if ! is_address "$address"; then
        die "${name} must be a 20-byte hex address"
    fi
}

require_command() {
    local command_name="$1"

    if ! command -v "$command_name" >/dev/null 2>&1; then
        die "${command_name} is required"
    fi
}

cast_view() {
    local contract_address="$1"
    local signature="$2"
    shift 2

    cast call "$contract_address" "$signature" "$@" --rpc-url "$RPC_URL"
}

contract_has_code() {
    local contract_address="$1"
    local code

    code="$(cast code "$contract_address" --rpc-url "$RPC_URL")"
    [[ -n "$code" && "$code" != "0x" ]]
}

require_contract_code() {
    local name="$1"
    local contract_address="$2"

    if ! contract_has_code "$contract_address"; then
        die "${name} has no code at ${contract_address} on ${RPC_URL}"
    fi
}

clean_cast_string() {
    local value="$1"

    value="${value%$'\r'}"
    if [[ "$value" == \"*\" ]]; then
        value="${value#\"}"
        value="${value%\"}"
    fi
    printf '%s\n' "$value"
}

clean_cast_uint() {
    local value="$1"

    value="${value%$'\r'}"
    printf '%s\n' "${value%% *}"
}

address_array_json() {
    local value="$1"
    local addresses

    addresses="$(printf '%s\n' "$value" | grep -Eio '0x[0-9a-f]{40}' || true)"
    if [[ -z "$addresses" ]]; then
        printf '[]\n'
        return
    fi

    printf '%s\n' "$addresses" | jq -R . | jq -s .
}

uint_array_json() {
    local value="$1"
    local values

    values="$(printf '%s\n' "$value" | grep -Eo '[0-9]+' || true)"
    if [[ -z "$values" ]]; then
        printf '[]\n'
        return
    fi

    printf '%s\n' "$values" | jq -R 'tonumber' | jq -s .
}

eip712_domain_json() {
    local contract_address="$1"
    local raw
    local name
    local version
    local chain_id
    local verifying_contract

    raw="$(cast_view "$contract_address" "eip712Domain()(bytes1,string,string,uint256,address,bytes32,uint256[])")"

    name="$(clean_cast_string "$(printf '%s\n' "$raw" | sed -n '2p')")"
    version="$(clean_cast_string "$(printf '%s\n' "$raw" | sed -n '3p')")"
    chain_id="$(clean_cast_uint "$(printf '%s\n' "$raw" | sed -n '4p')")"
    verifying_contract="$(printf '%s\n' "$raw" | sed -n '5p')"

    jq -n \
        --arg name "$name" \
        --arg version "$version" \
        --argjson chainId "$chain_id" \
        --arg verifyingContract "$verifying_contract" \
        '{
            name: $name,
            version: $version,
            chainId: $chainId,
            verifyingContract: $verifyingContract
        }'
}

ACL_ADDRESS=""
KMS_VERIFIER_ADDRESS=""
declare -a POSITIONAL_ARGS=()

while [[ $# -gt 0 ]]; do
    case "$1" in
        --acl)
            require_arg_value "$1" "${2:-}"
            ACL_ADDRESS="$2"
            shift 2
            ;;
        --kms-verifier)
            require_arg_value "$1" "${2:-}"
            KMS_VERIFIER_ADDRESS="$2"
            shift 2
            ;;
        --rpc-url)
            require_arg_value "$1" "${2:-}"
            RPC_URL="$2"
            shift 2
            ;;
        --anvil-port)
            require_arg_value "$1" "${2:-}"
            RPC_URL="http://127.0.0.1:$2"
            shift 2
            ;;
        -h|--help)
            print_usage
            exit 0
            ;;
        -*)
            die "unknown option: $1"
            ;;
        *)
            POSITIONAL_ARGS+=("$1")
            shift
            ;;
    esac
done

if [[ ${#POSITIONAL_ARGS[@]} -gt 2 ]]; then
    die "too many positional arguments"
fi

if [[ -z "$ACL_ADDRESS" && ${#POSITIONAL_ARGS[@]} -ge 1 ]]; then
    ACL_ADDRESS="${POSITIONAL_ARGS[0]}"
fi

if [[ -z "$KMS_VERIFIER_ADDRESS" && ${#POSITIONAL_ARGS[@]} -ge 2 ]]; then
    KMS_VERIFIER_ADDRESS="${POSITIONAL_ARGS[1]}"
fi

if [[ -z "$ACL_ADDRESS" || -z "$KMS_VERIFIER_ADDRESS" ]]; then
    print_usage >&2
    exit 1
fi

if [[ -z "${RPC_URL:-}" ]]; then
    if [[ -n "${ANVIL_PORT:-}" ]]; then
        RPC_URL="http://127.0.0.1:${ANVIL_PORT}"
    else
        RPC_URL="http://127.0.0.1:8545"
    fi
fi

require_command cast
require_command jq

require_address "ACL address" "$ACL_ADDRESS"
require_address "KMSVerifier address" "$KMS_VERIFIER_ADDRESS"

CHAIN_ID="$(cast chain-id --rpc-url "$RPC_URL")"

require_contract_code "ACL" "$ACL_ADDRESS"
require_contract_code "KMSVerifier" "$KMS_VERIFIER_ADDRESS"

ACL_OWNER_ADDRESS="$(cast_view "$ACL_ADDRESS" "owner()(address)")"
FHEVM_EXECUTOR_ADDRESS="$(cast_view "$ACL_ADDRESS" "getFHEVMExecutorAddress()(address)")"
PAUSER_SET_ADDRESS="$(cast_view "$ACL_ADDRESS" "getPauserSetAddress()(address)")"

require_contract_code "FHEVMExecutor" "$FHEVM_EXECUTOR_ADDRESS"
require_contract_code "PauserSet" "$PAUSER_SET_ADDRESS"

EXECUTOR_ACL_ADDRESS="$(cast_view "$FHEVM_EXECUTOR_ADDRESS" "getACLAddress()(address)")"
INPUT_VERIFIER_ADDRESS="$(cast_view "$FHEVM_EXECUTOR_ADDRESS" "getInputVerifierAddress()(address)")"
HCU_LIMIT_ADDRESS="$(cast_view "$FHEVM_EXECUTOR_ADDRESS" "getHCULimitAddress()(address)")"

require_contract_code "InputVerifier" "$INPUT_VERIFIER_ADDRESS"
require_contract_code "HCULimit" "$HCU_LIMIT_ADDRESS"

HCU_LIMIT_EXECUTOR_ADDRESS="$(cast_view "$HCU_LIMIT_ADDRESS" "getFHEVMExecutorAddress()(address)")"

if [[ "$(normalize_address "$EXECUTOR_ACL_ADDRESS")" != "$(normalize_address "$ACL_ADDRESS")" ]]; then
    die "FHEVMExecutor.getACLAddress() returned ${EXECUTOR_ACL_ADDRESS}, expected ${ACL_ADDRESS}"
fi

if [[ "$(normalize_address "$HCU_LIMIT_EXECUTOR_ADDRESS")" != "$(normalize_address "$FHEVM_EXECUTOR_ADDRESS")" ]]; then
    die "HCULimit.getFHEVMExecutorAddress() returned ${HCU_LIMIT_EXECUTOR_ADDRESS}, expected ${FHEVM_EXECUTOR_ADDRESS}"
fi

ACL_VERSION="$(clean_cast_string "$(cast_view "$ACL_ADDRESS" "getVersion()(string)")")"
FHEVM_EXECUTOR_VERSION="$(clean_cast_string "$(cast_view "$FHEVM_EXECUTOR_ADDRESS" "getVersion()(string)")")"
FHEVM_EXECUTOR_HANDLE_VERSION="$(clean_cast_uint "$(cast_view "$FHEVM_EXECUTOR_ADDRESS" "getHandleVersion()(uint8)")")"
KMS_VERIFIER_VERSION="$(clean_cast_string "$(cast_view "$KMS_VERIFIER_ADDRESS" "getVersion()(string)")")"
INPUT_VERIFIER_VERSION="$(clean_cast_string "$(cast_view "$INPUT_VERIFIER_ADDRESS" "getVersion()(string)")")"
INPUT_VERIFIER_HANDLE_VERSION="$(clean_cast_uint "$(cast_view "$INPUT_VERIFIER_ADDRESS" "getHandleVersion()(uint8)")")"
HCU_LIMIT_VERSION="$(clean_cast_string "$(cast_view "$HCU_LIMIT_ADDRESS" "getVersion()(string)")")"
PAUSER_SET_VERSION="$(clean_cast_string "$(cast_view "$PAUSER_SET_ADDRESS" "getVersion()(string)")")"

if [[ "$FHEVM_EXECUTOR_HANDLE_VERSION" != "$INPUT_VERIFIER_HANDLE_VERSION" ]]; then
    die "handle version mismatch: FHEVMExecutor=${FHEVM_EXECUTOR_HANDLE_VERSION}, InputVerifier=${INPUT_VERIFIER_HANDLE_VERSION}"
fi
HANDLE_VERSION="$FHEVM_EXECUTOR_HANDLE_VERSION"

KMS_SIGNERS_JSON="$(address_array_json "$(cast_view "$KMS_VERIFIER_ADDRESS" "getKmsSigners()(address[])")")"
COPROCESSOR_SIGNERS_JSON="$(address_array_json "$(cast_view "$INPUT_VERIFIER_ADDRESS" "getCoprocessorSigners()(address[])")")"

KMS_THRESHOLD="$(clean_cast_uint "$(cast_view "$KMS_VERIFIER_ADDRESS" "getThreshold()(uint256)")")"
INPUT_THRESHOLD="$(clean_cast_uint "$(cast_view "$INPUT_VERIFIER_ADDRESS" "getThreshold()(uint256)")")"

#KMS_CONTEXT_ID="$(clean_cast_uint "$(cast_view "$KMS_VERIFIER_ADDRESS" "getCurrentKmsContextId()(uint256)")")"

#HCU_CAP_PER_BLOCK="$(clean_cast_uint "$(cast_view "$HCU_LIMIT_ADDRESS" "getGlobalHCUCapPerBlock()(uint48)")")"
#MAX_HCU_DEPTH_PER_TX="$(clean_cast_uint "$(cast_view "$HCU_LIMIT_ADDRESS" "getMaxHCUDepthPerTx()(uint48)")")"
#MAX_HCU_PER_TX="$(clean_cast_uint "$(cast_view "$HCU_LIMIT_ADDRESS" "getMaxHCUPerTx()(uint48)")")"

KMS_EIP712_DOMAIN_JSON="$(eip712_domain_json "$KMS_VERIFIER_ADDRESS")"
INPUT_EIP712_DOMAIN_JSON="$(eip712_domain_json "$INPUT_VERIFIER_ADDRESS")"
GATEWAY_CHAIN_ID="$(jq -r '.chainId' <<<"$KMS_EIP712_DOMAIN_JSON")"

jq -n \
    --arg rpcUrl "$RPC_URL" \
    --argjson chainId "$CHAIN_ID" \
    --arg acl "$ACL_ADDRESS" \
    --arg kmsVerifier "$KMS_VERIFIER_ADDRESS" \
    --arg fhevmExecutor "$FHEVM_EXECUTOR_ADDRESS" \
    --arg inputVerifier "$INPUT_VERIFIER_ADDRESS" \
    --arg hcuLimit "$HCU_LIMIT_ADDRESS" \
    --arg pauserSet "$PAUSER_SET_ADDRESS" \
    --arg aclOwner "$ACL_OWNER_ADDRESS" \
    --arg aclVersion "$ACL_VERSION" \
    --arg fhevmExecutorVersion "$FHEVM_EXECUTOR_VERSION" \
    --argjson handleVersion "$HANDLE_VERSION" \
    --arg kmsVerifierVersion "$KMS_VERIFIER_VERSION" \
    --arg inputVerifierVersion "$INPUT_VERIFIER_VERSION" \
    --arg hcuLimitVersion "$HCU_LIMIT_VERSION" \
    --arg pauserSetVersion "$PAUSER_SET_VERSION" \
    --argjson kmsSigners "$KMS_SIGNERS_JSON" \
    --argjson coprocessorSigners "$COPROCESSOR_SIGNERS_JSON" \
    --argjson kmsThreshold "$KMS_THRESHOLD" \
    --argjson inputThreshold "$INPUT_THRESHOLD" \
    --argjson gatewayChainId "$GATEWAY_CHAIN_ID" \
    --argjson kmsEip712Domain "$KMS_EIP712_DOMAIN_JSON" \
    --argjson inputEip712Domain "$INPUT_EIP712_DOMAIN_JSON" \
    '{
        rpc: {
            url: $rpcUrl
        },
        chainId: $chainId,
        hostContracts: {
            acl: $acl,
            kmsVerifier: $kmsVerifier,
            fhevmExecutor: $fhevmExecutor,
            inputVerifier: $inputVerifier,
            hcuLimit: $hcuLimit,
            pauserSet: $pauserSet
        },
        owners: {
            acl: $aclOwner,
            kmsVerifier: $aclOwner,
            fhevmExecutor: $aclOwner,
            inputVerifier: $aclOwner,
            hcuLimit: $aclOwner,
            pauserSet: $aclOwner
        },
        handleVersion: $handleVersion,
        gatewayChainId: $gatewayChainId,
        versions: {
            acl: $aclVersion,
            fhevmExecutor: $fhevmExecutorVersion,
            kmsVerifier: $kmsVerifierVersion,
            inputVerifier: $inputVerifierVersion,
            hcuLimit: $hcuLimitVersion,
            pauserSet: $pauserSetVersion
        },
        kmsVerifier: {
            signers: $kmsSigners,
            threshold: $kmsThreshold,
        },
        inputVerifier: {
            signers: $coprocessorSigners,
            threshold: $inputThreshold
        },
        eip712Domains: {
            kmsVerifier: $kmsEip712Domain,
            inputVerifier: $inputEip712Domain
        }
    }'
