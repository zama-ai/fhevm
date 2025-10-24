#!/bin/bash
FHEVM_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"

if [ ! -d "${FHEVM_DIR}/.github" ]; then
  echo "Error: invalid FHEVM repo root directory." >&2
  exit 1
fi

OUT_BASE_DIR=$(jq -r '.directories.baseDir' ./codegen.e2e.config.json)
SOL_REL_DIR=$(jq -r '.solidity.outDir' ./codegen.e2e.config.json)
TS_REL_DIR=$(jq -r '.typescript.outDir' ./codegen.e2e.config.json)

npm run build && ./codegen.mjs lib --overloads ./overloads/e2e.json --config ./codegen.e2e.config.json --verbose 

SOL_DIR="${FHEVM_DIR}/test-suite/e2e/${SOL_REL_DIR}-local.orig"
TS_DIR="${FHEVM_DIR}/test-suite/e2e/${TS_REL_DIR}-local.orig"

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

for i in {1..105}; do
    FILE_A=${OUT_TS_DIR}/fhevmOperations${i}.ts
    FILE_B=${TS_DIR}/fhevmOperations${i}.ts
    diff "${FILE_A}" "${FILE_B}" 
    if [ $? -eq 0 ]; then
        echo "✅ Files are identical: '${FILE_A}' and '${FILE_B}'"
    fi
done

