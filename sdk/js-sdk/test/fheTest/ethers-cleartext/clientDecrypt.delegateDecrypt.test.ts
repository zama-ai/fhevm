import { createFhevmCleartextDecryptClient } from '@fhevm/sdk/ethers/cleartext';
import { getEthersTestConfig } from '../setup-ethers.js';
import { isCleartext } from '../setupCommon.js';
import { defineClientDecryptDelegateDecryptTests } from '../ethers-common/clientDecrypt.delegateDecrypt.tests.js';

////////////////////////////////////////////////////////////////////////////////
//
// CHAIN=localcleartext_v1_0 npx vitest run --config test/fheTest/vitest.config.ts ethers-cleartext/clientDecrypt.delegateDecrypt.test.ts
//
////////////////////////////////////////////////////////////////////////////////

defineClientDecryptDelegateDecryptTests({
  runIf: isCleartext(getEthersTestConfig().chainName),
  createFhevmDecryptClient: (params) => createFhevmCleartextDecryptClient(params),
});
