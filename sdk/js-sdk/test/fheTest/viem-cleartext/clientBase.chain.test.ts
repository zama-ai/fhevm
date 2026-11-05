import { createFhevmCleartextBaseClient } from '@fhevm/sdk/viem/cleartext';
import { getViemTestConfig } from '../setup-viem.js';
import { isCleartext } from '../setupCommon.js';
import { defineClientBaseChainTests } from '../viem-common/clientBase.chain.tests.js';

////////////////////////////////////////////////////////////////////////////////
//
// CHAIN=localcleartext_v1_0 npx vitest run --config test/fheTest/vitest.config.ts viem-cleartext/clientBase.chain.test.ts
//
////////////////////////////////////////////////////////////////////////////////

defineClientBaseChainTests({
  runIf: isCleartext(getViemTestConfig().chainName),
  createFhevmBaseClient: (params) => createFhevmCleartextBaseClient(params),
});
