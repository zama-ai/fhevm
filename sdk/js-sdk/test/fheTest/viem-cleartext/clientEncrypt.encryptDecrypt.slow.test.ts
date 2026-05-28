import { createFhevmCleartextEncryptClient, createFhevmCleartextDecryptClient } from '@fhevm/sdk/viem/cleartext';
import { getViemTestConfig } from '../setup-viem.js';
import { isCleartext } from '../setupCommon.js';
import { defineClientEncryptDecryptSlowTests } from '../viem-common/clientEncrypt.encryptDecrypt.slow.tests.js';

defineClientEncryptDecryptSlowTests(isCleartext(getViemTestConfig().chainName), {
  createEncryptClient: (params) => createFhevmCleartextEncryptClient(params),
  createDecryptClient: (params) => createFhevmCleartextDecryptClient(params),
});
