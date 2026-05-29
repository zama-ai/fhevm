import { createFhevmCleartextBaseClient } from '@fhevm/sdk/ethers/cleartext';
import { getEthersTestConfig } from '../setup-ethers.js';
import { isCleartext } from '../setupCommon.js';
import { defineClientBaseTests } from '../ethers-common/clientBase.tests.js';

defineClientBaseTests(isCleartext(getEthersTestConfig().chainName), {
  createClient: (params) => createFhevmCleartextBaseClient(params),
  keyMode: 'cleartext',
});
