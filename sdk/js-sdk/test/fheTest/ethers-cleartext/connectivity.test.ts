import { getEthersTestConfig } from '../setup-ethers.js';
import { isCleartext } from '../setupCommon.js';
import { defineConnectivityTests } from '../ethers-common/connectivity.tests.js';

////////////////////////////////////////////////////////////////////////////////
//
// CHAIN=localcleartext npx vitest run --config test/fheTest/vitest.config.ts ethers-cleartext/connectivity.test.ts
//
////////////////////////////////////////////////////////////////////////////////

defineConnectivityTests(isCleartext(getEthersTestConfig().chainName));
