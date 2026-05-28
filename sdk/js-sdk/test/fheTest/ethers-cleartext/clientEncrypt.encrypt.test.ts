import { createFhevmCleartextEncryptClient } from '@fhevm/sdk/ethers/cleartext';
import { getEthersTestConfig } from '../setup-ethers.js';
import { isCleartext } from '../setupCommon.js';
import { defineClientEncryptEncryptTests } from '../ethers-common/clientEncrypt.encrypt.tests.js';

defineClientEncryptEncryptTests(isCleartext(getEthersTestConfig().chainName), {
  createClient: (params) => createFhevmCleartextEncryptClient(params),
});
