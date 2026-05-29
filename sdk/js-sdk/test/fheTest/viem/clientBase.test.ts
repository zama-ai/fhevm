import { createFhevmBaseClient } from '@fhevm/sdk/viem';
import { getViemTestConfig } from '../setup-viem.js';
import { isCleartext } from '../setupCommon.js';
import { defineClientBaseTests } from '../viem-common/clientBase.tests.js';

defineClientBaseTests(!isCleartext(getViemTestConfig().chainName), {
  createClient: (params) => createFhevmBaseClient(params),
  keyMode: 'fhe',
});
