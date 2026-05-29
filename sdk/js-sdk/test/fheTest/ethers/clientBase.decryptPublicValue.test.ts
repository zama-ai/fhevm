import { createFhevmBaseClient } from '@fhevm/sdk/ethers';
import { getEthersTestConfig } from '../setup-ethers.js';
import { isCleartext } from '../setupCommon.js';
import { defineClientBaseDecryptPublicValueTests } from '../ethers-common/clientBase.decryptPublicValue.tests.js';

defineClientBaseDecryptPublicValueTests(!isCleartext(getEthersTestConfig().chainName), (params) =>
  createFhevmBaseClient(params),
);
