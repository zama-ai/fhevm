#!/usr/bin/env bash
set -euo pipefail

export PATH="/Users/alex/.foundry/bin:$PATH"

# Usage: ./read-destroyed-context.sh <KMS_VERIFIER_ADDRESS> [RPC_URL]
#
# Fetches the current KMS context ID via getCurrentKmsContextId(), then reads
# KMSVerifier.destroyedContexts[contextId] from ERC-7201 namespaced storage.

KMS_VERIFIER_ADDRESS="0x3F3819BeBE4bD0EFEf8078Df6f9B574ADa80CCA4"
RPC_URL='https://ethereum-sepolia-rpc.publicnode.com'

# KMS_VERIFIER_ADDRESS="${1:?Usage: $0 <KMS_VERIFIER_ADDRESS> [RPC_URL]}"
# RPC_URL="${2:-http://localhost:8545}"

# Fetch current context ID on-chain
CONTEXT_ID=$(cast call "$KMS_VERIFIER_ADDRESS" "getCurrentKmsContextId()(uint256)" --rpc-url "$RPC_URL" | awk '{print $1}')
echo "Current KMS Context ID: $CONTEXT_ID"

# Fetch current threshold on-chain
THRESHOLD=$(cast call "$KMS_VERIFIER_ADDRESS" "getThreshold()(uint256)" --rpc-url "$RPC_URL" | awk '{print $1}')
echo "Current KMS Threshold: $THRESHOLD"

# Low-level storage read of currentKmsContextId (4th field, offset 3)
STRUCT_BASE=0x7e81a744be86773af8644dd7304fa1dc9350ccabf16cfcaa614ddb78b4ce8900
CONTEXT_ID_SLOT=$(python3 -c "print(hex($STRUCT_BASE + 3))")
CONTEXT_ID_RAW=$(cast storage "$KMS_VERIFIER_ADDRESS" "$CONTEXT_ID_SLOT" --rpc-url "$RPC_URL")
CONTEXT_ID_UINT256=$(cast to-dec "$CONTEXT_ID_RAW")
echo "currentKmsContextId (raw storage): $CONTEXT_ID_UINT256"

# Low-level storage read of currentThreshold (4th field, offset 2)
THRESHOLD_SLOT=$(python3 -c "print(hex($STRUCT_BASE + 2))")
THRESHOLD_RAW=$(cast storage "$KMS_VERIFIER_ADDRESS" "$THRESHOLD_SLOT" --rpc-url "$RPC_URL")
THRESHOLD_UINT256=$(cast to-dec "$THRESHOLD_RAW")
echo "currentThreshold (raw storage): $THRESHOLD_UINT256"

# Low-level storage read of contextThreshold[contextId] (7th field, offset 6)
CTX_THRESHOLD_ROOT=$(python3 -c "print(hex($STRUCT_BASE + 6))")
CTX_THRESHOLD_SLOT=$(cast keccak "$(cast abi-encode "f(uint256,uint256)" "$CONTEXT_ID" "$CTX_THRESHOLD_ROOT")")
CTX_THRESHOLD_RAW=$(cast storage "$KMS_VERIFIER_ADDRESS" "$CTX_THRESHOLD_SLOT" --rpc-url "$RPC_URL")
CTX_THRESHOLD_UINT256=$(cast to-dec "$CTX_THRESHOLD_RAW")
echo "contextThreshold[$CONTEXT_ID] (raw storage): $CTX_THRESHOLD_UINT256"

# # ERC-7201 struct base slot
# STRUCT_BASE=0x7e81a744be86773af8644dd7304fa1dc9350ccabf16cfcaa614ddb78b4ce8900

# # destroyedContexts is the 8th field (offset 7) in KMSVerifierStorage
# MAPPING_ROOT=$(python3 -c "print(hex($STRUCT_BASE + 7))")

# # mapping slot = keccak256(abi.encode(contextId, mappingRoot))
# SLOT=$(cast keccak "$(cast abi-encode "f(uint256,uint256)" "$CONTEXT_ID" "$MAPPING_ROOT")")

# echo "KMSVerifier:  $KMS_VERIFIER_ADDRESS"
# echo "Context ID:   $CONTEXT_ID"
# echo "Storage slot: $SLOT"

# RESULT=$(cast storage "$KMS_VERIFIER_ADDRESS" "$SLOT" --rpc-url "$RPC_URL")

# echo "Value:        $RESULT"

# if [ "$RESULT" = "0x0000000000000000000000000000000000000000000000000000000000000001" ]; then
#   echo "Status:       DESTROYED"
# else
#   echo "Status:       ACTIVE"
# fi
