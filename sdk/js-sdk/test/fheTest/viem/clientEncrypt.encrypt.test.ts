import { createFhevmEncryptClient } from '@fhevm/sdk/viem';
import { getViemTestConfig } from '../setup-viem.js';
import { isCleartext } from '../setupCommon.js';
import { defineClientEncryptEncryptTests } from '../viem-common/clientEncrypt.encrypt.tests.js';

defineClientEncryptEncryptTests({
  runIf: !isCleartext(getViemTestConfig().chainName),
  createFhevmEncryptClient: (params) => createFhevmEncryptClient(params),
});
