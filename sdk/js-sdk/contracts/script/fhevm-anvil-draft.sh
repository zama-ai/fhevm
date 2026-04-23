#!/usr/bin/env bash
# =============================================================================
# fhevm-anvil.sh — Start a local anvil node and deploy the FHEVM stack onto it.
#
# Convenience wrapper for local dev:
#   1. Derive KMS / coprocessor signer keys from a BIP-39 mnemonic (deterministic).
#   2. Launch anvil in the background on the chosen port.
#   3. Wait for the JSON-RPC endpoint to become ready.
#   4. Invoke ./deploy-local.sh against that endpoint with the derived keys exported.
#   5. Keep anvil running in the foreground; Ctrl-C tears it down cleanly.
#
# Derivation paths (BIP-44, Ethereum coin type):
#   Deployer             : m/44'/60'/0'/1/0
#   Coprocessor signer i : m/44'/60'/0'/2/i
#   KMS signer i         : m/44'/60'/0'/3/i
#   Input verification   : m/44'/60'/0'/4/0
#   Decryption           : m/44'/60'/0'/4/1
#
# Both the mnemonic and the counts are overridable — see --help.
# =============================================================================
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

# Default "Zama test mnemonic" — deterministic keys for local dev only. Not for production.
DEFAULT_MNEMONIC="test test test test test test test future home engine virtual motion"

print_usage() {
    cat <<'EOF'
Usage: ./fhevm-anvil.sh [options] [-- <anvil args>]

Start an anvil node and deploy the fixed-address local FHEVM stack onto it,
with KMS / coprocessor signer private keys deterministically derived from a
BIP-39 mnemonic.

Options:
  --port <port>           Port for anvil (default: 8545).
  --chain-id <id>         Chain id for anvil (default: anvil's own, 31337).
  --block-time <sec>      Pass-through --block-time to anvil.
  --kms-count <N>         Number of KMS signers to derive (default: 1).
  --coproc-count <N>      Number of coprocessor signers to derive (default: 1).
  --mnemonic <phrase>     Mnemonic used for key derivation.
                          Also overridable via MNEMONIC env var.
  --skip-build            Forward --skip-build to deploy-local.sh.
  --deploy-timeout <s>    How long to wait for anvil readiness (default: 30s).
  -v, --verbose           Forward --verbose to deploy-local.sh.
  -h, --help              Show this help.

Anything after `--` is forwarded to anvil unchanged, e.g.:
  ./fhevm-anvil.sh --port 8546 -- --accounts 20 --balance 10000
EOF
}

PORT=8545
CHAIN_ID=""
BLOCK_TIME=""
DEPLOY_TIMEOUT=30
KMS_COUNT=1
COPROC_COUNT=1
MNEMONIC="${MNEMONIC:-$DEFAULT_MNEMONIC}"
declare -a DEPLOY_ARGS=()
declare -a ANVIL_EXTRA=()

while [[ $# -gt 0 ]]; do
    case "$1" in
        --port)
            PORT="${2:?--port requires a value}"
            shift 2
            ;;
        --chain-id)
            CHAIN_ID="${2:?--chain-id requires a value}"
            shift 2
            ;;
        --block-time)
            BLOCK_TIME="${2:?--block-time requires a value}"
            shift 2
            ;;
        --deploy-timeout)
            DEPLOY_TIMEOUT="${2:?--deploy-timeout requires a value}"
            shift 2
            ;;
        --kms-count)
            KMS_COUNT="${2:?--kms-count requires a value}"
            shift 2
            ;;
        --coproc-count)
            COPROC_COUNT="${2:?--coproc-count requires a value}"
            shift 2
            ;;
        --mnemonic)
            MNEMONIC="${2:?--mnemonic requires a value}"
            shift 2
            ;;
        --skip-build)
            DEPLOY_ARGS+=(--skip-build)
            shift
            ;;
        -v|--verbose)
            DEPLOY_ARGS+=(-v)
            shift
            ;;
        -h|--help)
            print_usage
            exit 0
            ;;
        --)
            shift
            ANVIL_EXTRA=("$@")
            break
            ;;
        *)
            echo "Error: unknown option: $1" >&2
            print_usage >&2
            exit 1
            ;;
    esac
done

if ! [[ "$KMS_COUNT" =~ ^[0-9]+$ ]] || (( KMS_COUNT < 1 )); then
    echo "Error: --kms-count must be a positive integer (got '$KMS_COUNT')." >&2
    exit 1
fi

if ! [[ "$COPROC_COUNT" =~ ^[0-9]+$ ]] || (( COPROC_COUNT < 1 )); then
    echo "Error: --coproc-count must be a positive integer (got '$COPROC_COUNT')." >&2
    exit 1
fi

if ! command -v anvil >/dev/null 2>&1; then
    echo "Error: anvil not found in PATH. Install Foundry: https://book.getfoundry.sh/getting-started/installation" >&2
    exit 1
fi

if ! command -v cast >/dev/null 2>&1; then
    echo "Error: cast not found in PATH." >&2
    exit 1
fi

derive_private_key() {
    local path="$1"
    cast wallet private-key --mnemonic "$MNEMONIC" --mnemonic-derivation-path "$path"
}

derive_address() {
    local path="$1"
    cast wallet address --mnemonic "$MNEMONIC" --mnemonic-derivation-path "$path"
}

# ---- Derive KMS / coprocessor signer keys from the mnemonic ----

echo "Deriving signer keys from mnemonic..."
for ((i = 0; i < COPROC_COUNT; i++)); do
    pk="$(derive_private_key "m/44'/60'/0'/2/$i")"
    export "COPROCESSOR_SIGNER_PRIVATE_KEY_$i=$pk"
done
for ((i = 0; i < KMS_COUNT; i++)); do
    pk="$(derive_private_key "m/44'/60'/0'/3/$i")"
    export "KMS_SIGNER_PRIVATE_KEY_$i=$pk"
done

# Threshold defaults to N (unanimous) unless the caller has overridden it.
export PUBLIC_DECRYPTION_THRESHOLD="${PUBLIC_DECRYPTION_THRESHOLD:-$KMS_COUNT}"
export COPROCESSOR_THRESHOLD="${COPROCESSOR_THRESHOLD:-$COPROC_COUNT}"

# Input verification address & Decryption verification address
export INPUT_VERIFICATION_ADDRESS="${INPUT_VERIFICATION_ADDRESS:-$(derive_address "m/44'/60'/0'/4/0")}"
export DECRYPTION_ADDRESS="${DECRYPTION_ADDRESS:-$(derive_address "m/44'/60'/0'/4/1")}"

# Deployer
export DEPLOYER_PRIVATE_KEY="${DEPLOYER_PRIVATE_KEY:-$(derive_private_key "m/44'/60'/0'/1/0")}"
export FHE_TEST_USER_PRIVATE_KEY="${FHE_TEST_USER_PRIVATE_KEY:-$(derive_private_key "m/44'/60'/0'/0/0")}"

echo "  KMS signers        : $KMS_COUNT (threshold: $PUBLIC_DECRYPTION_THRESHOLD)"
echo "  Coprocessor signers: $COPROC_COUNT (threshold: $COPROCESSOR_THRESHOLD)"

echo "  Deployer           : $(cast wallet address --private-key "$DEPLOYER_PRIVATE_KEY")"
echo "  Input verification : $INPUT_VERIFICATION_ADDRESS"
echo "  Decryption         : $DECRYPTION_ADDRESS"

# ---- Launch anvil ----

export RPC_URL="http://127.0.0.1:${PORT}"
export CHAIN_ID_GATEWAY="${CHAIN_ID_GATEWAY:-31337}"
export BROADCAST="${BROADCAST:---broadcast}"

if cast chain-id --rpc-url "$RPC_URL" >/dev/null 2>&1; then
    echo "Error: something is already listening on $RPC_URL. Stop it or pick another --port." >&2
    exit 1
fi

declare -a ANVIL_ARGS=(--port "$PORT")
if [[ -n "$CHAIN_ID" ]]; then
    ANVIL_ARGS+=(--chain-id "$CHAIN_ID")
fi
if [[ -n "$BLOCK_TIME" ]]; then
    ANVIL_ARGS+=(--block-time "$BLOCK_TIME")
fi
if [[ ${#ANVIL_EXTRA[@]} -gt 0 ]]; then
    ANVIL_ARGS+=("${ANVIL_EXTRA[@]}")
fi

echo "🚚 Launching anvil on ${RPC_URL}..."
anvil "${ANVIL_ARGS[@]}" &
ANVIL_PID=$!

cleanup() {
    if [[ -n "${ANVIL_PID:-}" ]] && kill -0 "$ANVIL_PID" 2>/dev/null; then
        kill "$ANVIL_PID" 2>/dev/null || true
        wait "$ANVIL_PID" 2>/dev/null || true
    fi
}
trap cleanup EXIT INT TERM

echo "🛺 Waiting for anvil to become ready (timeout: ${DEPLOY_TIMEOUT}s)..."
DEADLINE=$(( $(date +%s) + DEPLOY_TIMEOUT ))
until cast chain-id --rpc-url "$RPC_URL" >/dev/null 2>&1; do
    if ! kill -0 "$ANVIL_PID" 2>/dev/null; then
        echo "Error: anvil exited before becoming ready." >&2
        exit 1
    fi
    if (( $(date +%s) > DEADLINE )); then
        echo "Error: anvil did not become ready within ${DEPLOY_TIMEOUT}s." >&2
        exit 1
    fi
    sleep 0.2
done
echo "anvil is ready."

DEPLOYER_ADDRESS="$(cast wallet address --private-key "$DEPLOYER_PRIVATE_KEY")"
echo "🍟 Funding ${DEPLOYER_ADDRESS} with 10000 ETH"
# 0x21e19e0c9bab2400000 wei == 10000 ETH
cast rpc anvil_setBalance "$DEPLOYER_ADDRESS" "0x21e19e0c9bab2400000" --rpc-url "$RPC_URL" >/dev/null

FHE_TEST_USER_ADDRESS="$(cast wallet address --private-key "$FHE_TEST_USER_PRIVATE_KEY")"
echo "🍟 Funding ${FHE_TEST_USER_ADDRESS} with 10000 ETH"
# 0x21e19e0c9bab2400000 wei == 10000 ETH
cast rpc anvil_setBalance "$FHE_TEST_USER_ADDRESS" "0x21e19e0c9bab2400000" --rpc-url "$RPC_URL" >/dev/null

echo "🍔 Running ./deploy.sh against ${RPC_URL}..."
./deploy.sh

# ---- Install FhevmCheats on the live anvil ----
#
# `DeployFHEVMHost.s.sol` calls `_installFhevmCheats()` + `fhevmCheats.setAll(...)`
# inside the forge run, but `vm.etch` is a simulation-only cheat — it doesn't
# bridge to the live node during broadcast. So we plant the contract and
# populate storage here via anvil_setCode + setAll(), using the runtime
# bytecode from the forge build and the addresses from FHEVMHostAddresses.sol.

CHEATS_ARTIFACT="out/FhevmCheats.sol/FhevmCheats.json"
if [[ ! -f "$CHEATS_ARTIFACT" ]]; then
    echo "Error: $CHEATS_ARTIFACT not found — did forge build run?" >&2
    exit 1
fi

CHEATS_HASH="$(cast keccak "fhevm cheat code")"
FHEVM_CHEATS_ADDRESS="0x${CHEATS_HASH: -40}"
CHEATS_RUNTIME="$(jq -r '.deployedBytecode.object // .deployedBytecode' "$CHEATS_ARTIFACT")"

extract_fhevm_addr() {
    local name="$1"
    local src="src/fhevm-host/addresses/FHEVMHostAddresses.sol"
    sed -nE "s/^[[:space:]]*address[[:space:]]+constant[[:space:]]+${name}[[:space:]]*=[[:space:]]*address\(([^)]+)\).*/\1/p" "$src"
}

ACL_ADD="$(extract_fhevm_addr aclAdd)"
FHEVM_EXECUTOR_ADD="$(extract_fhevm_addr fhevmExecutorAdd)"
KMS_VERIFIER_ADD="$(extract_fhevm_addr kmsVerifierAdd)"
INPUT_VERIFIER_ADD="$(extract_fhevm_addr inputVerifierAdd)"
HCU_LIMIT_ADD="$(extract_fhevm_addr hcuLimitAdd)"
PAUSER_SET_ADD="$(extract_fhevm_addr pauserSetAdd)"

# FHETest is deployed fresh by DeployFHEVMHost.s.sol — pull its address from
# the forge broadcast log (FHEVMHostAddresses.sol only holds the fixed proxy
# addresses).
CHAIN_ID_DEC="$(($(cast chain-id --rpc-url "$RPC_URL")))"
BROADCAST_LOG="broadcast/DeployFHEVMHost.s.sol/${CHAIN_ID_DEC}/run-latest.json"
if [[ ! -f "$BROADCAST_LOG" ]]; then
    echo "Error: $BROADCAST_LOG not found — forge broadcast didn't write a log for chain ${CHAIN_ID_DEC}." >&2
    exit 1
fi
FHE_TEST_ADD="$(jq -r '[.transactions[] | select(.contractName == "FHETest") | .contractAddress] | last // ""' "$BROADCAST_LOG")"
if [[ -z "$FHE_TEST_ADD" || "$FHE_TEST_ADD" == "null" ]]; then
    echo "Error: FHETest contract address not found in $BROADCAST_LOG." >&2
    exit 1
fi

echo "🧩 Installing FhevmCheats at ${FHEVM_CHEATS_ADDRESS}"
cast rpc anvil_setCode "$FHEVM_CHEATS_ADDRESS" "$CHEATS_RUNTIME" --rpc-url "$RPC_URL" >/dev/null

echo "🧩 Populating FhevmCheats.setAll(...)"
cast send "$FHEVM_CHEATS_ADDRESS" \
    "setAll(address,address,address,address,address,address,address)" \
    "$ACL_ADD" "$FHEVM_EXECUTOR_ADD" "$KMS_VERIFIER_ADD" \
    "$INPUT_VERIFIER_ADD" "$HCU_LIMIT_ADD" "$PAUSER_SET_ADD" "$FHE_TEST_ADD" \
    --private-key "$DEPLOYER_PRIVATE_KEY" --rpc-url "$RPC_URL" >/dev/null

echo
echo "🔎 Verifying FhevmCheats registry"
for getter in acl fhevmExecutor kmsVerifier inputVerifier hcuLimit pauserSet fheTest; do
    value="$(cast call "$FHEVM_CHEATS_ADDRESS" "${getter}()(address)" --rpc-url "$RPC_URL" 2>/dev/null || echo "")"
    printf "  %-16s : %s\n" "$getter" "${value:-<no code at slot>}"
done

echo
echo "🔎 Coprocessor signers (via InputVerifier.getCoprocessorSigners())"
signers_raw="$(cast call "$INPUT_VERIFIER_ADD" "getCoprocessorSigners()(address[])" --rpc-url "$RPC_URL" 2>/dev/null || echo "")"
if [[ -z "$signers_raw" ]]; then
    echo "  <call failed — InputVerifier not reachable at $INPUT_VERIFIER_ADD>"
else
    # cast returns "[addr1, addr2, ...]" — strip brackets and split on commas.
    signers_clean="${signers_raw#[}"
    signers_clean="${signers_clean%]}"
    i=0
    IFS=',' read -ra signer_arr <<< "$signers_clean"
    for s in "${signer_arr[@]}"; do
        # Trim whitespace
        s_trimmed="$(echo "$s" | tr -d '[:space:]')"
        printf "  [%d] %s\n" "$i" "$s_trimmed"
        i=$((i + 1))
    done
    if (( i == 0 )); then
        echo "  <empty signer list>"
    fi
fi

echo
echo "FHEVM stack deployed on ${RPC_URL} (chain-id: $(cast chain-id --rpc-url "$RPC_URL"))."
echo "anvil is running as PID ${ANVIL_PID}. Press Ctrl-C to stop."
wait "$ANVIL_PID"

# Deployer : cast wallet address --mnemonic "test test test test test test test future home engine virtual motion" --mnemonic-derivation-path "m/44'/60'/0'/1/0"
# aclAddress = cast compute-address 0xE58DB03a451AF600124C0Ad5A86c321FEcDD9fe7 --nonce 1
# fhevmExecutor = cast compute-address 0xE58DB03a451AF600124C0Ad5A86c321FEcDD9fe7 --nonce 3
# kmsVerifier = cast compute-address 0xE58DB03a451AF600124C0Ad5A86c321FEcDD9fe7 --nonce 5
# inputVerifier = cast compute-address 0xE58DB03a451AF600124C0Ad5A86c321FEcDD9fe7 --nonce 7
# hcuLimit = cast compute-address 0xE58DB03a451AF600124C0Ad5A86c321FEcDD9fe7 --nonce 9
# pauserSet = cast compute-address 0xE58DB03a451AF600124C0Ad5A86c321FEcDD9fe7 --nonce 10
#   acl          : 0x7588E52a58D2ddDe340839353012015F036F8135
#   fhevmExecutor: 0xBC12d9D87b899BC8B740CE58589a491e403FDB4b
#   kmsVerifier  : 0x611abC2CaA700A0b26E4B18e00ceBB8beBeF21e7
#   inputVerifier: 0x4d4843FF21CE703067B2575E01Dbe4BfE9b736AF
#   hcuLimit     : 0xe7202AEF2Ef29785889c185eBE89864ef6B1e68e
#   pauserSet    : 0xcf8e989db234b3ef330D7a5cE144fEBd57F77429

# MNEMONIC-FHEVM-REPO : "adapt mosquito move limb mobile illegal tree voyage juice mosquito burger raise father hope layer"
# DEPLOYER_PRIVATE_KEY: 0x7697c90f7863e6057fbe25674464e14b57f2c670b1a8ee0f60fb87eb9b615c4d
# DEPLOYER_ADDRESS: 0x8B8f5091f8b9817EF69cFC1E8B2f721BafF60DF4
# cast wallet private-key --mnemonic "adapt mosquito move limb mobile illegal tree voyage juice mosquito burger raise father hope layer" --mnemonic-derivation-path "m/44'/60'/0'/0/5"
# aclAddress = cast compute-address 0x8B8f5091f8b9817EF69cFC1E8B2f721BafF60DF4 --nonce 1
# CoprocessorAddress = cast compute-address 0x8B8f5091f8b9817EF69cFC1E8B2f721BafF60DF4 --nonce 3
# kmsVerifier = cast compute-address 0x8B8f5091f8b9817EF69cFC1E8B2f721BafF60DF4 --nonce 4
#
# ACLAddress: 0x50157CFfD6bBFA2DECe204a89ec419c23ef5755D,
# CoprocessorAddress: 0xe3a9105a3a932253A70F126eb1E3b589C643dD24,
# KMSVerifierAddress: 0x901F8942346f7AB3a01F6D7613119Bca447Bb030

# Deployer : cast wallet address --mnemonic "test test test test test test test future home engine virtual motion" --mnemonic-derivation-path "m/44'/60'/0'/4/0"