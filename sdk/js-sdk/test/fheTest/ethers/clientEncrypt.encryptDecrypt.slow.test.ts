import { createFhevmEncryptClient, createFhevmDecryptClient } from '@fhevm/sdk/ethers';
import { getEthersTestConfig } from '../setup-ethers.js';
import { isCleartext } from '../setupCommon.js';
import { defineClientEncryptDecryptSlowTests } from '../ethers-common/clientEncrypt.encryptDecrypt.slow.tests.js';

defineClientEncryptDecryptSlowTests(!isCleartext(getEthersTestConfig().chainName), {
  createEncryptClient: (params) => createFhevmEncryptClient(params),
  createDecryptClient: (params) => createFhevmDecryptClient(params),
});
