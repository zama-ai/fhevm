import { createFhevmDecryptClient } from '@fhevm/sdk/viem';
import { getViemTestConfig } from '../setup-viem.js';
import { isCleartext } from '../setupCommon.js';
import { defineClientDecryptPermitCacheTests } from '../viem-common/clientDecrypt.permitCache.tests.js';

////////////////////////////////////////////////////////////////////////////////
//
// CHAIN=localstack_v11 npx vitest run --config test/fheTest/vitest.config.ts viem/clientDecrypt.permitCache.test.ts
// CHAIN=localstack_v12 npx vitest run --config test/fheTest/vitest.config.ts viem/clientDecrypt.permitCache.test.ts
// CHAIN=localstack     npx vitest run --config test/fheTest/vitest.config.ts viem/clientDecrypt.permitCache.test.ts
//
////////////////////////////////////////////////////////////////////////////////

defineClientDecryptPermitCacheTests({
  runIf: !isCleartext(getViemTestConfig().chainName),
  createFhevmDecryptClient: (params) => createFhevmDecryptClient(params),
});
