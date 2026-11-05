import { createFhevmCleartextEncryptClient, createFhevmCleartextDecryptClient } from '@fhevm/sdk/ethers/cleartext';
import { getEthersTestConfig } from '../setup-ethers.js';
import { isCleartext } from '../setupCommon.js';
import { defineClientEncryptDecryptSlowTests } from '../ethers-common/clientEncrypt.encryptDecrypt.slow.tests.js';

////////////////////////////////////////////////////////////////////////////////
//
// CHAIN=localcleartext_v1_0 npx vitest run --config test/fheTest/vitest.config.ts ethers-cleartext/clientEncrypt.encryptDecrypt.slow.test.ts
//
////////////////////////////////////////////////////////////////////////////////

defineClientEncryptDecryptSlowTests({
  runIf: isCleartext(getEthersTestConfig().chainName),
  createFhevmEncryptClient: (params) => createFhevmCleartextEncryptClient(params),
  createFhevmDecryptClient: (params) => createFhevmCleartextDecryptClient(params),
});
