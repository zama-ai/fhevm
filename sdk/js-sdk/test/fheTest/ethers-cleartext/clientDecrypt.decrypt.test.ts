import { createFhevmCleartextDecryptClient } from '@fhevm/sdk/ethers/cleartext';
import { getEthersTestConfig } from '../setup-ethers.js';
import { isCleartext } from '../setupCommon.js';
import { defineClientDecryptDecryptTests } from '../ethers-common/clientDecrypt.decrypt.tests.js';

defineClientDecryptDecryptTests(isCleartext(getEthersTestConfig().chainName), (params) =>
  createFhevmCleartextDecryptClient(params),
);
