import { createFhevmBaseClient } from '@fhevm/sdk/ethers';
import { getEthersTestConfig } from '../setup-ethers.js';
import { isCleartext } from '../setupCommon.js';
import { defineClientBaseTests } from '../ethers-common/clientBase.tests.js';

defineClientBaseTests({
  runIf: !isCleartext(getEthersTestConfig().chainName),
  createFhevmBaseClient: (params) => createFhevmBaseClient(params),
  keyMode: 'fhe',
});
