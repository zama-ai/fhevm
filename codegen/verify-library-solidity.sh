#!/bin/bash
FHEVM_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

OUT_BASE_DIR=$(jq -r '.directories.baseDir' ./codegen.library-solidity.config.json)
SOL_REL_DIR=$(jq -r '.solidity.outDir' ./codegen.library-solidity.config.json)
TS_REL_DIR=$(jq -r '.typescript.outDir' ./codegen.library-solidity.config.json)

npm run build && ./codegen.mjs --overloads ./overloads/library-solidity.json --config ./codegen.library-solidity.config.json --verbose 

LIB_DIR="${FHEVM_DIR}/library-solidity/lib-local.orig"
SOL_DIR="${FHEVM_DIR}/library-solidity/examples/tests-local.orig"
TS_DIR="${FHEVM_DIR}/library-solidity/test/fhevmOperations-local.orig"

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
FILE_A="${LIB_DIR}/FheType.sol"
FILE_B="${OUT_BASE_DIR}/lib/FheType.sol"
diff "${FILE_A}" "${FILE_B}" 
if [ $? -eq 0 ]; then
    echo "✅ Files are identical: '${FILE_A}' and '${FILE_B}'"
fi
