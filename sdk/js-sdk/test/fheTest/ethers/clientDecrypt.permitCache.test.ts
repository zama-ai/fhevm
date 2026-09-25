import { createFhevmDecryptClient } from '@fhevm/sdk/ethers';
import { getEthersTestConfig } from '../setup-ethers.js';
import { isCleartext } from '../setupCommon.js';
import { defineClientDecryptPermitCacheTests } from '../ethers-common/clientDecrypt.permitCache.tests.js';

////////////////////////////////////////////////////////////////////////////////
//
// CHAIN=localstack npx vitest run --config test/fheTest/vitest.config.ts ethers/clientDecrypt.permitCache.test.ts
//
////////////////////////////////////////////////////////////////////////////////

defineClientDecryptPermitCacheTests({
  runIf: !isCleartext(getEthersTestConfig().chainName),
  createFhevmDecryptClient: (params) => createFhevmDecryptClient(params),
});
