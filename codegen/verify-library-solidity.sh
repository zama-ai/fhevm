#!/bin/bash
FHEVM_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

OUT_BASE_DIR=$(jq -r '.directories.baseDir' ./codegen.library-solidity.config.json)
SOL_REL_DIR=$(jq -r '.solidity.outDir' ./codegen.library-solidity.config.json)
TS_REL_DIR=$(jq -r '.typescript.outDir' ./codegen.library-solidity.config.json)

npm run build && ./codegen.mjs --overloads ./overloads/library-solidity.json --config ./codegen.library-solidity.config.json --debug 

SOL_DIR="${FHEVM_DIR}/library-solidity/examples/tests"
TS_DIR="${FHEVM_DIR}/library-solidity/test/fhevmOperations"

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
