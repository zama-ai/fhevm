// Run: npm run generate:cleartext-config   (copy)
//      npm run check:cleartext-config      (verify, no writes)
//
// The copy runs as an early step of `make generate` — so a normal
// install refreshes the payload copy. The check runs inside `npm run build`, which is where a stale or
// hand-edited copy has to be caught: `build` compiles pkg/ts, so from that point on the copy is what ships.

import {
  checkCleartextConfig,
  copyCleartextConfig,
  packageRelative,
  CLEARTEXT_CONFIG_PAYLOAD_PATH,
} from '../copyCleartextConfig.ts';

const CHECK_ONLY = process.argv.includes('--check');
const payload = packageRelative(CLEARTEXT_CONFIG_PAYLOAD_PATH);

if (CHECK_ONLY) {
  const { status } = checkCleartextConfig();
  if (status === 'identical') {
    console.log(`   ✅ ${payload} is byte-identical to the source of truth`);
  } else {
    const detail =
      status === 'missing'
        ? 'it does not exist'
        : 'it differs from internal/cleartext-config.ts — the payload copy is generated, not editable';
    console.error(`   ❌ ${payload}: ${detail}.`);
    console.error('      Edit internal/cleartext-config.ts and run `npm run generate:cleartext-config`.');
    process.exit(1);
  }
} else {
  copyCleartextConfig();
  console.log(`   wrote ${payload} from internal/cleartext-config.ts`);
}
