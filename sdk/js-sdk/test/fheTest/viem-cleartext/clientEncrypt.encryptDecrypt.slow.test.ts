import { createFhevmCleartextEncryptClient, createFhevmCleartextDecryptClient } from '@fhevm/sdk/viem/cleartext';
import { getViemTestConfig } from '../setup-viem.js';
import { isCleartext } from '../setupCommon.js';
import { defineClientEncryptDecryptSlowTests } from '../viem-common/clientEncrypt.encryptDecrypt.slow.tests.js';

////////////////////////////////////////////////////////////////////////////////
//
// CHAIN=localcleartext npx vitest run --config test/fheTest/vitest.config.ts viem-cleartext/clientEncrypt.encryptDecrypt.slow.test.ts
//
////////////////////////////////////////////////////////////////////////////////

defineClientEncryptDecryptSlowTests({
  runIf: isCleartext(getViemTestConfig().chainName),
  createFhevmEncryptClient: (params) => createFhevmCleartextEncryptClient(params),
  createFhevmDecryptClient: (params) => createFhevmCleartextDecryptClient(params),
});
