import { createFhevmBaseClient } from '@fhevm/sdk/ethers';
import { getEthersTestConfig } from '../setup-ethers.js';
import { isCleartext } from '../setupCommon.js';
import { defineClientBaseDecryptPublicValueTests } from '../ethers-common/clientBase.decryptPublicValue.tests.js';

////////////////////////////////////////////////////////////////////////////////
//
// CHAIN=localstack     npx vitest run --config test/fheTest/vitest.config.ts ethers/clientBase.decryptPublicValue.test.ts
// CHAIN=testnet        npx vitest run --config test/fheTest/vitest.config.ts ethers/clientBase.decryptPublicValue.test.ts
// CHAIN=devnet         npx vitest run --config test/fheTest/vitest.config.ts ethers/clientBase.decryptPublicValue.test.ts
//
////////////////////////////////////////////////////////////////////////////////

defineClientBaseDecryptPublicValueTests({
  runIf: !isCleartext(getEthersTestConfig().chainName),
  createFhevmBaseClient: (params) => createFhevmBaseClient(params),
});
