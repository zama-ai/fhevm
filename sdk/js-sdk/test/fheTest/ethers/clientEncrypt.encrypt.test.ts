import { createFhevmEncryptClient } from '@fhevm/sdk/ethers';
import { getEthersTestConfig } from '../setup-ethers.js';
import { isCleartext } from '../setupCommon.js';
import { defineClientEncryptEncryptTests } from '../ethers-common/clientEncrypt.encrypt.tests.js';

defineClientEncryptEncryptTests({
  runIf: !isCleartext(getEthersTestConfig().chainName),
  createFhevmEncryptClient: (params) => createFhevmEncryptClient(params),
});
