import { createFhevmCleartextEncryptClient } from '@fhevm/sdk/ethers/cleartext';
import { getEthersTestConfig } from '../setup-ethers.js';
import { isCleartext } from '../setupCommon.js';
import { defineClientEncryptEncryptTests } from '../ethers-common/clientEncrypt.encrypt.tests.js';

////////////////////////////////////////////////////////////////////////////////
//
// CHAIN=localcleartext npx vitest run --config test/fheTest/vitest.config.ts ethers-cleartext/clientEncrypt.encrypt.test.ts
//
////////////////////////////////////////////////////////////////////////////////

defineClientEncryptEncryptTests({
  runIf: isCleartext(getEthersTestConfig().chainName),
  createFhevmEncryptClient: (params) => createFhevmCleartextEncryptClient(params),
});
