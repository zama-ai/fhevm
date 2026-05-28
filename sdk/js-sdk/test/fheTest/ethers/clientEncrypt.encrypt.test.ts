import { createFhevmEncryptClient } from '@fhevm/sdk/ethers';
import { getEthersTestConfig } from '../setup-ethers.js';
import { isCleartext } from '../setupCommon.js';
import { defineClientEncryptEncryptTests } from '../ethers-common/clientEncrypt.encrypt.tests.js';

defineClientEncryptEncryptTests(!isCleartext(getEthersTestConfig().chainName), {
  createClient: (params) => createFhevmEncryptClient(params),
  moduleVersions: { tfhe: '1.5.3' }, // hardcoded for testnet protocol v0.12
});
