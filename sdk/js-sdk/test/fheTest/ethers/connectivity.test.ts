import { getEthersTestConfig } from '../setup-ethers.js';
import { isCleartext } from '../setupCommon.js';
import { defineConnectivityTests } from '../ethers-common/connectivity.tests.js';

////////////////////////////////////////////////////////////////////////////////
//
// CHAIN=localstack     npx vitest run --config test/fheTest/vitest.config.ts ethers/connectivity.test.ts
// CHAIN=testnet        npx vitest run --config test/fheTest/vitest.config.ts ethers/connectivity.test.ts
// CHAIN=devnet         npx vitest run --config test/fheTest/vitest.config.ts ethers/connectivity.test.ts
//
////////////////////////////////////////////////////////////////////////////////

defineConnectivityTests(!isCleartext(getEthersTestConfig().chainName));
