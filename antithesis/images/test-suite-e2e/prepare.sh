#!/usr/bin/env bash
# Container command: what preview-env's e2e run-test step does before each test
# (ci/preview-env/test-suite/values-test-suite-workflow-e2e.yaml), done once, then idle so
# Test Composer can run the test commands. /tmp/ready gates the readiness probe.
set -euo pipefail
printf '%s\n' "${KMS_CONNECTOR_PROXY_CA}" >"${NODE_EXTRA_CA_CERTS}"
config=/app/test-suite/e2e/contracts/E2ECoprocessorConfigLocal.sol
sed -i "s|ACLAddress: 0x[0-9a-fA-F]*,|ACLAddress: ${ACL_CONTRACT_ADDRESS},|g" "${config}"
sed -i "s|CoprocessorAddress: 0x[0-9a-fA-F]*,|CoprocessorAddress: ${FHEVM_EXECUTOR_CONTRACT_ADDRESS},|g" "${config}"
sed -i "s|KMSVerifierAddress: 0x[0-9a-fA-F]*|KMSVerifierAddress: ${KMS_VERIFIER_CONTRACT_ADDRESS}|g" "${config}"
cd /app/test-suite/e2e
npx hardhat clean
npx hardhat compile
touch /tmp/ready
exec sleep infinity
