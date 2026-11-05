import { createFhevmCleartextEncryptWithPadClient } from '@fhevm/sdk/ethers/cleartext';
import { getEthersTestConfig } from '../setup-ethers.js';
import { isCleartext, protocolEraOf } from '../setupCommon.js';
import { defineClientEncryptWithPadEncryptTests } from '../ethers-common/clientEncryptWithPad.encrypt.tests.js';

////////////////////////////////////////////////////////////////////////////////
//
// CHAIN=localcleartext_v16 npx vitest run --config test/fheTest/vitest.config.ts ethers-cleartext/clientEncryptWithPad.encrypt.test.ts
//
////////////////////////////////////////////////////////////////////////////////

defineClientEncryptWithPadEncryptTests({
  // RFC-030 pad-based encryption needs the v0.16 cleartext executor (PaddedInputter).
  runIf: isCleartext(getEthersTestConfig().chainName) && protocolEraOf(getEthersTestConfig().chainName) >= 16,
  createFhevmEncryptWithPadClient: (params) => createFhevmCleartextEncryptWithPadClient(params),
});
