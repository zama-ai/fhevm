#!/bin/bash
FHEVM_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

OUT_BASE_DIR=$(jq -r '.directories.baseDir' ./codegen.host-contracts.config.json)
SOL_REL_DIR=$(jq -r '.solidity.outDir' ./codegen.host-contracts.config.json)
TS_REL_DIR=$(jq -r '.typescript.outDir' ./codegen.host-contracts.config.json)

npm run build && ./codegen.mjs --overloads ./overloads/host-contracts.json --config ./codegen.host-contracts.config.json --verbose

LIB_DIR="${FHEVM_DIR}/host-contracts/lib-local.orig"
SOL_DIR="${FHEVM_DIR}/host-contracts/examples/tests-local.orig"
TS_DIR="${FHEVM_DIR}/host-contracts/test/fhevmOperations-local.orig"

OUT_SOL_DIR="${OUT_BASE_DIR}/${SOL_REL_DIR}"
OUT_TS_DIR="${OUT_BASE_DIR}/${TS_REL_DIR}"

for i in {1..7}; do
    FILE_A=${OUT_SOL_DIR}/FHEVMTestSuite${i}.sol
    FILE_B=${SOL_DIR}/FHEVMTestSuite${i}.sol
    diff "${FILE_A}" "${FILE_B}" 
    if [ $? -eq 0 ]; then
        echo "✅ Files are identical: '${FILE_A}' and '${FILE_B}'"
    fi
done

for i in {1..13}; do
    FILE_A=${OUT_TS_DIR}/fhevmOperations${i}.ts
    FILE_B=${TS_DIR}/fhevmOperations${i}.ts
    diff "${FILE_A}" "${FILE_B}" 
    if [ $? -eq 0 ]; then
        echo "✅ Files are identical: '${FILE_A}' and '${FILE_B}'"
    fi
done

# Only host-contracts
# HCULimit.sol
FILE_A="${FHEVM_DIR}/host-contracts/contracts/HCULimit-local.sol.orig"
FILE_B="${OUT_BASE_DIR}/contracts/HCULimit.sol"
diff "${FILE_A}" "${FILE_B}" 
if [ $? -eq 0 ]; then
    echo "✅ Files are identical: '${FILE_A}' and '${FILE_B}'"
fi

# Check FHE.sol
FILE_A="${LIB_DIR}/FHE.sol"
FILE_B="${OUT_BASE_DIR}/lib/FHE.sol"
diff "${FILE_A}" "${FILE_B}" 
if [ $? -eq 0 ]; then
    echo "✅ Files are identical: '${FILE_A}' and '${FILE_B}'"
fi

# Check Impl.sol
FILE_A="${LIB_DIR}/Impl.sol"
FILE_B="${OUT_BASE_DIR}/lib/Impl.sol"
diff "${FILE_A}" "${FILE_B}" 
if [ $? -eq 0 ]; then
    echo "✅ Files are identical: '${FILE_A}' and '${FILE_B}'"
fi

# Check FheType.sol
FILE_A="${FHEVM_DIR}/host-contracts/contracts/shared/FheType-local.sol.orig"
FILE_B="${OUT_BASE_DIR}/contracts/shared/FheType.sol"
diff "${FILE_A}" "${FILE_B}" 
if [ $? -eq 0 ]; then
    echo "✅ Files are identical: '${FILE_A}' and '${FILE_B}'"
fi
