// Run after Forge compilation: npm run check:contract-sizes
//
// EIP-170 limits deployed runtime bytecode to 24,576 bytes. Forge-only implementations are explicit
// exceptions because they are installed directly into the local test VM rather than deployed on-chain.

import { checkContractSizes, CONTRACT_SIZE_EXCEPTIONS, EIP_170_RUNTIME_SIZE_LIMIT } from '../checkContractSizes.ts';

try {
  const report = checkContractSizes();
  console.log(`🔎 EIP-170 deployed runtime limit: ${EIP_170_RUNTIME_SIZE_LIMIT.toLocaleString('en-US')} B`);

  for (const contract of report.contracts) {
    const overflow = contract.runtimeSize > EIP_170_RUNTIME_SIZE_LIMIT;
    const allowed = overflow && CONTRACT_SIZE_EXCEPTIONS.has(contract.sourcePath);
    const status = allowed ? '⚠️' : overflow ? '❌' : '✅';
    const exception = allowed ? ' (allowed Forge-only exception)' : '';
    console.log(
      `${status} ${contract.sourcePath}:${contract.contractName} — ${contract.runtimeSize.toLocaleString('en-US')} B${exception}`,
    );
  }

  if (report.violations.length > 0) {
    throw new Error(
      `${String(report.violations.length)} deployable contract(s) exceed the EIP-170 limit without an exception`,
    );
  }

  console.log(
    `✅ ${String(report.contracts.length)} deployable contract(s) checked; ${String(report.allowedOverflows.length)} allowed overflow(s)`,
  );
} catch (error) {
  console.error(`❌ ${error instanceof Error ? error.message : String(error)}`);
  process.exit(1);
}
