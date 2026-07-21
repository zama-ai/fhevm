import { createFhevmBaseClient } from '@fhevm/sdk/viem';
import { getViemTestConfig } from '../setup-viem.js';
import { isCleartext } from '../setupCommon.js';
import { defineClientBaseDecryptPublicValueTests } from '../viem-common/clientBase.decryptPublicValue.tests.js';

////////////////////////////////////////////////////////////////////////////////
//
// CHAIN=localstack     npx vitest run --config test/fheTest/vitest.config.ts viem/clientBase.decryptPublicValue.test.ts
// CHAIN=testnet        npx vitest run --config test/fheTest/vitest.config.ts viem/clientBase.decryptPublicValue.test.ts
// CHAIN=devnet         npx vitest run --config test/fheTest/vitest.config.ts viem/clientBase.decryptPublicValue.test.ts
//
////////////////////////////////////////////////////////////////////////////////

defineClientBaseDecryptPublicValueTests({
  runIf: !isCleartext(getViemTestConfig().chainName),
  createFhevmBaseClient: (params) => createFhevmBaseClient(params),
});
