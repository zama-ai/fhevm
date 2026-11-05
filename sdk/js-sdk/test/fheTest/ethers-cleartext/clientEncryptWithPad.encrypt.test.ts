import { createFhevmCleartextEncryptWithPadClient } from '@fhevm/sdk/ethers/cleartext';
import { getEthersTestConfig } from '../setup-ethers.js';
import { isLocalCleartextChain } from '../setupCommon.js';
import { defineClientEncryptWithPadEncryptTests } from '../ethers-common/clientEncryptWithPad.encrypt.tests.js';

////////////////////////////////////////////////////////////////////////////////
//
// CHAIN=localcleartext_v1_0 npx vitest run --config test/fheTest/vitest.config.ts ethers-cleartext/clientEncryptWithPad.encrypt.test.ts
//
////////////////////////////////////////////////////////////////////////////////

defineClientEncryptWithPadEncryptTests({
  // RFC-030 pad-based encryption needs the v1.0 cleartext stack (PaddedInputter).
  runIf: isLocalCleartextChain(getEthersTestConfig().chainName),
  createFhevmEncryptWithPadClient: (params) => createFhevmCleartextEncryptWithPadClient(params),
});
