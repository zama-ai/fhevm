import { createFhevmCleartextBaseClient } from '@fhevm/sdk/ethers/cleartext';
import { getEthersTestConfig } from '../setup-ethers.js';
import { isCleartext } from '../setupCommon.js';
import { defineClientBaseChainTests } from '../ethers-common/clientBase.chain.tests.js';

defineClientBaseChainTests(isCleartext(getEthersTestConfig().chainName), (params) =>
  createFhevmCleartextBaseClient(params),
);
