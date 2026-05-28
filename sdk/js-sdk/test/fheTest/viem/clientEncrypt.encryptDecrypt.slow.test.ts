import { createFhevmEncryptClient, createFhevmDecryptClient } from '@fhevm/sdk/viem';
import { getViemTestConfig } from '../setup-viem.js';
import { isCleartext } from '../setupCommon.js';
import { defineClientEncryptDecryptSlowTests } from '../viem-common/clientEncrypt.encryptDecrypt.slow.tests.js';

defineClientEncryptDecryptSlowTests(!isCleartext(getViemTestConfig().chainName), {
  createEncryptClient: (params) => createFhevmEncryptClient(params),
  createDecryptClient: (params) => createFhevmDecryptClient(params),
});
