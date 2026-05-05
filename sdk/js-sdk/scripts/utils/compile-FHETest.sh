#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/../../.." && pwd)"
CONTRACT_DIR="$(cd "$SCRIPT_DIR/../contracts" && pwd)"
CONTRACT="${CONTRACT_DIR}/FHETest.sol"
CONTRACT_NAME=FHETest
CONTRACT_NONCE=0
MNEMONIC="test test test test test test test future home engine virtual motion"
RPC_URL="http://localhost:8545"

# mnemonic(0) + nonce(0)
EXPECTED_CONTRACT_ADDR=0x61a8e950161daCCA06b0b623Efa2b7237BaBD4a0

# Diagnostics:
# forge tree

#
# Default Copressor Config addresses
# ----------------------------------
# cast wallet private-key "adapt mosquito move limb mobile illegal tree voyage juice mosquito burger raise father hope layer" 9 
# cast wallet address --private-key 0x2d24c36c57e6bfbf90c43173481cc00edcbd1a3922de5e5fdb9aba5fc4e0fafd
# cast compute-address 0xc45994e4098271c3140117ebD5c74C70dd56D9cd --nonce 1
# cast compute-address 0xc45994e4098271c3140117ebD5c74C70dd56D9cd --nonce 3
# cast compute-address 0xc45994e4098271c3140117ebD5c74C70dd56D9cd --nonce 4
# cast compute-address 0xc45994e4098271c3140117ebD5c74C70dd56D9cd --nonce 5
# cast compute-address 0xc45994e4098271c3140117ebD5c74C70dd56D9cd --nonce 6
#
# ACL           : nonce 1 : 0x05fD9B5EFE0a996095f42Ed7e77c390810CF660c
# FHEVMExecutor : nonce 3 : 0xcCAe95fF1d11656358E782570dF0418F59fA40e1
# KMSVerifier   : nonce 4 : 0xa1880e99d86F081E8D3868A8C4732C8f65dfdB11
# InputVerifier : nonce 5 : 0x857Ca72A957920Fa0FB138602995839866Bd4005
# HCULimit      : nonce 6 : 0xAb30999D17FAAB8c95B2eCD500cFeFc8f658f15d
#
ACL_ADDRESS=0x05fD9B5EFE0a996095f42Ed7e77c390810CF660c
COPROCESSOR_ADDRESS=0xcCAe95fF1d11656358E782570dF0418F59fA40e1
KMS_VERIFIER_ADDRESS=0xa1880e99d86F081E8D3868A8C4732C8f65dfdB11

# Make sure the fs environment is valid
if [ ! -f "$CONTRACT" ]; then
  echo "Error: Contract file not found: $CONTRACT" >&2
  exit 1
fi

#
# Compute private key
#
PRIVATE_KEY=$(cast wallet private-key "$MNEMONIC" 0)

#
# Compute deployer address
#
DEPLOYER=$(cast wallet address --private-key "$PRIVATE_KEY")

#
# Compute FHETest.sol future address using predefined nonce
#
DEPLOYED_ADDR=$(cast compute-address "$DEPLOYER" --nonce "${CONTRACT_NONCE}" | sed -n 's/.*: \(0x[0-9a-fA-F]*\).*/\1/p')

#
# Verify computed address matches expected (helper in case of future reconfig)
#
if [ "$(echo "$DEPLOYED_ADDR" | tr '[:upper:]' '[:lower:]')" != "$(echo "$EXPECTED_CONTRACT_ADDR" | tr '[:upper:]' '[:lower:]')" ]; then
  echo "Error: Computed address $DEPLOYED_ADDR does not match expected $EXPECTED_CONTRACT_ADDR" >&2
  exit 1
fi

#
# <fhevm>/library-solidity/node_modules
#
LIB_DIR="$REPO_ROOT/library-solidity"
LIB_NODE_MODULES="$LIB_DIR/node_modules"

#
# Setup remappings 
# ----------------
#   @fhevm/solidity -> <fhevm>/library-solidity
#   encrypted-types -> <fhevm>/library-solidity/node_modules
#   @openzeppelin -> <fhevm>/library-solidity/node_modules
#
REMAPPINGS=(\
  -R "@fhevm/solidity/=$LIB_DIR/" \
  -R "encrypted-types/=$LIB_NODE_MODULES/encrypted-types/" \
  -R "@openzeppelin/=$LIB_NODE_MODULES/@openzeppelin/" \
)

#
# Build: FHETest.sol
#
forge build "$CONTRACT" --root "$REPO_ROOT" "${REMAPPINGS[@]}"

#
# Check if FHETest.sol is already deployed
#
CODE_SIZE=$(cast codesize "$DEPLOYED_ADDR" --rpc-url "$RPC_URL" 2>/dev/null || echo "0")
if [ "$CODE_SIZE" != "0" ]; then
  echo "FHETest already deployed at $DEPLOYED_ADDR (code size: $CODE_SIZE bytes), skipping deploy."
else
  #
  # Make sure the deployer has sufficient balance
  #
  cast rpc anvil_setBalance "$DEPLOYER" 0x56BC75E2D63100000 --rpc-url "$RPC_URL" > /dev/null

  #
  # Compile and deploy FHETest.sol
  #
  echo "Deploying FHETest to $RPC_URL from $DEPLOYER..."
  DEPLOY_OUTPUT=$(forge create "$CONTRACT:$CONTRACT_NAME" \
    --root "$REPO_ROOT" \
    "${REMAPPINGS[@]}" \
    --rpc-url "$RPC_URL" \
    --private-key "$PRIVATE_KEY" \
    --broadcast 2>&1)
  DEPLOYED_ADDR=$(echo "$DEPLOY_OUTPUT" | sed -n 's/.*Deployed to: \(0x[0-9a-fA-F]*\).*/\1/p')
fi

echo ""
echo "Deployed at: $DEPLOYED_ADDR"

#
# Verify deployed address matches expected
#
if [ "$(echo "$DEPLOYED_ADDR" | tr '[:upper:]' '[:lower:]')" != "$(echo "$EXPECTED_CONTRACT_ADDR" | tr '[:upper:]' '[:lower:]')" ]; then
  echo "Error: Deployed address $DEPLOYED_ADDR does not match expected $EXPECTED_CONTRACT_ADDR" >&2
  exit 1
fi

echo ""
echo "Setting CoprocessorConfig..."
echo "  ACL:          $ACL_ADDRESS"
echo "  Coprocessor:  $COPROCESSOR_ADDRESS"
echo "  KMSVerifier:  $KMS_VERIFIER_ADDRESS"

#
# Call setCoprocessorConfig to enable use of FHE lib 
#
cast send "$DEPLOYED_ADDR" \
  "setCoprocessorConfig((address,address,address))" \
  "($ACL_ADDRESS,$COPROCESSOR_ADDRESS,$KMS_VERIFIER_ADDRESS)" \
  --rpc-url "$RPC_URL" \
  --private-key "$PRIVATE_KEY" > /dev/null

echo ""
echo "Calling getCoprocessorConfig()..."

#
# Call getCoprocessorConfig for verification
#
CONFIG=$(cast call "$DEPLOYED_ADDR" "getCoprocessorConfig()((address,address,address))" --rpc-url "$RPC_URL")
echo "$CONFIG"

#
# Verify returned addresses match expected
#
RETURNED_ACL=$(echo "$CONFIG" | sed -n 's/.*(\(0x[0-9a-fA-F]*\),.*/\1/p')
RETURNED_COPROCESSOR=$(echo "$CONFIG" | sed -n 's/.*, \s*\(0x[0-9a-fA-F]*\),.*/\1/p')
RETURNED_KMS=$(echo "$CONFIG" | sed -n 's/.*, \s*\(0x[0-9a-fA-F]*\))/\1/p')

check_addr() {
  local name="$1" got="$2" expected="$3"
  if [ "$(echo "$got" | tr '[:upper:]' '[:lower:]')" != "$(echo "$expected" | tr '[:upper:]' '[:lower:]')" ]; then
    echo "Error: $name mismatch: got $got, expected $expected" >&2
    exit 1
  fi
  echo "  ✅ $name: $got"
}

echo ""
echo "Verifying CoprocessorConfig..."
check_addr "ACL" "$RETURNED_ACL" "$ACL_ADDRESS"
check_addr "Coprocessor" "$RETURNED_COPROCESSOR" "$COPROCESSOR_ADDRESS"
check_addr "KMSVerifier" "$RETURNED_KMS" "$KMS_VERIFIER_ADDRESS"
echo ""
echo "All checks passed."

#
# Initialize random encrypted values if not already set
# For each type: if get<Type>() == 0x, call rand<Type>() + makePubliclyDecryptable<Type>()
#
ZERO="0x0000000000000000000000000000000000000000000000000000000000000000"

init_encrypted_value() {
  local type_name="$1"
  local get_fn="get${type_name}()(bytes32)"
  local rand_fn="rand${type_name}()"
  local decrypt_fn="makePubliclyDecryptable${type_name}()"

  local current
  current=$(cast call "$DEPLOYED_ADDR" "$get_fn" --rpc-url "$RPC_URL" --from "$DEPLOYER" 2>/dev/null || echo "$ZERO")

  if [ "$current" = "$ZERO" ]; then
    echo "  $type_name: not initialized, calling rand${type_name}()..."
    cast send "$DEPLOYED_ADDR" "$rand_fn" \
      --rpc-url "$RPC_URL" \
      --private-key "$PRIVATE_KEY" > /dev/null

    echo "  $type_name: calling makePubliclyDecryptable${type_name}()..."
    cast send "$DEPLOYED_ADDR" "$decrypt_fn" \
      --rpc-url "$RPC_URL" \
      --private-key "$PRIVATE_KEY" > /dev/null

    echo "  ✅ $type_name: initialized"
  else
    echo "  ✅ $type_name: already initialized ($current)"
  fi
}

echo ""
echo "Initializing encrypted values..."
init_encrypted_value "Ebool"
init_encrypted_value "Euint8"
init_encrypted_value "Euint16"
init_encrypted_value "Euint32"
init_encrypted_value "Euint64"
init_encrypted_value "Euint128"
init_encrypted_value "Euint256"
echo ""
echo "Done."

