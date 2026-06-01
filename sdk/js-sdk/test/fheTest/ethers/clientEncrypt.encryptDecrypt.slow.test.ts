import { createFhevmEncryptClient, createFhevmDecryptClient } from '@fhevm/sdk/ethers';
import { getEthersTestConfig } from '../setup-ethers.js';
import { isCleartext } from '../setupCommon.js';
import { defineClientEncryptDecryptSlowTests } from '../ethers-common/clientEncrypt.encryptDecrypt.slow.tests.js';

defineClientEncryptDecryptSlowTests({
  runIf: !isCleartext(getEthersTestConfig().chainName),
  createFhevmEncryptClient: (params) => createFhevmEncryptClient(params),
  createFhevmDecryptClient: (params) => createFhevmDecryptClient(params),
});
