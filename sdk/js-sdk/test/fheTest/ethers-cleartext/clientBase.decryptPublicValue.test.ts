import { createFhevmCleartextBaseClient } from '@fhevm/sdk/ethers/cleartext';
import { getEthersTestConfig } from '../setup-ethers.js';
import { isCleartext } from '../setupCommon.js';
import { defineClientBaseDecryptPublicValueTests } from '../ethers-common/clientBase.decryptPublicValue.tests.js';

////////////////////////////////////////////////////////////////////////////////
//
// CHAIN=localcleartext npx vitest run --config test/fheTest/vitest.config.ts ethers-cleartext/clientBase.decryptPublicValue.test.ts
//
////////////////////////////////////////////////////////////////////////////////

defineClientBaseDecryptPublicValueTests({
  runIf: isCleartext(getEthersTestConfig().chainName),
  createFhevmBaseClient: (params) => createFhevmCleartextBaseClient(params),
});
