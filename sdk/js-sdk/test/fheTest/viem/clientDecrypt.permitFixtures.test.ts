import { createFhevmDecryptClient } from '@fhevm/sdk/viem';
import { getViemTestConfig } from '../setup-viem.js';
import { isCleartext } from '../setupCommon.js';
import { defineClientDecryptPermitFixturesTests } from '../viem-common/clientDecrypt.permitFixtures.tests.js';

////////////////////////////////////////////////////////////////////////////////
//
// Verify committed fixtures:
//   CHAIN=localstack_v12 npx vitest run --config test/fheTest/vitest.config.ts viem/clientDecrypt.permitFixtures.test.ts
//
// Generate a missing fixture (requires the chain's stack running):
//   GENERATE_PERMIT_FIXTURES=1 CHAIN=localstack_v12 npx vitest run --config test/fheTest/vitest.config.ts viem/clientDecrypt.permitFixtures.test.ts
//
// Parsing is core-SDK logic shared by both adapters, so this suite runs on
// viem only.
//
////////////////////////////////////////////////////////////////////////////////

defineClientDecryptPermitFixturesTests({
  runIf: !isCleartext(getViemTestConfig().chainName),
  createFhevmDecryptClient: (params) => createFhevmDecryptClient(params),
});
