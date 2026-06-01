import { createFhevmCleartextBaseClient } from '@fhevm/sdk/ethers/cleartext';
import { getEthersTestConfig } from '../setup-ethers.js';
import { isCleartext } from '../setupCommon.js';
import { defineClientBaseTests } from '../ethers-common/clientBase.tests.js';

defineClientBaseTests({
  runIf: isCleartext(getEthersTestConfig().chainName),
  createFhevmBaseClient: (params) => createFhevmCleartextBaseClient(params),
  keyMode: 'cleartext',
});
