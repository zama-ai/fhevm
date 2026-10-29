import { createFhevmCleartextEncryptTransciClient, createFhevmCleartextDecryptClient } from '@fhevm/sdk/viem/cleartext';
import { getViemTestConfig } from '../setup-viem.js';
import { isCleartext, protocolEraOf } from '../setupCommon.js';
import { defineClientEncryptTransciDecryptSlowTests } from '../viem-common/clientEncryptTransci.encryptDecrypt.slow.tests.js';

////////////////////////////////////////////////////////////////////////////////
//
// CHAIN=localcleartext_v16 npx vitest run --config test/fheTest/vitest.config.ts viem-cleartext/clientEncryptTransci.encryptDecrypt.slow.test.ts
//
////////////////////////////////////////////////////////////////////////////////

defineClientEncryptTransciDecryptSlowTests({
  // RFC-030 pad-based encryption needs the v0.16 cleartext executor (PaddedInputter).
  runIf: isCleartext(getViemTestConfig().chainName) && protocolEraOf(getViemTestConfig().chainName) >= 16,
  createFhevmEncryptTransciClient: (params) => createFhevmCleartextEncryptTransciClient(params),
  createFhevmDecryptClient: (params) => createFhevmCleartextDecryptClient(params),
});
