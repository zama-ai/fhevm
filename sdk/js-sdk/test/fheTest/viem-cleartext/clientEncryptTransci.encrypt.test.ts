import { createFhevmCleartextEncryptTransciClient } from '@fhevm/sdk/viem/cleartext';
import { getViemTestConfig } from '../setup-viem.js';
import { isCleartext, protocolEraOf } from '../setupCommon.js';
import { defineClientEncryptTransciEncryptTests } from '../viem-common/clientEncryptTransci.encrypt.tests.js';

////////////////////////////////////////////////////////////////////////////////
//
// CHAIN=localcleartext_v16 npx vitest run --config test/fheTest/vitest.config.ts viem-cleartext/clientEncryptTransci.encrypt.test.ts
//
////////////////////////////////////////////////////////////////////////////////

defineClientEncryptTransciEncryptTests({
  // RFC-030 pad-based encryption needs the v0.16 cleartext executor (PaddedInputter).
  runIf: isCleartext(getViemTestConfig().chainName) && protocolEraOf(getViemTestConfig().chainName) >= 16,
  createFhevmEncryptTransciClient: (params) => createFhevmCleartextEncryptTransciClient(params),
});
