import { createFhevmCleartextBaseClient } from '@fhevm/sdk/viem/cleartext';
import { getViemTestConfig } from '../setup-viem.js';
import { isCleartext } from '../setupCommon.js';
import { defineClientBaseChainTests } from '../viem-common/clientBase.chain.tests.js';

defineClientBaseChainTests({
  runIf: isCleartext(getViemTestConfig().chainName),
  createFhevmBaseClient: (params) => createFhevmCleartextBaseClient(params),
});
