import { getViemTestConfig } from '../setup-viem.js';
import { isCleartext } from '../setupCommon.js';
import { defineConnectivityTests } from '../viem-common/connectivity.tests.js';

////////////////////////////////////////////////////////////////////////////////
//
// CHAIN=localstack     npx vitest run --config test/fheTest/vitest.config.ts viem/connectivity.test.ts
// CHAIN=testnet        npx vitest run --config test/fheTest/vitest.config.ts viem/connectivity.test.ts
// CHAIN=devnet         npx vitest run --config test/fheTest/vitest.config.ts viem/connectivity.test.ts
//
////////////////////////////////////////////////////////////////////////////////

defineConnectivityTests(!isCleartext(getViemTestConfig().chainName));
