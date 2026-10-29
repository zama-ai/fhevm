import { createFhevmCleartextEncryptTransciClient } from '@fhevm/sdk/ethers/cleartext';
import { getEthersTestConfig } from '../setup-ethers.js';
import { isCleartext, protocolEraOf } from '../setupCommon.js';
import { defineClientEncryptTransciEncryptTests } from '../ethers-common/clientEncryptTransci.encrypt.tests.js';

////////////////////////////////////////////////////////////////////////////////
//
// CHAIN=localcleartext_v16 npx vitest run --config test/fheTest/vitest.config.ts ethers-cleartext/clientEncryptTransci.encrypt.test.ts
//
////////////////////////////////////////////////////////////////////////////////

defineClientEncryptTransciEncryptTests({
  // RFC-030 pad-based encryption needs the v0.16 cleartext executor (PaddedInputter).
  runIf: isCleartext(getEthersTestConfig().chainName) && protocolEraOf(getEthersTestConfig().chainName) >= 16,
  createFhevmEncryptTransciClient: (params) => createFhevmCleartextEncryptTransciClient(params),
});
