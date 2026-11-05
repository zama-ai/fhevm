import { createFhevmCleartextDecryptClient } from '@fhevm/sdk/viem/cleartext';
import { getViemTestConfig } from '../setup-viem.js';
import { isCleartext } from '../setupCommon.js';
import { defineClientDecryptDelegateDecryptTests } from '../viem-common/clientDecrypt.delegateDecrypt.tests.js';

////////////////////////////////////////////////////////////////////////////////
//
// CHAIN=localcleartext_v1_0 npx vitest run --config test/fheTest/vitest.config.ts viem-cleartext/clientDecrypt.delegateDecrypt.test.ts
//
////////////////////////////////////////////////////////////////////////////////

defineClientDecryptDelegateDecryptTests({
  runIf: isCleartext(getViemTestConfig().chainName),
  createFhevmDecryptClient: (params) => createFhevmCleartextDecryptClient(params),
});
