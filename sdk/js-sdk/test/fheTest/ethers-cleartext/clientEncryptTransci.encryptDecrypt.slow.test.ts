import {
  createFhevmCleartextEncryptTransciClient,
  createFhevmCleartextDecryptClient,
} from '@fhevm/sdk/ethers/cleartext';
import { getEthersTestConfig } from '../setup-ethers.js';
import { isCleartext, protocolEraOf } from '../setupCommon.js';
import { defineClientEncryptTransciDecryptSlowTests } from '../ethers-common/clientEncryptTransci.encryptDecrypt.slow.tests.js';

////////////////////////////////////////////////////////////////////////////////
//
// CHAIN=localcleartext_v16 npx vitest run --config test/fheTest/vitest.config.ts ethers-cleartext/clientEncryptTransci.encryptDecrypt.slow.test.ts
//
////////////////////////////////////////////////////////////////////////////////

defineClientEncryptTransciDecryptSlowTests({
  // RFC-030 pad-based encryption needs the v0.16 cleartext executor (PaddedInputter).
  runIf: isCleartext(getEthersTestConfig().chainName) && protocolEraOf(getEthersTestConfig().chainName) >= 16,
  createFhevmEncryptTransciClient: (params) => createFhevmCleartextEncryptTransciClient(params),
  createFhevmDecryptClient: (params) => createFhevmCleartextDecryptClient(params),
});
