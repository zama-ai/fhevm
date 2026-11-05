import { createFhevmCleartextEncryptWithPadClient, createFhevmCleartextDecryptClient } from '@fhevm/sdk/viem/cleartext';
import { getViemTestConfig } from '../setup-viem.js';
import { isCleartext, protocolEraOf } from '../setupCommon.js';
import { defineClientEncryptWithPadDecryptSlowTests } from '../viem-common/clientEncryptWithPad.encryptDecrypt.slow.tests.js';

////////////////////////////////////////////////////////////////////////////////
//
// CHAIN=localcleartext_v16 npx vitest run --config test/fheTest/vitest.config.ts viem-cleartext/clientEncryptWithPad.encryptDecrypt.slow.test.ts
//
////////////////////////////////////////////////////////////////////////////////

defineClientEncryptWithPadDecryptSlowTests({
  // RFC-030 pad-based encryption needs the v0.16 cleartext executor (PaddedInputter).
  runIf: isCleartext(getViemTestConfig().chainName) && protocolEraOf(getViemTestConfig().chainName) >= 16,
  createFhevmEncryptWithPadClient: (params) => createFhevmCleartextEncryptWithPadClient(params),
  createFhevmDecryptClient: (params) => createFhevmCleartextDecryptClient(params),
});
