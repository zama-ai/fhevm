import { getViemTestConfig } from '../setup-viem.js';
import { isCleartext } from '../setupCommon.js';
import { defineChainTests } from '../viem-common/chain.tests.js';

////////////////////////////////////////////////////////////////////////////////
//
// CHAIN=localcleartext npx vitest run --config test/fheTest/vitest.config.ts viem-cleartext/chain.test.ts
//
////////////////////////////////////////////////////////////////////////////////

defineChainTests(isCleartext(getViemTestConfig().chainName));
