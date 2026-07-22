import { createFhevmBaseClient } from '@fhevm/sdk/ethers';
import { getEthersTestConfig } from '../setup-ethers.js';
import { isCleartext } from '../setupCommon.js';
import { defineClientBaseTests } from '../ethers-common/clientBase.tests.js';

////////////////////////////////////////////////////////////////////////////////
//
// CHAIN=localstack     npx vitest run --config test/fheTest/vitest.config.ts ethers/clientBase.test.ts
// CHAIN=testnet        npx vitest run --config test/fheTest/vitest.config.ts ethers/clientBase.test.ts
// CHAIN=devnet         npx vitest run --config test/fheTest/vitest.config.ts ethers/clientBase.test.ts
//
////////////////////////////////////////////////////////////////////////////////

defineClientBaseTests({
  runIf: !isCleartext(getEthersTestConfig().chainName),
  createFhevmBaseClient: (params) => createFhevmBaseClient(params),
  keyMode: 'fhe',
});
