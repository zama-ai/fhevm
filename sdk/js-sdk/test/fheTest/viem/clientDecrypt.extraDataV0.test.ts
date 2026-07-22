import { createFhevmDecryptClient } from '@fhevm/sdk/viem';
import { getViemTestConfig } from '../setup-viem.js';
import { isCleartext } from '../setupCommon.js';
import { defineClientDecryptExtraDataV0Tests } from '../viem-common/clientDecrypt.extraDataV0.tests.js';

////////////////////////////////////////////////////////////////////////////////
//
// CHAIN=testnet npx vitest run --config test/fheTest/vitest.config.ts viem/clientDecrypt.extraDataV0.test.ts
// CHAIN=devnet  npx vitest run --config test/fheTest/vitest.config.ts viem/clientDecrypt.extraDataV0.test.ts
//
////////////////////////////////////////////////////////////////////////////////

defineClientDecryptExtraDataV0Tests({
  runIf: !isCleartext(getViemTestConfig().chainName),
  createFhevmDecryptClient: (params) => createFhevmDecryptClient(params),
});
