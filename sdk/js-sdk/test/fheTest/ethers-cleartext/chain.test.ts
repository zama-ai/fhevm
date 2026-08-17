import { getEthersTestConfig } from '../setup-ethers.js';
import { isCleartext } from '../setupCommon.js';
import { defineChainTests } from '../ethers-common/chain.tests.js';

////////////////////////////////////////////////////////////////////////////////
//
// CHAIN=localcleartext npx vitest run --config test/fheTest/vitest.config.ts ethers-cleartext/chain.test.ts
//
////////////////////////////////////////////////////////////////////////////////

defineChainTests(isCleartext(getEthersTestConfig().chainName));
