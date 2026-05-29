import { createFhevmCleartextEncryptClient } from '@fhevm/sdk/viem/cleartext';
import { getViemTestConfig } from '../setup-viem.js';
import { isCleartext } from '../setupCommon.js';
import { defineClientEncryptEncryptTests } from '../viem-common/clientEncrypt.encrypt.tests.js';

defineClientEncryptEncryptTests(isCleartext(getViemTestConfig().chainName), {
  createClient: (params) => createFhevmCleartextEncryptClient(params),
});
