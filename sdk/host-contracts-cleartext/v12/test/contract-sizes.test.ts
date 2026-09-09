import assert from 'node:assert/strict';
import test from 'node:test';

import {
  assessContractSizes,
  EIP_170_RUNTIME_SIZE_LIMIT,
  type DeployableContractSize,
} from '../internal/checkContractSizes.ts';

function contract(sourcePath: string, runtimeSize: number): DeployableContractSize {
  return { sourcePath, contractName: 'TestContract', runtimeSize };
}

void test('rejects an EIP-170 overflow outside the two Forge-only exception files', () => {
  const report = assessContractSizes([contract('pkg/src/contracts/Deployable.sol', EIP_170_RUNTIME_SIZE_LIMIT + 1)]);

  assert.equal(report.violations.length, 1);
  assert.equal(report.allowedOverflows.length, 0);
});

void test('allows only the two declared Forge-only source files to exceed EIP-170', () => {
  const report = assessContractSizes([
    contract('pkg/src/cleartext/CleartextForgeArithmetic.sol', EIP_170_RUNTIME_SIZE_LIMIT + 1),
    contract('pkg/src/cleartext/CleartextForgeFHEVMExecutor.sol', EIP_170_RUNTIME_SIZE_LIMIT + 1),
    contract('pkg/src/cleartext/CleartextFHEVMExecutor.sol', EIP_170_RUNTIME_SIZE_LIMIT),
  ]);

  assert.equal(report.violations.length, 0);
  assert.equal(report.allowedOverflows.length, 2);
});
