import { createFhevmEncryptClient, createFhevmDecryptClient } from '@fhevm/sdk/viem';
import { getViemTestConfig } from '../setup-viem.js';
import { isCleartext } from '../setupCommon.js';
import { defineClientEncryptDecryptSlowTests } from '../viem-common/clientEncrypt.encryptDecrypt.slow.tests.js';

defineClientEncryptDecryptSlowTests({
  runIf: !isCleartext(getViemTestConfig().chainName),
  createFhevmEncryptClient: (params) => createFhevmEncryptClient(params),
  createFhevmDecryptClient: (params) => createFhevmDecryptClient(params),
});
