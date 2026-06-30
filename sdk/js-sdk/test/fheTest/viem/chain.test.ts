import { getViemTestConfig } from '../setup-viem.js';
import { isCleartext } from '../setupCommon.js';
import { defineChainTests } from '../viem-common/chain.tests.js';

////////////////////////////////////////////////////////////////////////////////
//
// CHAIN=localstack     npx vitest run --config test/fheTest/vitest.config.ts viem/chain.test.ts
// CHAIN=testnet        npx vitest run --config test/fheTest/vitest.config.ts viem/chain.test.ts
// CHAIN=devnet         npx vitest run --config test/fheTest/vitest.config.ts viem/chain.test.ts
//
////////////////////////////////////////////////////////////////////////////////

defineChainTests(!isCleartext(getViemTestConfig().chainName));
