import {
  createFhevmCleartextEncryptWithPadClient,
  createFhevmCleartextDecryptClient,
} from '@fhevm/sdk/ethers/cleartext';
import { getEthersTestConfig } from '../setup-ethers.js';
import { isCleartext, protocolEraOf } from '../setupCommon.js';
import { defineClientEncryptWithPadDecryptSlowTests } from '../ethers-common/clientEncryptWithPad.encryptDecrypt.slow.tests.js';

////////////////////////////////////////////////////////////////////////////////
//
// CHAIN=localcleartext_v16 npx vitest run --config test/fheTest/vitest.config.ts ethers-cleartext/clientEncryptWithPad.encryptDecrypt.slow.test.ts
//
////////////////////////////////////////////////////////////////////////////////

defineClientEncryptWithPadDecryptSlowTests({
  // RFC-030 pad-based encryption needs the v0.16 cleartext executor (PaddedInputter).
  runIf: isCleartext(getEthersTestConfig().chainName) && protocolEraOf(getEthersTestConfig().chainName) >= 16,
  createFhevmEncryptWithPadClient: (params) => createFhevmCleartextEncryptWithPadClient(params),
  createFhevmDecryptClient: (params) => createFhevmCleartextDecryptClient(params),
});
