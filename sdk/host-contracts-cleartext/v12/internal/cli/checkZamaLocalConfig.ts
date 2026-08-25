// Run: npm run check:zama-config
//
// RULES.md rules 15 and 17 gate: ZAMA_LOCAL_CONFIG in internal/constants.ts must equal the addresses
// `library-solidity/config/ZamaConfig.sol` returns from `_getLocalConfig()`. See the module for why the
// existing derivation assertions do not already cover this.
//
// Wired into `npm run build` (and therefore `npm run test`, which builds). Cheap and read-only, so it
// runs before anything that compiles or deploys.

import { checkZamaLocalConfig } from '../checkZamaLocalConfig.ts';

try {
  const { label, entries, mismatches } = checkZamaLocalConfig();

  console.log('🔎 rules 15 and 17: ZAMA_LOCAL_CONFIG must match ZamaConfig.sol _getLocalConfig()');
  console.log(`   ${label}`);

  for (const entry of entries) {
    const names = `${entry.zamaField.padEnd(20)} ${entry.ourField.padEnd(22)}`;
    if (entry.matches) {
      console.log(`   ✅ ${names} ${entry.declared}`);
    } else {
      // Both values, labelled by which side said what — a single address is unreadable without knowing
      // which one is the one that is not ours to change.
      console.log(`   ❌ ${names}`);
      console.log(`      ZamaConfig.sol says  ${entry.declared}`);
      console.log(`      constants.ts says    ${entry.ours}`);
    }
  }

  if (mismatches.length > 0) {
    console.error('');
    console.error(
      `${String(mismatches.length)} of ${String(entries.length)} localhost addresses drifted from ZamaConfig.sol.`,
    );
    console.error('ZamaConfig.sol is the source of truth, not this package: those literals are compiled into');
    console.error('every dApp inheriting its localhost config, so they are not ours to choose. Update');
    console.error('ZAMA_LOCAL_CONFIG in internal/constants.ts to match, then re-derive the stack with');
    console.error('`npm run build:templates` — a moved address means the deploy order or the deployer no');
    console.error('longer produces the right set, which generateLocalHostBytecode.ts will tell you next.');
    process.exit(1);
  }

  console.log(`   ✅ ${String(entries.length)} localhost addresses match _getLocalConfig()`);
} catch (error) {
  console.error(`   ❌ ${error instanceof Error ? error.message : String(error)}`);
  process.exit(1);
}
