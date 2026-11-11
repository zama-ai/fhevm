/** `npm run build` step: compiles `contracts/` into `dist/contracts/` (see contracts.ts). */
import { CONTRACTS_ARTIFACTS_DIR, CONTRACTS_SOURCES_DIR, PACKAGE_ROOT } from '../infrastructure/paths.js';
import { buildContracts } from './contracts.js';

try {
  const result = buildContracts(CONTRACTS_SOURCES_DIR, CONTRACTS_ARTIFACTS_DIR, PACKAGE_ROOT);
  for (const warning of result.warnings) console.warn(warning);
  console.log(`Compiled ${result.contracts.join(', ')} with solc ${result.compiler}`);
} catch (error) {
  console.error(`contracts build failed: ${error instanceof Error ? error.message : String(error)}`);
  process.exitCode = 1;
}
