import { createFhevmCleartextDecryptClient } from '@fhevm/sdk/ethers/cleartext';
import { getEthersTestConfig } from '../setup-ethers.js';
import { isCleartext } from '../setupCommon.js';
import { defineClientDecryptDelegateDecryptTests } from '../ethers-common/clientDecrypt.delegateDecrypt.tests.js';

defineClientDecryptDelegateDecryptTests(isCleartext(getEthersTestConfig().chainName), (params) =>
  createFhevmCleartextDecryptClient(params),
);
