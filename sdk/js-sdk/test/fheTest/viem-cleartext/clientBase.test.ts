import { createFhevmCleartextBaseClient } from '@fhevm/sdk/viem/cleartext';
import { getViemTestConfig } from '../setup-viem.js';
import { isCleartext } from '../setupCommon.js';
import { defineClientBaseTests } from '../viem-common/clientBase.tests.js';

defineClientBaseTests(isCleartext(getViemTestConfig().chainName), {
  createClient: (params) => createFhevmCleartextBaseClient(params),
  keyMode: 'cleartext',
});
