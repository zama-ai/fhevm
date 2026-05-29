import { createFhevmCleartextBaseClient } from '@fhevm/sdk/ethers/cleartext';
import { getEthersTestConfig } from '../setup-ethers.js';
import { isCleartext } from '../setupCommon.js';
import { defineClientBaseDecryptPublicValueTests } from '../ethers-common/clientBase.decryptPublicValue.tests.js';

defineClientBaseDecryptPublicValueTests(isCleartext(getEthersTestConfig().chainName), (params) =>
  createFhevmCleartextBaseClient(params),
);
