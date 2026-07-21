import { createFhevmDecryptClient } from '@fhevm/sdk/viem';
import { getViemTestConfig } from '../setup-viem.js';
import { isCleartext } from '../setupCommon.js';
import { defineClientDecryptDecryptTests } from '../viem-common/clientDecrypt.decrypt.tests.js';

////////////////////////////////////////////////////////////////////////////////
//
// CHAIN=localstack_v13 npx vitest run --config test/fheTest/vitest.config.ts viem/clientDecrypt.decrypt.test.ts
// CHAIN=localstack     npx vitest run --config test/fheTest/vitest.config.ts viem/clientDecrypt.decrypt.test.ts
// CHAIN=testnet        npx vitest run --config test/fheTest/vitest.config.ts viem/clientDecrypt.decrypt.test.ts
// CHAIN=devnet         npx vitest run --config test/fheTest/vitest.config.ts viem/clientDecrypt.decrypt.test.ts
//
////////////////////////////////////////////////////////////////////////////////

defineClientDecryptDecryptTests({
  runIf: !isCleartext(getViemTestConfig().chainName),
  createFhevmDecryptClient: (params) => createFhevmDecryptClient(params),
});
