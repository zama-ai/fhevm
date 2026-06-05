import { createFhevmBaseClient } from '@fhevm/sdk/ethers';
import { getEthersTestConfig } from '../setup-ethers.js';
import { isCleartext } from '../setupCommon.js';
import { defineClientBaseChainTests } from '../ethers-common/clientBase.chain.tests.js';

////////////////////////////////////////////////////////////////////////////////
//
// CHAIN=localstack     npx vitest run --config test/fheTest/vitest.config.ts ethers/clientBase.chain.test.ts
// CHAIN=testnet        npx vitest run --config test/fheTest/vitest.config.ts ethers/clientBase.chain.test.ts
// CHAIN=devnet         npx vitest run --config test/fheTest/vitest.config.ts ethers/clientBase.chain.test.ts
//
////////////////////////////////////////////////////////////////////////////////

defineClientBaseChainTests({
  runIf: !isCleartext(getEthersTestConfig().chainName),
  createFhevmBaseClient: (params) => createFhevmBaseClient(params),
});
