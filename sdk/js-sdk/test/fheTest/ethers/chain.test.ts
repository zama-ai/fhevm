import { getEthersTestConfig } from '../setup-ethers.js';
import { isCleartext } from '../setupCommon.js';
import { defineChainTests } from '../ethers-common/chain.tests.js';

////////////////////////////////////////////////////////////////////////////////
//
// CHAIN=localstack     npx vitest run --config test/fheTest/vitest.config.ts ethers/chain.test.ts
// CHAIN=testnet        npx vitest run --config test/fheTest/vitest.config.ts ethers/chain.test.ts
// CHAIN=devnet         npx vitest run --config test/fheTest/vitest.config.ts ethers/chain.test.ts
//
////////////////////////////////////////////////////////////////////////////////

defineChainTests(!isCleartext(getEthersTestConfig().chainName));
